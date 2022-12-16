use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let path = Path::new(match args.get(1) {
        None => {
            panic!("No input file found.");
        }
        Some(arg1) => arg1,
    });
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(why) => {
            panic!("Couldn't open file: {}", why)
        }
    };
    let mut str = String::new();
    file.read_to_string(&mut str).expect("Invalid input file!");

    drop(file);

    if !str.is_ascii() {panic!("Bad input")}

    let (index1, _) = str.as_str().as_bytes().windows(4).enumerate().find(|(_, window)| -> bool {
        4 == window.iter().fold( 0, |fold, item|-> u32 {
            (1u32 << (item - b'a')) | fold
        }).count_ones()
    }).expect("No unique sequence found!");

    let (index2, _) = str.as_str().as_bytes().windows(14).enumerate().find(|(_, window)| -> bool {
        14 == window.iter().fold( 0, |fold, item|-> u32 {
            (1u32 << (item - b'a')) | fold
        }).count_ones()
    }).expect("No unique sequence found!");

    println!("{}", index1 + 4);
    println!("{}", index2 + 14);
}
