use std::{
    env::{args, current_dir},
    time::Duration,
};

use ascii_forge::prelude::*;
use click_data::ClickData;
use config::Config;
use confirmation::Confirmation;
use crokey::Combiner;
use events::ExplorerEvent;
use explorer::Explorer;
use input::{Input, InputEvent};
use sh::handle_sh;

mod dir_items;
mod entry;

mod events;
mod explorer;

mod config;
mod style;

mod click_data;

mod sh;

mod confirmation;
mod input;

fn main() -> anyhow::Result<()> {
    let mut input = Input::new();
    let mut confirmation = Confirmation::new();

    let mut last_click = ClickData::default();

    // Create the command combiner, and try to enable kitty keyboard protocol
    let mut combiner = Combiner::default();
    combiner.enable_combining()?;

    // Set path to local if no path is passed into the arguments
    let path = args()
        .collect::<Vec<String>>()
        .get(1)
        .cloned()
        .unwrap_or(current_dir()?.into_os_string().into_string().unwrap());

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
        explorer.refresh()?;

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
                        if last_click.is_double(&click_data, &config)
                            && explorer.selection_valid(idx - 1)
                        {
                            if explorer.is_file() {
                                // If file double clicked, run configured command.

                                if let Some(event) = config.double_click.clone() {
                                    match event {
                                        ExplorerEvent::Quit => return Ok(()),
                                        ExplorerEvent::Sh { command, args } => handle_sh(
                                            &explorer,
                                            command,
                                            args,
                                            &mut log_string,
                                            None,
                                        ),
                                        ExplorerEvent::Input { event } => {
                                            input.set_event(*event);
                                        }
                                        ExplorerEvent::Confirmation { event } => {
                                            confirmation.set(*event);
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
                    if let Some(e) = input.event(*k) {
                        match e {
                            InputEvent::Cancel => {
                                input.set_active(false);
                                input.clear();
                            }
                            InputEvent::Accept => {
                                let text = input.text();
                                if let Some(event) = input.take_event() {
                                    match event {
                                        ExplorerEvent::Quit => return Ok(()),
                                        ExplorerEvent::Sh { command, args } => handle_sh(
                                            &explorer,
                                            command,
                                            args,
                                            &mut log_string,
                                            Some(text),
                                        ),
                                        ExplorerEvent::Input { event } => {
                                            input.set_event(*event);
                                            input.set_active(true);
                                        }
                                        ExplorerEvent::Confirmation { event } => {
                                            confirmation.set(*event);
                                        }

                                        _ => explorer.handle_event(event.clone())?,
                                    }
                                }
                                input.set_active(false);
                            }
                        }
                    }
                    if input.active() {
                        continue;
                    }

                    if confirmation.active() {
                        let event = confirmation.take().expect("Confirmation should be Some");
                        if confirmation.handle(*k) {
                            match event {
                                ExplorerEvent::Quit => return Ok(()),
                                ExplorerEvent::Sh { command, args } => {
                                    handle_sh(&explorer, command, args, &mut log_string, None)
                                }
                                ExplorerEvent::Input { event } => {
                                    input.set_event(*event);
                                    input.set_active(true);
                                }
                                ExplorerEvent::Confirmation { event } => {
                                    confirmation.set(*event);
                                }

                                _ => explorer.handle_event(event.clone())?,
                            }
                        }
                        continue;
                    }
                    // Find the keybind pressed, and run the binding that is pressed, if a configuration is written.
                    if let Some(key_combo) = combiner.transform(*k) {
                        if let Some(event) = config.bindings.get(&key_combo) {
                            match event.clone() {
                                ExplorerEvent::Quit => return Ok(()),
                                ExplorerEvent::Sh { command, args } => {
                                    handle_sh(&explorer, command, args, &mut log_string, None)
                                }
                                ExplorerEvent::Input { event } => {
                                    input.set_event(*event);
                                    input.set_active(true);
                                }
                                ExplorerEvent::Confirmation { event } => {
                                    confirmation.set(*event);
                                }

                                _ => explorer.handle_event(event.clone())?,
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        // Render window, border, and log-string to the screen.
        render!(window,
            vec2(0, 0) => [ explorer ],
        );
        if confirmation.active() {
            render!( window,
                vec2(0, window.size().y - 6) =>
                [
                    "Are you sure? ( ", "y".green(), " / ", "n".red(), " )?"
                ]
            );
        } else if input.active() {
            render!( window,
                vec2(0, window.size().y - 6) =>
                [
                    "INPUT ".red(), "─".repeat(window.size().x as usize - 6).red()
                ],
                vec2(0, window.size().y - 5) =>
                [
                    ">>> ".red(), input.get_text()
                ],
            );
        } else {
            render!( window,
                vec2(0, window.size().y - 6) =>
                [
                    "Log ", "─".repeat(window.size().x as usize - 4)
                ],
                vec2(0, window.size().y - 5) => [ log_string ]
            );
        }

        // Update the window over a long duration
        window.update(Duration::from_secs(10))?;
    }

    Ok(())
}
