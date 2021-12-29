use std::fs::File;
use std::io::{BufRead, BufReader};

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
