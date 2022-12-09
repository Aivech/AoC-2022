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

    let score = lines.map(|line| -> (i32, i32) {
        match line {
            Ok(str) => if str.len() == 3 && str.is_ascii() {
                let them = (str.as_bytes()[0] as i32) - ('A' as i32);
                let me = (str.as_bytes()[2] as i32) - ('X' as i32);
                if (0..=2).contains(&them) && (0..=2).contains(&me) {
                    (them, me)
                } else {panic!("Malformed input: \"{}\"", str)}
            } else {panic!("Malformed input: \"{}\"", str)}
            Err(why) => panic!("Malformed input: \"{}\"", why)
        }
    }).fold(0, |score, (them, me)| -> i32 {
        let shape_score = me+1;
        // i solved this puzzle by the power of MODULAR ARITHMETIC
        let round_score = ((me-them).rem_euclid(3)+1).rem_euclid(3)*3;
        score + shape_score + round_score
    });

    println!("Total score (part 1): {}", score);
}