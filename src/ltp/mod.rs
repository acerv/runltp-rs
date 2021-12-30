pub mod install;
pub mod session;
pub mod suite;
pub mod test;
pub mod utils;

use std::env;

pub fn root_dir() -> String {
    let ltproot: String;

    match env::var("LTPROOT") {
        Ok(root) => ltproot = root,
        Err(_) => {
            let cwd = env::current_dir().unwrap();
            ltproot = String::from(cwd.to_string_lossy());
        }
    }

    ltproot
}

pub fn tmp_dir() -> String {
    let tmpdir: String;

    match env::var("TMPDIR") {
        Ok(tmp) => tmpdir = tmp,
        Err(_) => tmpdir = "/tmp".to_string(),
    }

    tmpdir
}

pub fn testcases_dir() -> String {
    let root_dir = root_dir();
    let tc_dir: String = format!("{}/{}", root_dir, "testcases/bin");

    tc_dir
}

pub fn basebin_dir() -> String {
    let root_dir = root_dir();
    let bin_dir: String = format!("{}/{}", root_dir, "bin");

    bin_dir
}

pub fn runtest_dir() -> String {
    let root_dir = root_dir();
    let runtest_dir: String = format!("{}/{}", root_dir, "runtest");

    runtest_dir
}

pub fn scenario_dir() -> String {
    let root_dir = root_dir();
    let scenario_dir: String = format!("{}/{}", root_dir, "scenario_groups");

    scenario_dir
}
