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

    let mut ship: HashMap<(u32, u32), char> = HashMap::new();
    let mut ship_length: u32 = 0;
    let mut row_heights = Vec::new();

    // intake and prepare ship data
    {
        let ship_data = lines
            .map_while(|line| -> Option<String> {
                match line {
                    // expects ship data to be separated from move instructions by an empty line
                    Ok(str) => {
                        if str.is_empty() {
                            None
                        } else {
                            Some(str)
                        }
                    }
                    Err(why) => {
                        panic!("Malformed input: {}", why)
                    }
                }
            })
            .collect::<Vec<String>>();

        // ship should have at least 1 cargo line and 1 data line
        if ship_data.len() < 2 { panic!("Malformed input: Ship data missing or corrupt") }

        // ship length should be equal to the last number on the last line of the ship data
        // unwrap because we already verified the size of the ship data
        ship_length = match ship_data.last().unwrap().split_whitespace().last() {
            None => {
                panic!("Malformed input: Ship data corrupt")
            }
            Some(num) => num
                .parse::<u32>()
                .unwrap_or_else(|why| panic!("Malformed input: Ship data corrupt: {}", why)),
        };

        // fill the ship row heights with zeros
        row_heights.resize(ship_length as usize, 0);

        // last line is labels
        let cargo_depth = ship_data.len() - 1;
        for i in 0..cargo_depth {
            // boxes are labeled A-Z
            ship_data[i].chars().enumerate().filter(|(_, char)| char.is_alphabetic()).for_each(|(col, chr)| -> () {
                if col >= ship_length as usize { panic!("Malformed input: row {} of ship data exceeds ship length: {}", i, ship_data[i])}
                if row_heights[col] < (cargo_depth - i) as u32 {row_heights[col] = (cargo_depth - i) as u32 };
                   ship.insert(((cargo_depth -i) as u32, col as u32), chr);
            });
        }


    };
    ship.iter().for_each(|((x,y),v)| println!("({},{}): {}", x,y , v))
}
