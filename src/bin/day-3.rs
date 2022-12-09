use std::env;

pub mod common {
    pub mod files;
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let lines = match args.get(1) {
        None => {
            panic!("No input file found.");
        }
        Some(arg1) => common::files::load_file_as_lines(arg1),
    };
}
