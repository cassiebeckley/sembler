use std::collections::HashMap;
use ast::*;

fn first_pass<'a>(section: &'a Vec<Entry>, symbols: &mut HashMap<&'a str, u32>) -> Vec<Result<u8, &'a str>> {
    let mut entities = vec![];
    let mut byte_count = 0;

    for entry in section {
        if let Some(label) = entry.label {
            symbols.insert(label, byte_count);
        }

        for entity in entry.to_bytes() {
            entities.push(entity);
            match entity {
                Ok(_) => byte_count += 1,
                Err(_) => byte_count += 4
            }
        }
    }

    entities
}

fn print_transitional(code: &Vec<Result<u8, &str>>) {
    println!("Printing... something");
    for entity in code {
        match *entity {
            Ok(byte) => {
                println!("{:x}", byte);
            },
            Err(label) => {
                println!("[{}]", label);
            }
        }
    }
}

pub fn assemble(ast: &Program) -> Vec<u8> {
    let mut symbols = HashMap::new();

    let bss = first_pass(&ast.bss, &mut symbols);
    let raw = first_pass(&ast.raw, &mut symbols);

    print_transitional(&bss);
    print_transitional(&raw);

    println!("\nSymbols:\n{:?}", symbols);

    vec![]
}
