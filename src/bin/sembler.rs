extern crate sembler;
extern crate rustc_serialize as serialize;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use sembler::parser;
use sembler::assemble;

use serialize::json::ToJson;

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

    // TODO: take this as arg
    let entry_point = "main";

    let ast = match parser::parse_svm(&source) {
        Ok(ast) => ast,
        Err(e) => panic!("{:?}", e)
    };
    let blob = assemble::assemble(&ast, entry_point);

    println!("{}", blob.to_json().pretty());
}
