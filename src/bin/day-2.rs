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
        .map(|line| -> (i32, i32) {
            match line {
                Ok(str) => {
                    if str.len() == 3 && str.is_ascii() {
                        let them = (str.as_bytes()[0] as i32) - ('A' as i32);
                        let me = (str.as_bytes()[2] as i32) - ('X' as i32);
                        if (0..=2).contains(&them) && (0..=2).contains(&me) {
                            (them, me)
                        } else {
                            panic!("Malformed input: \"{}\"", str)
                        }
                    } else {
                        panic!("Malformed input: \"{}\"", str)
                    }
                }
                Err(why) => panic!("Malformed input: \"{}\"", why),
            }
        })
        .fold((0, 0), |(score1, score2), (them, me)| -> (i32, i32) {
            let shape_score1 = me + 1; // 1 rock, 2 paper, 3 scissor
            let round_score2 = me * 3; // 0 loss, 3 draw, 6 win
                                       // i solved this puzzle by the power of MODULAR ARITHMETIC
                                       // me - them + 1 = result (mod 3); 0 loss 1 draw 2 win
            let round_score1 = (me - them + 1).rem_euclid(3) * 3;
            // i solved the second puzzle by the power of MODULAR ARITHMETIC
            // me - 1 + them = result (mod 3)
            let shape_score2 = (me - 1 + them).rem_euclid(3) + 1;
            (
                score1 + shape_score1 + round_score1,
                score2 + shape_score2 + round_score2,
            )
        });

    println!("Total score (part 1): {}", score1);
    println!("Total score (part 2): {}", score2);
}
