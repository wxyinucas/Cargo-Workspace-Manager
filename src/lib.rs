mod error;
mod structures;
use crate::structures::traits::ProcessSub;

use std::env;

use clap::Parser;
use structures::command::{Cli, Action};
use error::ManagerError;
use crate::command::New;
use crate::structures::command;

pub fn run() -> Result<(), ManagerError> {
    let cli = Cli::parse();

    if cli.debug {
        env::set_current_dir(env::current_dir()?.parent().unwrap())?;
    }

    let mut cargo_path = env::current_dir()?.as_path().to_owned();
    cargo_path = cargo_path.join("Cargo.toml");

    match cli.action {
        command::Action::Add(add) => {
            let new = New;
            new.process(&cargo_path)?;
            add.process(&cargo_path)
        }
        Action::Delete(delete) => delete.process(&cargo_path),
        Action::Rename(rename) => rename.process(&cargo_path),
        Action::Fix(fix) => fix.process(&cargo_path),
        Action::New(new) => new.process(&cargo_path),
    }?;

    Ok(())
}
