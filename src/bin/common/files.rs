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
