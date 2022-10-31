mod error;
mod fmt_logger;
mod structures;

use crate::structures::traits::ProcessSub;
pub use fmt_logger::init_loggers;

use std::env;
use std::path::Path;

use crate::command::New;
use crate::structures::command;
use clap::Parser;
pub use error::ManagerError;
use structures::command::{Action, Cli};

pub fn run() -> Result<(), ManagerError> {
    // init_loggers();

    let cli = Cli::parse();
    if cli.debug {
        env::set_current_dir(env::current_dir()?.parent().unwrap())?;
    }

    let mut cargo_path = if cli.loc.is_none() {
        env::current_dir()?.as_path().to_owned()
    } else {
        Path::new(&cli.loc.as_ref().unwrap()).to_path_buf()
    };
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
