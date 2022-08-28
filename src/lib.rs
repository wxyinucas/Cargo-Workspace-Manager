mod error;
mod structures;

use std::env;

use clap::Parser;
use structures::command::{Cli, ProcessSub};
use error::ManagerError;
use crate::structures::command;

pub fn run() -> Result<(), ManagerError> {
    let cli = Cli::parse();

    if cli.debug {
        env::set_current_dir(env::current_dir()?.parent().unwrap())?;
    }

    let mut cargo_path = env::current_dir()?.as_path().to_owned();
    cargo_path = cargo_path.join("Cargo.toml");

    match cli.action {
        command::Action::Add(add) => add.process(&cargo_path),
        command::Action::Delete(delete) => delete.process(&cargo_path),
        command::Action::Rename(rename) => rename.process(&cargo_path),
        command::Action::Fix(fix) => fix.process(&cargo_path),
    }?;

    Ok(())
}
