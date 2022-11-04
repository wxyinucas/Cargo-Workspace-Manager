use std::process::Command;

fn main() {
    println!("{:?}", std::env::current_dir());
    let res = Command::new("./src/bin/print").output().unwrap();
    println!("{}", res.status);
}