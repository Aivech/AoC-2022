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

    let score = lines.map(|line| -> (i32, i32, i32, i32) {
         match line {
            Ok(str) => {
                let bounds= str.split(&['-',',']).map(|spl| match spl.parse::<i32>() {
                    Ok(i) => {i}
                    Err(why) => { panic!("Malformed input: {}", why) }
                }).collect::<Vec<i32>>();
                if bounds.len() < 4 {panic!("Malformed input: {}", str)};
                (bounds[0], bounds[1], bounds[2], bounds[3])
            },
            Err(why) => panic!("Malformed input: {}", why)
        }
    }).fold(0, |score, bounds| -> i32 {
        if (bounds.0 <= bounds.2 && bounds.1 >= bounds.3) || (bounds.2 <= bounds.0 && bounds.3 >= bounds.1) {
            score + 1
        } else { score }
    });

    println!("part 1: {}", score);
}