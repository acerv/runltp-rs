use crate::ltp;

use chrono::Local;
use ltp::suite::Suite;
use std::fs;

pub struct Session {
    pub name: String,
    pub suites: Vec<Suite>,
}

impl Session {
    /// Create a new session.
    pub fn new(suites: Vec<Suite>) -> Self {
        let date = Local::now();
        let name = date.format("%Y_%m_%d-%Hh_%Mm_%Ss").to_string();

        Session {
            name: name,
            suites: suites,
        }
    }

    /// Create a session from a scenario_groups file.
    pub fn all() -> Self {
        let dir = ltp::runtest_dir();
        let paths = fs::read_dir(dir).unwrap();

        let mut suites = Vec::new();
        for item in paths {
            let path = item.unwrap().path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let suite = Suite::from(file_name);
            suites.push(suite);
        }

        Session::new(suites)
    }

    /// Create a session from a scenario_groups file.
    pub fn from_scenario(name: &str) -> Self {
        let path = format!("{}/{}", ltp::scenario_dir(), name);
        let lines = ltp::read_ltp_file(path);

        let mut suites = Vec::new();
        for line in lines {
            let suite = Suite::from(&line);
            suites.push(suite);
        }

        Session::new(suites)
    }

    /// Create a session from suite names.
    pub fn from(names: Vec<&str>) -> Self {
        let mut suites = Vec::new();
        for name in names {
            let suite = Suite::from(name);
            suites.push(suite);
        }

        Session::new(suites)
    }

    /// Run all suites inside the session.
    pub fn run(self) {
        for item in self.suites {
            item.run();
        }
    }
}
