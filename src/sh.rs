use std::process::Command;

use crate::explorer::Explorer;

pub fn handle_sh(
    explorer: &Explorer,
    command: String,
    mut args: Vec<String>,
    log_string: &mut String,
) {
    args.iter_mut().for_each(|x| {
        if x.starts_with('$') {
            *x = match x.as_str() {
                "$focused" => explorer
                    .focused_path()
                    .into_os_string()
                    .into_string()
                    .unwrap(),
                _ => "ERR".to_string(),
            }
        }
    });
    *log_string = format!("{:?}", Command::new(command).args(args).output());
}
