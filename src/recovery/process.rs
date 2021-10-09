use std::process::{Command, ExitStatus};

use crate::ui_print;

pub fn exec_logged(command: &mut Command) -> std::io::Result<ExitStatus> {
    let output = command.output()?;
    ui_print!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    Ok(output.status)
}
