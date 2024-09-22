use std::{env::args, time::Duration};

use ascii_forge::prelude::*;
use click_data::ClickData;
use config::Config;
use crokey::Combiner;
use events::ExplorerEvent;
use explorer::Explorer;
use sh::handle_sh;

mod dir_items;
mod entry;

mod events;
mod explorer;

mod config;
mod style;

mod click_data;

mod sh;

fn main() -> anyhow::Result<()> {
    let mut last_click = ClickData::default();

    // Create the command combiner, and try to enable kitty keyboard protocol
    let mut combiner = Combiner::default();
    combiner.enable_combining()?;

    // Set path to local if no path is passed into the arguments
    let path = args()
        .collect::<Vec<String>>()
        .get(1)
        .cloned()
        .unwrap_or(".".to_string());

    // Initialize the window and have the window handle panics automatically
    let mut window = Window::init()?;
    handle_panics();

    // Load configuration from file system.
    let config = Config::load()?;

    // Create explorer and load the default folder automatically
    let mut explorer = Explorer::new(&path, &config)?;

    // A string for a previous log.
    let mut log_string = String::new();

    // The main exploring loop
    loop {
        // Re-read the file system for new changes.
        explorer = explorer.refresh()?;

        // If control-c is pressed, quit the program. (reserved command)
        if event!(window, Event::Key(k) => *k == KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL))
        {
            break;
        }

        // Loop through collected events.
        let events = window.events();
        for event in events {
            match event {
                Event::Mouse(m) => {
                    if m.kind == MouseEventKind::Down(MouseButton::Left) {
                        let idx = m.row as usize;
                        let click_data = ClickData::new(idx);

                        // Ignore click if clicking on ./ path.
                        if idx == 0 {
                            continue;
                        }

                        // Set Selected File
                        explorer.set_selected(idx - 1);

                        // If We just double clilcked
                        if last_click.is_double(&click_data, &config) {
                            if explorer.is_file() {
                                // If file double clicked, run configured command.

                                if let Some(event) = config.double_click.clone() {
                                    match event {
                                        ExplorerEvent::Quit => return Ok(()),
                                        ExplorerEvent::Sh { command, args } => {
                                            handle_sh(&explorer, command, args, &mut log_string)
                                        }
                                        _ => explorer.handle_event(event)?,
                                    }
                                }
                            } else {
                                // If double clicked folder, expand/collapse it.
                                explorer.toggle()?;
                            }
                            // Reset click to impossible line.
                            last_click = ClickData::default()
                        } else {
                            // Set last click to current click data.
                            last_click = click_data;
                        }
                    }

                    // On Scroll, scroll the explorer.
                    if m.kind == MouseEventKind::ScrollDown {
                        explorer.scroll_down(1);
                    }
                    if m.kind == MouseEventKind::ScrollUp {
                        explorer.scroll_up(1);
                    }
                }
                Event::Key(k) => {
                    // Find the keybind pressed, and run the binding that is pressed, if a configuration is written.
                    if let Some(key_combo) = combiner.transform(*k) {
                        if let Some(event) = config.bindings.get(&key_combo) {
                            match event.clone() {
                                ExplorerEvent::Quit => return Ok(()),
                                ExplorerEvent::Sh { command, args } => {
                                    handle_sh(&explorer, command, args, &mut log_string)
                                }
                                _ => explorer.handle_event(event.clone())?,
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // Render window and log-string to the screen.
        render!(window, vec2(0, 0) => [ explorer ], vec2(0, window.size().y - 5) => [ log_string ]);

        // Update the window over a long duration
        window.update(Duration::from_secs(10))?;
    }

    Ok(())
}
