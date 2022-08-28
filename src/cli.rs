use crate::error::ManagerError;
use crate::toml_struct::*;
use clap::{Parser, Subcommand};
use std::collections::HashSet;
use std::{
    fs::File,
    io::{BufReader, Read, Write},
    path::Path,
    process::{Command, Output},
};
use toml;

#[derive(Parser, Debug)]
#[clap(author = "Rex Wang", version, about, long_about= None)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: Action,

    #[clap(short, long)]
    pub debug: bool,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    Add(Add),
    Fix(Fix),
    Rename(Rename),
    Delete(Delete),
}

#[derive(Debug, Parser)]
pub struct Add {
    name: String,
}

#[derive(Debug, Parser)]
pub struct Fix;

#[derive(Debug, Parser)]
pub struct Rename {
    old: String,
    new: String,
}

#[derive(Debug, Parser)]
pub struct Delete {
    name: String,
}
pub trait ProcessSub {
    fn process(self, file_path: &Path) -> Result<(), ManagerError>;
}

impl ProcessSub for Add {
    fn process(self, file_path: &Path) -> Result<(), ManagerError> {
        let name = &self.name;
        // process toml file
        let raw_string = read_toml(file_path)?;
        let mut config = toml::from_str::<Config>(&raw_string)?;
        config.workspace.members.push(self.name.clone());

        let content = toml::to_string(&config)?;
        overwrite_toml(file_path, content)?;

        // process file system
        let res = Command::new("cargo").arg("new").arg(name).output()?;
        print_output(res);

        let res = Command::new("rm")
            .arg(format!("{name}/src/main.rs"))
            .output()?;
        print_output(res);

        let res = Command::new("mkdir")
            .arg(format!("{name}/src/bin"))
            .output()?;
        print_output(res);

        let res = Command::new("touch")
            .arg(format!("{name}/src/bin/blank.rs"))
            .output()?;
        print_output(res);
        Ok(())
    }
}

impl ProcessSub for Delete {
    fn process(self, file_path: &Path) -> Result<(), ManagerError> {
        let name = &self.name;
        // process toml file
        let raw_string = read_toml(file_path)?;
        let mut config = toml::from_str::<Config>(&raw_string)?;

        let loc = config
            .workspace
            .members
            .iter()
            .position(|x| x == name)
            .ok_or(ManagerError::NotFoundError)?;
        config.workspace.members.remove(loc);

        let content = toml::to_string(&config)?;
        overwrite_toml(file_path, content)?;

        // process file system
        let res = Command::new("rm")
            .args(["-rf", &format!("{}", name)])
            .output()?;
        print_output(res);

        Ok(())
    }
}

impl ProcessSub for Rename {
    fn process(self, file_path: &Path) -> Result<(), ManagerError> {
        let old = &self.old;
        let new = &self.new;

        // process toml file
        let raw_string = read_toml(file_path)?;
        let mut config = toml::from_str::<Config>(&raw_string)?;

        let loc = config
            .workspace
            .members
            .iter()
            .position(|x| x == old)
            .ok_or(ManagerError::NotFoundError)?;
        config.workspace.members.remove(loc);
        config.workspace.members.push(new.to_owned());

        let content = toml::to_string(&config)?;
        overwrite_toml(file_path, content)?;

        // process file system
        let res = Command::new("mv")
            .args([&format!("{old}"), &format!("{}", new)])
            .output()?;
        print_output(res);

        Ok(())
    }
}

impl ProcessSub for Fix {
    fn process(self, file_path: &Path) -> Result<(), ManagerError> {
        let raw_string = read_toml(file_path)?;
        let mut config = toml::from_str::<Config>(&raw_string)?;

        let toml_names = config
            .workspace
            .members
            .into_iter()
            .collect::<HashSet<String>>();

        let dir_path = file_path.parent().ok_or(ManagerError::NotFoundError)?;
        let dir_names: HashSet<String> = std::fs::read_dir(dir_path)?
            .into_iter()
            .filter_map(|f| {
                let entry = f.unwrap();
                if entry.file_type().unwrap().is_dir() {
                    Some(entry.file_name().into_string().unwrap())
                } else {
                    None
                }
            })
            .filter(|f| f != "targer" && f != r".git")
            .collect();
        // NOTICE 注意这一句的逻辑，error如何处理的？

        println!("here {:?}", dir_names);
        let join_name: Vec<String> = toml_names
            .intersection(&dir_names)
            .into_iter()
            .map(|f| f.to_owned())
            .collect();

        config.workspace.members = join_name.clone();
        let content = toml::to_string(&config)?;
        overwrite_toml(file_path, content)?;

        // FICKING Wrong!!!!!!!
        // for project in join_name.iter() {
        //     let res = Command::new("rm")
        //         .args(["-rf", &format!("{project}")])
        //         .output()?;
        //     print_output(res);
        // }
        Ok(())
    }
}

fn read_toml(file_path: &Path) -> Result<String, ManagerError> {
    let file = File::options().read(true).open(file_path)?;
    let mut file = BufReader::new(file);

    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn overwrite_toml(file_path: &Path, content: String) -> Result<(), ManagerError> {
    let mut file = File::options().write(true).truncate(true).open(file_path)?;
    file.write_all(&content.as_bytes()[..])?;

    Ok(())
}

fn print_output(output: Output) {
    if output.status.success() {
        println!(
            "success: {} + {}",
            std::str::from_utf8(output.stdout.as_slice()).unwrap(),
            std::str::from_utf8(output.stderr.as_slice()).unwrap()
        );
    } else {
        eprintln!(
            "err: {}",
            std::str::from_utf8(output.stderr.as_slice()).unwrap()
        );
    }
}
