mod cli;
mod error;
mod toml_struct;

use std::env;

use clap::Parser;
use cli::{Cli, ProcessSub};
use error::ManagerError;

pub fn run() -> Result<(), ManagerError> {
    let cli = Cli::parse();

    if cli.debug {
        env::set_current_dir(env::current_dir()?.parent().unwrap())?;
    }

    let mut cargo_path = env::current_dir()?.as_path().to_owned();
    cargo_path = cargo_path.join("Cargo.toml");

    match cli.action {
        cli::Action::Add(add) => add.process(&cargo_path),
        cli::Action::Delete(delete) => delete.process(&cargo_path),
        cli::Action::Rename(rename) => rename.process(&cargo_path),
        cli::Action::Fix(fix) => fix.process(&cargo_path),
    }?;

    Ok(())
}
