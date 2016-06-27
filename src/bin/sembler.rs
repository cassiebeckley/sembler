extern crate sembler;
extern crate clap;
extern crate rustc_serialize as serialize;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use sembler::{parser, assemble};

use clap::{Arg, App};

use serialize::json::ToJson;

fn main() {
    let matches = App::new("Sembler")
    .version("0.1.0")
    .author("David Beckley <beckl.ds@gmail.com>")
    .about("An assembler for the Stockfighter Virtual Machine")
    .arg(Arg::with_name("entry-point")
        .short("e")
        .long("entry-point")
        .value_name("SYMBOL")
        .help("Sets the entry point to SYMBOL")
        .takes_value(true))
    .arg(Arg::with_name("output")
        .short("o")
        .long("output")
        .value_name("FILE")
        .help("Write output to FILE")
        .takes_value(true))
    .arg(Arg::with_name("INPUT")
        .help("Input file")
        .required(true)
        .index(1))
    .get_matches();

    let file = matches.value_of("INPUT").unwrap();
    let path = Path::new(&file);

    let mut file = File::open(&path).unwrap();

    let source = {
        let mut bytes = vec!();
        file.read_to_end(&mut bytes).unwrap();
        bytes
    };

    let entry_point = matches.value_of("entry-point").unwrap_or("main");

    let ast = match parser::parse_svm(&source) {
        Ok(ast) => ast,
        Err(e) => panic!("{:?}", e)
    };
    let blob = assemble::assemble(&ast, entry_point);

    match matches.value_of("output") {
        Some(filename) => {
            let mut f = File::create(filename).unwrap();
            write!(f, "{}", blob.to_json().pretty()).unwrap();
        },
        None => println!("{}", blob.to_json().pretty())
    }
}
