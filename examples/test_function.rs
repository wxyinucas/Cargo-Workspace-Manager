use std::process::Command;
use std::time::Duration;
use tracing::{debug};
use manager::init_loggers;

fn main() {
    init_loggers();
    let test_dir: &'static str = "/Users/wangxiaoyu/Desktop/wxy_manager/test_dir";

    let res = Command::new("cargo")
        .arg("run")
        .args(["--", "--loc", &format!("{}", test_dir)])
        .args(["add", "tmp"])
        .output()
        .unwrap();

    debug!("{:?}", res);
    let res = Command::new("cargo")
        .arg("run")
        .args(["--", "--loc", &format!("{}", test_dir)])
        .args(["add", "tmp1"])
        .output()
        .unwrap();
    debug!("{:?}", res);
    let res = Command::new("cargo")
        .arg("run")
        .args(["--", "--loc", &format!("{}", test_dir)])
        .args(["add", "tmp2"])
        .output()
        .unwrap();

    debug!("{:?}", res);
}
