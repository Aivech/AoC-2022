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

    let (score1, score2, _, _) = lines.fold(
        (0, 0, u64::MAX, 0),
        |(score1, score2, badge_mask, group_count), line| -> (i32, i32, u64, i32) {
            match line {
                Ok(str) => {
                    if !str.is_ascii() {
                        panic!("Malformed input - string not ASCII: {}", str)
                    }
                    let compartment_size = str.len() / 2;
                    let compartments = str.split_at(compartment_size);
                    let mask1 = create_bitmask_from_str(compartments.0);
                    let mask2 = create_bitmask_from_str(compartments.1);

                    // part 1 solution: find matching letter and convert to score
                    let chr1 = mask1 & mask2;
                    let result1 = match lib::common::u64_log_2(chr1) {
                        Some(i) => i + 1,
                        None => panic!("Malformed input: no container match for {}", str),
                    };

                    // part 2 solution: mask 3 entries and find char
                    let group_count = (group_count + 1) % 3;
                    let mut badge_mask = badge_mask & (mask1 | mask2);
                    let result2 = if group_count == 0 {
                        match lib::common::u64_log_2(badge_mask) {
                            Some(i) => {
                                badge_mask = u64::MAX;
                                i + 1
                            }
                            None => panic!("Malformed input: no badge match for {}", str),
                        }
                    } else {
                        0
                    };
                    (score1 + result1, score2 + result2, badge_mask, group_count)
                }
                Err(why) => {
                    panic!("malformed input: {}", why)
                }
            }
        },
    );

    println!("part 1: {}", score1);
    println!("part 1: {}", score2);
}

fn create_bitmask_from_str(s: &str) -> u64 {
    s.chars().fold(0u64, |mask, c| -> u64 {
        let ascii = c as i32;
        let index = if ascii < 'a' as i32 {
            ascii - 'A' as i32 + 26
        } else {
            ascii - 'a' as i32
        };
        mask | 1u64 << index
    })
}
