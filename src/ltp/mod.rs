pub mod session;
pub mod suite;
pub mod test;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn root_dir() -> String {
    let cwd = env::current_dir().unwrap();
    let root_dir = String::from(cwd.to_string_lossy());

    root_dir
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

pub fn read_ltp_file(path: String) -> Vec<String> {
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path, why),
        Ok(file) => file,
    };

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    let mut items = Vec::new();
    for line in lines {
        if line.is_empty() || line.trim().starts_with("#") {
            continue;
        }

        items.push(line);
    }

    items
}
