extern crate rustc_serialize;

#[macro_use]
extern crate nom;

pub mod ast;
pub mod parser;
pub mod assemble;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
