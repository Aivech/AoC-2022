extern crate core;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let path = match args.get(1) {
        None => {
            panic!("No input file found.");
        }
        Some(arg1) => Path::new(arg1)
    };

    let lines = match File::open(path) {
        Ok(file) => BufReader::new(file).lines(),
        Err(msg) => {panic!("Couldn't open \"{}\": {}", path.display(), msg)}
    };

    let res = lines.fold((0,0), |(accum, max), line| -> (i32, i32){
        match line {
            Ok(str) => if str.is_empty() {
                if accum > max { (0, accum) } else { (0, max) }
            } else { match str.parse::<i32>() {
                Ok(int) => (accum + int, max),
                Err(why) => panic!("Malformed input: {}", why)
            }}
            Err(why) => panic!("Malformed input: {}", why)
        }
    });

    println!("{}", if res.0 > res.1 {res.0} else {res.1})
}