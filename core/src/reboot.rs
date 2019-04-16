use failure::Error;
use std::process::Command;

pub fn reboot() -> Result<(), Error> {
    let _output = prepare_command().output()?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn prepare_command() -> Command {
    let command = Command::new("shutdown");
    command.arg("/r");
    command.arg("/f");
    command
}

#[cfg(not(target_os = "windows"))]
fn prepare_command() -> Command {
    Command::new("reboot")
}
