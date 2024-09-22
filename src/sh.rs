use std::process::Command;

use crate::explorer::Explorer;

pub fn handle_sh(
    explorer: &Explorer,
    command: String,
    mut args: Vec<String>,
    log_string: &mut String,
) {
    args.iter_mut().for_each(|x| {
        *x = shellexpand::full_with_context_no_errors(
            x,
            || dirs::home_dir().map(|x| x.into_os_string().into_string().unwrap()),
            |val| match val {
                "FOCUSED" => Some(
                    explorer
                        .focused_path()
                        .into_os_string()
                        .into_string()
                        .unwrap(),
                ),
                _ => None,
            },
        )
        .to_string();
    });
    *log_string = format!("{:?}", Command::new(command).args(args).output());
}
