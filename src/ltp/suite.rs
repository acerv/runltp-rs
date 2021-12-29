use crate::ltp;

use ltp::test::Test;

pub struct Suite {
    pub name: String,
    pub tests: Vec<Test>,
}

impl Suite {
    /// Create a new testing suite.
    pub const fn new(name: String, tests: Vec<Test>) -> Self {
        Suite {
            name: name,
            tests: tests,
        }
    }

    /// Create a testing suite from its name.
    pub fn from(name: &str) -> Self {
        let runtest_dir = ltp::runtest_dir();
        let path = format!("{}/{}", runtest_dir, name);
        let lines = ltp::utils::read_ltp_file(path);

        let mut tests = Vec::new();
        for line in lines {
            let test = Test::from_declaration(&line);
            tests.push(test);
        }

        Suite::new(name.to_string(), tests)
    }

    /// Run all tests inside the suite.
    pub fn run(self) {
        for mut test in self.tests {
            let ret = test.run();
            match ret {
                Ok(_) => test.print_report(),
                Err(msg) => println!("Error: {}", msg),
            };
        }
    }
}
