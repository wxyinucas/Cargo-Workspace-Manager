use std::fs::File;
use std::io::Write;

use clap::{Parser, Subcommand};

use errors::{ManagerError, Result};

mod errors;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.action {
        Action::Init(init) => init.execute(),
        _ => todo!(),
    }
}

// TODO 思考，parser的抽象应在什么时候用，怎么调用？
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

        TODO 重构，找一个说一个改一个。
        TODO 更新顺序，更新任务完成的表达放法。
        */
        if exists_file("Cargo.toml").expect("Already has a cargo.toml file.") {
            println!("ERROR: Cargo.toml already exists!");
            return Ok(());
        }
        println!("INFO: Trying to create `Cargo.toml` `README` & `gitignore`...");

        create_file_if_not_exists("Cargo.toml", "[workspace]")?;
        create_file_if_not_exists("README.md", "")?;
        create_file_if_not_exists(".gitignore", "/target\n/Cargo.lock")?;

        println!("INFO: Done!");
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
        todo!()
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
        todo!()
    }
}

fn exists_file(file_name: &str) -> Result<bool> {
    let dir_entries = std::fs::read_dir(std::env::current_dir()?)?;
    let res = dir_entries
        .into_iter()
        .find(|entry| entry.as_ref().unwrap().file_name() == file_name);

    Ok(res.is_some())
}

fn create_file_if_not_exists(file_name: &str, content: &str) -> Result<()> {
    // TODO: 应该用不同error来表达
    if exists_file(file_name).unwrap() {
        println!("INFO: File `{}` already exists", file_name);
        return Ok(());
    }

    let mut file = File::create(file_name)?;
    file.write(content.as_ref())?;
    Ok(())
}

fn _debug_show_files() {
    let dir = std::fs::read_dir(std::env::current_dir().unwrap()).unwrap();
    for entry in dir {
        let e = entry.unwrap();
        println!("{:?}", e);
    }
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
    use std::process::Command;

    use tempfile::tempdir_in;

    use crate::{_debug_show_files, exists_file};

    // TODO 如何更高效的阅读crate
    #[test]
    fn manager_should_work() {
        // 找到上层目录
        // TODO：整理一下凌乱的目录操作
        let pwd = env::current_dir().unwrap();
        let upper_dir = pwd.parent().unwrap().to_owned();
        // println!("Debug Current: {:?}\n Upper: {:?}", pwd, upper_dir);

        // 生成临时文件夹，并切换进去
        let tmp_dir = tempdir_in(upper_dir).unwrap();
        // let mut tmp_dir = upper_dir.clone(); // TODO 记录不用temp时的临时开发方式，可以包装到一个函数里
        // tmp_dir.push("test_dir");
        // println!("tmp dir: {:?}", tmp_dir);
        // std::fs::create_dir(&tmp_dir).unwrap();

        env::set_current_dir(&tmp_dir).unwrap();

        // 粘贴文件
        let old_file ="/Users/wxy/Library/Mobile Documents/com~apple~CloudDocs/rust_icloud/wxy_manager/target/debug/manager";
        let mut new_file = env::current_dir().unwrap();
        new_file.push("manager");
        std::fs::copy(old_file, new_file).expect("Copy failed");

        init_should_work();
    }

    fn init_should_work() {
        // TODO 开发流程别太长，test似乎也是，在没有足够确定性的时候缩短测试流程。
        // 使用命令

        Command::new("./manager")
            .arg("init")
            .output()
            .expect("manager cant operate.");

        assert_eq!(exists_file("Cargo.toml").unwrap(), true);
        assert_eq!(exists_file("README.md").unwrap(), true);
        assert_eq!(exists_file(".gitignore").unwrap(), true);
    }
}
