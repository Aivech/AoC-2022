extern crate core;

use std::collections::HashMap;
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

    // (stack, height)
    let mut ship: HashMap<(u16, u16), char> = HashMap::new();
    let ship_length: u16;
    let cargo_depth: u16;
    let mut col_heights = Vec::new();

    // intake and prepare ship data
    {
        let ship_data = lines
            .take_while(|line| -> bool {
                match line {
                    Ok(str) => !str.is_empty(),
                    Err(why) => panic!("Malformed input: {}", why),
                }
            })
            .flatten()
            .collect::<Vec<String>>();
        // ship should have at least 1 cargo line and 1 data line
        if ship_data.len() < 2 {
            panic!("Malformed input: Ship data missing or corrupt")
        }

        // ship length should be equal to the last number on the last line of the ship data
        // unwrap because we already verified the size of the ship data
        ship_length = match ship_data.last().unwrap().split_whitespace().last() {
            None => {
                panic!("Malformed input: Ship data corrupt")
            }
            Some(num) => num
                .parse::<u16>()
                .unwrap_or_else(|why| panic!("Malformed input: Ship data corrupt: {}", why)),
        };

        // fill the ship row heights with zeros
        col_heights.resize(ship_length as usize, 0);

        // last line is labels
        cargo_depth = (ship_data.len() - 1) as u16;
        // yes i know i'm doing a for loop with an index. i don't want to consume every line, just those lines.
        for i in 0..cargo_depth as usize {
            // boxes are labeled A-Z
            ship_data[i]
                .chars()
                .enumerate()
                .filter(|(_, char)| char.is_alphabetic())
                .for_each(|(col, chr)| {
                    let col = (col - 1) / 4;
                    if col >= ship_length as usize {
                        panic!(
                            "Malformed input: row {} of ship data exceeds ship length: \"{}\"",
                            i, ship_data[i]
                        )
                    }
                    if col_heights[col] < cargo_depth - i as u16 - 1 {
                        col_heights[col] = cargo_depth - i as u16;
                    };
                    ship.insert((col as u16, cargo_depth - i as u16 - 1), chr);
                });
        }
    }
    // you can't get the iterator back because the borrow checker ate it, so we'll just get it again
    let lines = match args.get(1) {
        None => {
            panic!("No input file found.");
        }
        Some(arg1) => lib::common::load_file_as_lines(arg1),
    };

    // new instruction parse impl
    // prepare arrays
    let mut target_indices_part1 = (0..ship_length)
        .map(|i| -> (u16, u16) { (i, 0) })
        .collect::<Vec<(u16, u16)>>();
    let mut target_indices_part2 = target_indices_part1.clone();

    lines
        // skip the ship lines
        .skip(cargo_depth as usize + 2)
        // turn lines into strings, catch easy errors
        .map(|line| -> String {
            match line {
                Ok(str) => {
                    if str.is_empty() || !str.is_ascii() {
                        panic!("Malformed input: {}", str)
                    };
                    str
                }
                Err(why) => {
                    panic!("Malformed input: {}", why)
                }
            }
        })
        // turn strings into (count, src, tgt) instruction tuples
        .map(|str| -> (u16, u16, u16) {
            let result = str
                .chars()
                .filter(|c| c.is_ascii_digit() || c.is_whitespace())
                .collect::<String>()
                .split_whitespace()
                .flat_map(|s| s.parse::<u16>())
                .collect::<Vec<u16>>();
            (result[0], result[1], result[2])
        })
        // collect into a vec so we can reverse the order
        .collect::<Vec<(u16, u16, u16)>>()
        .iter()
        .rev()
        // apply instructions in reverse order
        .for_each(|(count, source, target)| {
            // change 1-indexing to 0-indexing
            let source = source - 1;
            let target = target - 1;
            // part 1: FIFO
            // if X crates were moved from A to B, the top of B was previously at position X-1 from the top of A
            // for X = 4: A [0] [-1] [-2] [T] ... -> B [T] ...
            // there are probably more efficient ways to look up values but for nine values it doesn't really matter
            for head in target_indices_part1.iter_mut() {
                let col = head.0;
                let depth = head.1;
                if col == target {
                    if depth >= *count {
                        // crane doesn't move head
                        head.1 -= *count;
                    } else {
                        // crane moves head
                        head.0 = source;
                        head.1 = *count - depth - 1;
                    }
                } else if col == source {
                    head.1 += *count;
                }
            }

            // part 2: crane preserves order
            for head in target_indices_part2.iter_mut() {
                let col = head.0;
                let depth = head.1;
                if col == target {
                    if depth >= *count {
                        head.1 -= *count;
                    } else {
                        head.0 = source;
                    }
                } else if col == source {
                    head.1 += count;
                }
            }
        });

    // print results: new method
    println!(
        "{}",
        collect_results(&target_indices_part1, &ship, &col_heights)
    );
    println!(
        "{}",
        collect_results(&target_indices_part2, &ship, &col_heights)
    );
}

fn collect_results(
    indices: &[(u16, u16)],
    ship: &HashMap<(u16, u16), char>,
    col_heights: &[u16],
) -> String {
    indices
        .iter()
        .map(|&(col, depth)| -> char {
            *(ship
                .get(&(col, col_heights[col as usize] - 1 - depth))
                .expect("Parse failure"))
        })
        .collect::<String>()
}
