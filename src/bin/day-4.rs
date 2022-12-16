use std::env;

pub mod lib {
    pub mod common;
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let lines = match args.get(1) {
        None => {
            panic!("No input file found.");
        }
        Some(arg1) => lib::common::load_file_as_lines(arg1),
    };

    let (score1, score2) = lines
        .map(|line| -> (i32, i32, i32, i32) {
            match line {
                Ok(str) => {
                    let bounds = str
                        .split(&['-', ','])
                        .map(|spl| match spl.parse::<i32>() {
                            Ok(i) => i,
                            Err(why) => {
                                panic!("Malformed input: {}", why)
                            }
                        })
                        .collect::<Vec<i32>>();
                    if bounds.len() < 4 {
                        panic!("Malformed input: {}", str)
                    };
                    (bounds[0], bounds[1], bounds[2], bounds[3])
                }
                Err(why) => panic!("Malformed input: {}", why),
            }
        })
        .fold((0, 0), |(score1, score2), bounds| -> (i32, i32) {
            let range1 = bounds.0..=bounds.1;
            let range2 = bounds.2..=bounds.3;
            if (range1.contains(&bounds.2) && range1.contains(&bounds.3))
                || (range2.contains(&bounds.0) && range2.contains(&bounds.1))
            {
                return (score1 + 1, score2 + 1);
            };
            if range1.contains(&bounds.2) || range1.contains(&bounds.3) {
                return (score1, score2 + 1);
            };
            (score1, score2)
        });

    println!("part 1: {}", score1);
    println!("part 1: {}", score2);
}
