use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

pub fn load_file_as_lines(s: &String) -> Lines<BufReader<File>> {
    match File::open(Path::new(s)) {
        Ok(file) => BufReader::new(file).lines(),
        Err(why) => {
            panic!("Couldn't open file: {}", why)
        }
    }
}

pub fn u64_log_2(i: u64) -> Option<i32> {
    if i == 0 {
        None
    } else {
        Some(63 - i.leading_zeros() as i32)
    }
}
