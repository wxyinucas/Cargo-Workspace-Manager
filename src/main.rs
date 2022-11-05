use std::collections::HashSet;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use errors::{ManagerError, Result};

mod errors;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.action {
        Action::Init(init) => init.execute(),
        Action::Add(add) => add.execute(),
        Action::Remove(remove) => remove.execute(),
        // _ => todo!(),
    }
}

// TODO 思考，parser的抽象应在什么时候用，怎么调用？
// TODO implement & test 的顺序应该是什么样的？先设计implement，再设计并完成test，最后implement
// TODO 开发流程别太长，test似乎也是，在没有足够确定性的时候缩短测试流程。
// TODO 如何更高效的阅读crate
// TODO 注意：test也需要开发，也不是一下就能得到想要的结果的
#[derive(Parser, Debug)]
#[clap(author = "Rex Wang")]
struct Cli {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    Init(Init),
    Add(Add),
    Remove(Remove),
}

#[derive(Parser, Debug)]
struct Init;

#[derive(Parser, Debug)]
struct Add {
    name: String,
}

#[derive(Parser, Debug)]
struct Remove {
    name: String,
}

trait TakeAction {
    fn execute(self) -> Result<()>;
}

impl TakeAction for Init {
    fn execute(self) -> Result<()> {
        /*
        流程：
        - 查看是否有toml，如果有，报错。
        - 生成README，生成带workspace的Cargo.toml，生成.gitignore并git init
        输出：3个步骤的结果。
        */
        if exists_file("Cargo.toml")? {
            println!("ERROR: Cargo.toml already exists!");
            return Ok(());
        }
        println!("INFO: Trying to create `Cargo.toml` `README` & `gitignore`...");

        let cargo = create_file_if_not_exists("Cargo.toml", "[workspace]\nmembers = []");
        let readme = create_file_if_not_exists("README.md", "");
        let git = create_file_if_not_exists(".gitignore", "/target\n/Cargo.lock");

        let mut exists = String::new();
        for res in [cargo, readme, git].into_iter() {
            if res.is_err() {
                match res.unwrap_err() {
                    ManagerError::FileExistError(file_name) => {
                        exists.push_str(&*format!("{}, ", file_name))
                    }
                    _ => continue,
                }
            }
        }
        if exists.is_empty() {
            println!("INFO: Done!");
        } else {
            exists.truncate(exists.len() - 2);
            println!("INFO: Done, files: {} exist.", exists);
        }
        Ok(())
    }
}

impl TakeAction for Add {
    fn execute(self) -> Result<()> {
        /*
        流程：
        - 查看toml中是否存在，如有报错。
        - 执行cargo 命令，改写Cargo.toml
        输出：返回第二步的两个结果
        */
        let new_crate = self.name;
        if exists_dir(new_crate.as_ref()).unwrap() {
            return Err(ManagerError::FileExistError(
                "ERROR: Project directory already exists!".into(),
            ));
        }

        let mut config = _read_toml()?;
        if config.workspace.members.get(&new_crate).is_some() {
            return Err(ManagerError::FileExistError(
                "ERROR: Project already exists in Cargo.toml!".into(),
            ));
        }
        println!("INFO: Trying to add project {} to workspace.", new_crate);

        // TODO 展示 command 的输出
        let output = Command::new("cargo")
            .arg("new")
            .arg(&new_crate)
            .arg("--lib")
            .arg("--vcs=none")
            .output()?;

        println!("TRACE: {}", String::from_utf8(output.stdout)?);
        config.workspace.members.insert(new_crate.clone());
        _overwrite_toml(config)?;
        println!("INFO: Project {} added!", new_crate);

        Ok(())
    }
}

impl TakeAction for Remove {
    fn execute(self) -> Result<()> {
        /*
        流程：
        - 查看是否有cargo project，toml里是否存在；若都不存在，直接退出。
        - 执行cargo 命令，改写Cargo.toml
        输出：返回每一步的结果
        */
        let remove_crate = self.name;
        let mut removed_contents = String::new();

        if exists_file(remove_crate.as_ref()).unwrap() {
            let output = Command::new("rm").arg("-rf").arg(&remove_crate).output()?;

            println!("TRACE: {}", String::from_utf8(output.stdout)?);
            removed_contents.push_str("dir");
        }

        let mut config = _read_toml()?;
        if config.workspace.members.remove(&remove_crate) {
            if removed_contents.len() > 0 {
                removed_contents.push_str(" ,")
            }
            removed_contents.push_str("toml")
        }
        _overwrite_toml(config)?;

        println!("INFO: {} removed", removed_contents);

        Ok(())
    }
}

fn exists_file(file_name: &str) -> Result<bool> {
    let dir_entries = std::fs::read_dir(std::env::current_dir()?)?;
    let res = dir_entries
        .into_iter()
        .find(|entry| entry.as_ref().unwrap().file_name() == file_name);

    Ok(res.is_some())
}

fn exists_dir(dir_name: &str) -> Result<bool> {
    let dir_entries = std::fs::read_dir(std::env::current_dir()?)?;
    let res = dir_entries
        .into_iter()
        .filter(|entry| entry.as_ref().unwrap().metadata().unwrap().is_dir())
        .find(|entry| entry.as_ref().unwrap().file_name() == dir_name);

    Ok(res.is_some())
}

fn create_file_if_not_exists(file_name: &str, content: &str) -> Result<()> {
    if exists_file(file_name).unwrap() {
        return Err(ManagerError::FileExistError(file_name.to_string()));
    }

    let mut file = File::create(file_name)?;
    file.write(content.as_ref())?;
    Ok(())
}

// =================================================================================================
//
// toml tools
//
// =================================================================================================
#[derive(Deserialize, Serialize)]
struct CargoToml {
    workspace: Workspace,
}

#[derive(Deserialize, Serialize)]
struct Workspace {
    members: HashSet<String>,
}

fn _read_toml() -> Result<CargoToml> {
    let mut file_path = std::env::current_dir()?;
    file_path.push("Cargo.toml");

    let mut file = File::options().read(true).open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(toml::from_str(&content)?)
}

fn _overwrite_toml(config: CargoToml) -> Result<()> {
    let mut file_path = std::env::current_dir()?;
    file_path.push("Cargo.toml");

    let mut file = File::options().write(true).truncate(true).open(file_path)?;
    file.write_all(&toml::to_string(&config)?.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    /*
    生成 temporary directory
    生成 bash 命令，执行；
    使用 std::fs 确认结果；
    删除 tmp directory
    */

    use std::env;
    use std::path::PathBuf;
    use std::process::Command;

    use tempfile::tempdir_in;

    use crate::{_read_toml, exists_dir, exists_file};

    #[test]
    fn manager_should_work() {
        // 找到上层目录
        let pwd = env::current_dir().unwrap();
        let upper_dir = pwd.parent().unwrap().to_owned();

        // 生成临时文件夹，并切换进去
        let tmp_dir = tempdir_in(upper_dir).unwrap();
        env::set_current_dir(&tmp_dir).unwrap();

        // 粘贴文件
        let old_file = "/Users/wxy/Library/Mobile Documents/com~apple~CloudDocs/rust_icloud/wxy_manager/target/debug/manager";
        let mut new_file = env::current_dir().unwrap();
        new_file.push("manager");
        std::fs::copy(old_file, new_file).expect("Copy failed");

        init_should_work();
        add_should_work();
        remove_should_work();
    }

    fn init_should_work() {
        Command::new("./manager")
            .arg("init")
            .output()
            .expect("manager cant operate.");

        assert_eq!(exists_file("Cargo.toml").unwrap(), true);
        assert_eq!(exists_file("README.md").unwrap(), true);
        assert_eq!(exists_file(".gitignore").unwrap(), true);
    }

    fn add_should_work() {
        let new_crate = "white_cat";
        Command::new("./manager")
            .arg("add")
            .arg(new_crate)
            .output()
            .expect("manager cant operate.");

        assert_eq!(exists_dir(new_crate).unwrap(), true);
        assert_eq!(
            _read_toml()
                .unwrap()
                .workspace
                .members
                .get(new_crate)
                .is_some(),
            true
        );
    }

    fn remove_should_work() {
        let new_crate = "white_cat";
        Command::new("./manager")
            .arg("remove")
            .arg(new_crate)
            .output()
            .expect("manager cant operate.");

        assert_eq!(exists_dir(new_crate).unwrap(), false);
        assert_eq!(
            _read_toml()
                .unwrap()
                .workspace
                .members
                .get(new_crate)
                .is_some(),
            false
        );
    }

    // =================================================================================================
    //
    // develop tools
    //
    // =================================================================================================

    fn _debug_show_files() {
        let dir = std::fs::read_dir(std::env::current_dir().unwrap()).unwrap();
        for entry in dir {
            let e = entry.unwrap();
            println!("{:?}", e);
        }
    }

    fn _generate_tmp_dir(mut path: PathBuf) -> crate::Result<()> {
        // TODO 和temp dir整合
        path.push("test_dir");
        println!("tmp dir: {:?}", path);
        std::fs::create_dir(&path).unwrap();
        Ok(())
    }
}
