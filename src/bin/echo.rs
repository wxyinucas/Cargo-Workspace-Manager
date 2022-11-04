use std::io;
use std::io::Write;
use std::process::{Command};

fn main() {
    // let output = Command::new("/bin/cat")
    //     .arg("README.md")
    //     .output()
    //     .expect("failed to execute process");
    let output = Command::new("echo")
        .arg("\"hello\"")
        .arg(">")
        .arg("hello.txt")
        .output()
        .expect("failed to execute process"); // > 只能是字符串，不能是重新定位符

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
