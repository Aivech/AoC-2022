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

    // parse instructions
    lines
        .skip(cargo_depth as usize + 2)
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
        .for_each(|(move_count, source, target)| {
            let source = source - 1;
            let target = target - 1;
            for _ in 0..move_count {
                let c = ship
                    .remove(&(source, col_heights[source as usize] - 1))
                    .unwrap_or_else(|| {
                        panic!(
                            "invalid move: {} from {} to {}",
                            move_count,
                            source + 1,
                            target + 1
                        )
                    });
                ship.insert((target, col_heights[target as usize]), c);
                col_heights[source as usize] -= 1;
                col_heights[target as usize] += 1;
            }
        });


    // print results
    eprintln!("Final ship: -----");
    for i in 0..ship_length {
        print!("{}: ", i+1);
        for j in 0..col_heights[i as usize] {
            print!("{} ", ship.get(&(i,j)).unwrap());
        }
        println!();
    }
    eprintln!("--------------");
    for i in 0..ship_length {
        print!("{}", ship.get(&(i,col_heights[i as usize]-1)).unwrap());
    }
    println!();
}
