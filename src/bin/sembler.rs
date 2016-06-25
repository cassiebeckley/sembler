extern crate sembler;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use sembler::parser;

fn main() {
    if env::args().count() < 2 {
        panic!("Usage: sembler <source_file>");
    }

    let file = env::args().nth(1).unwrap();
    let path = Path::new(&file);

    let mut file = File::open(&path).unwrap();

    let source = {
        let mut bytes = vec!();
        file.read_to_end(&mut bytes).unwrap();
        bytes
    };

    match parser::parse_svm(&source) {
        Some(program) => println!("{}", program),
        None => println!("error")
    }
}
