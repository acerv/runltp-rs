use crate::ltp;

use std::env;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};
use std::result;

pub struct Test {
    pub name: String,
    pub cmd: String,
    pub args: Vec<String>,
    pub out: String,
    pub fail: u64,
    pub pass: u64,
    pub brok: u64,
    pub skip: u64,
    pub warn: u64,
}

impl Test {
    /// Create a new test from a test declaration inside the runtest file.
    pub fn from_declaration(line: &str) -> Self {
        let parts: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();
        let name = parts[0].to_string();
        let cmd = parts[1].to_string();
        let args = parts[2..].to_vec();

        Test {
            name: name,
            cmd: cmd,
            args: args,
            out: String::new(),
            fail: 0,
            pass: 0,
            brok: 0,
            skip: 0,
            warn: 0,
        }
    }

    fn process_stdout(&mut self, line: &str) {
        if line.contains("TPASS") {
            self.pass += 1;
        } else if line.contains("TFAIL") {
            self.fail += 1;
        } else if line.contains("TSKIP") {
            self.skip += 1;
        } else if line.contains("TWARN") {
            self.warn += 1;
        } else if line.contains("TBROK") {
            self.brok += 1;
        }

        self.out.push_str(&format!("{}\n", line));

        println!("{}", line);
    }

    // Print output report.
    pub fn print_report(self) {
        println!(
            "\nPassed: {}\nFailures: {}\nSkipped: {}\nWarnings: {}\nBroken {}",
            self.pass, self.fail, self.skip, self.warn, self.brok
        );
    }

    /// Run the test.
    pub fn run(&mut self) -> result::Result<(), Error> {
        let root_dir = ltp::root_dir();
        let tmp_dir = ltp::tmp_dir();
        let tc_dir = ltp::testcases_dir();
        let bin_dir = ltp::basebin_dir();
        let path;

        match env::var("PATH") {
            Ok(val) => path = format!("{}:{}:{}", val, tc_dir, bin_dir),
            Err(_e) => path = String::new(),
        };

        let stdout = Command::new(&self.name)
            .args(self.args.clone())
            .stdout(Stdio::piped())
            .env("LTPROOT", root_dir)
            .env("TMPDIR", tmp_dir)
            .env("PATH", path)
            .spawn()?
            .stdout
            .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

        let reader = BufReader::new(stdout);
        reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| self.process_stdout(&line));

        Ok(())
    }
}
