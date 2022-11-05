use std::io::Stdout;
use std::process::{Command, Stdio};

fn main() {
    let output = Command::new("echo").arg("hello").output().unwrap();

    println!("{:?}", String::from_utf8(output.stdout).unwrap());
}
