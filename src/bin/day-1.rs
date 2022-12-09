extern crate core;

pub mod lib {
    pub mod common;
}

use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let lines = match args.get(1) {
        None => {
            panic!("No input file found.");
        }
        Some(arg1) => lib::common::load_file_as_lines(arg1),
    };

    let res = lines.fold(
        (0, 0, 0, 0),
        |(accum, max1, max2, max3), line| -> (i32, i32, i32, i32) {
            match line {
                Ok(str) => {
                    if str.is_empty() {
                        if accum > max3 {
                            if accum > max2 {
                                if accum > max1 {
                                    return (0, accum, max1, max2);
                                }
                                return (0, max1, accum, max2);
                            }
                            (0, max1, max2, accum)
                        } else {
                            (0, max1, max2, max3)
                        }
                    } else {
                        match str.parse::<i32>() {
                            Ok(int) => (accum + int, max1, max2, max3),
                            Err(why) => panic!("Malformed input: {}", why),
                        }
                    }
                }
                Err(why) => panic!("Malformed input: {}", why),
            }
        },
    );
    let finish = if res.0 > res.3 {
        if res.0 > res.1 {
            (res.0, res.0 + res.1 + res.2)
        } else {
            (res.1, res.0 + res.1 + res.2)
        }
    } else {
        (res.1, res.1 + res.2 + res.3)
    };

    println!("{}", finish.0);
    println!("{}", finish.1);
}
