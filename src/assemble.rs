use std::collections::{HashMap, BTreeMap};
use ast::*;

use rustc_serialize::base64::{self, ToBase64};
use rustc_serialize::json::{ToJson, Json};

fn first_pass<'a>(section: &'a Vec<Entry>, symbols: &mut HashMap<&'a str, u32>) -> Result<Vec<Result<u8, &'a str>>, String> {
    let mut entities = vec![];
    let mut byte_count = 0;

    for entry in section {
        if let Some(label) = entry.label {
            if let Some(_) = symbols.insert(label, byte_count) {
                return Err(format!("Symbol \"{}\" defined multiple times", label));
            }
        }

        for entity in entry.to_bytes() {
            entities.push(entity);
            match entity {
                Ok(_) => byte_count += 1,
                Err(_) => byte_count += 4
            }
        }
    }

    Ok(entities)
}

fn second_pass(entities: &Vec<Result<u8, &str>>, symbols: &HashMap<&str, u32>) -> Result<Vec<u8>, String> {
    let mut code = vec![];

    for entity in entities {
        match *entity {
            Ok(b) => code.push(b),
            Err(label) => {
                let address = try!(symbols.get(label).ok_or(format!("Undefined symbol \"{}\"", label)));
                code.push((address >> 24) as u8);
                code.push((address >> 16) as u8);
                code.push((address >> 8) as u8);
                code.push(address.clone() as u8);
            }
        }
    }

    Ok(code)
}

// don't perturb this
pub struct Blob {
    pub bss: Vec<u8>,
    pub raw: Vec<u8>,
    pub ep: u32
}

impl ToJson for Blob {
    fn to_json(&self) -> Json {
        let mut blob = BTreeMap::new();

        blob.insert("ok".to_string(), Json::Boolean(true));

        blob.insert("bss".to_string(), Json::String(self.bss.to_base64(base64::STANDARD)));
        blob.insert("raw".to_string(), Json::String(self.raw.to_base64(base64::STANDARD)));
        blob.insert("ep".to_string(), Json::U64(self.ep as u64));

        Json::Object(blob)
    }
}

pub fn assemble(ast: &Program, entry_point: &str) -> Blob {
    let mut symbols = HashMap::new();

    let bss = first_pass(&ast.bss, &mut symbols).unwrap();
    let raw = first_pass(&ast.raw, &mut symbols).unwrap();

    let bss = second_pass(&bss, &symbols).unwrap();
    let raw = second_pass(&raw, &symbols).unwrap();

    let ep = symbols.get(entry_point).expect(&format!("Could not find entry point \"{}\"", entry_point));

    Blob {
        bss: bss,
        raw: raw,
        ep: ep.clone()
    }
}
