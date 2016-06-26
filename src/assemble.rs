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

fn second_pass(entities: &Vec<Result<u8, &str>>, symbols: &HashMap<&str, u32>) -> Result<Vec<u8>, String> {
    let mut code = vec![];

    for entity in entities {
        match *entity {
            Ok(b) => code.push(b),
            Err(label) => {
                let address = try!(symbols.get(label).ok_or("Undefined symbol ".to_string() + label));
                code.push((address >> 24) as u8);
                code.push((address >> 16) as u8);
                code.push((address >> 8) as u8);
                code.push(address.clone() as u8);
            }
        }
    }

    Ok(code)
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

fn print_code(code: &Vec<u8>) {
    println!("Printing... something");
    for byte in code {
        println!("{:x}", byte);
    }
}

pub fn assemble(ast: &Program) -> Vec<u8> {
    let mut symbols = HashMap::new();

    let bss = first_pass(&ast.bss, &mut symbols);
    let raw = first_pass(&ast.raw, &mut symbols);

    // print_transitional(&bss);
    // print_transitional(&raw);

    let bss = second_pass(&bss, &symbols).unwrap();
    let raw = second_pass(&raw, &symbols).unwrap();

    print_code(&bss);
    print_code(&raw);

    println!("\nSymbols:\n{:?}", symbols);

    vec![]
}
