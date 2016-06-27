use std::str;

use nom::*;
use ast::*;

named!(comment,
    chain!(
        char!(';')        ~
        cc: is_not!("\n") ~
        char!('\n')       ,

        ||{cc}
    )
);

named!(whitespace < Vec<()> >,
    many0!(
        alt!(
            value!((), multispace) |
            value!((), comment)
        )
    )
);

fn is_id_char(chr: u8) -> bool {
    is_alphabetic(chr) || chr == b'_'
}

named!(identifier<&[u8], &str>,
    map_res!(take_while1!(is_id_char), str::from_utf8)
);

named!(label<&[u8], &str>,
    chain!(
        val: identifier ~
        char!(':')      ,

        ||{val}
    )
);

named!(string<&[u8], &str>, delimited!(char!('"'), map_res!(is_not!("\""), str::from_utf8), char!('"')));

named!(directive<&[u8], Directive>,
    chain!(
        tag!(".asciz") ~
        whitespace     ~
        s: string      ,

        ||{Directive::Asciz(s)}
    )
);

named!(hex<&[u8], u32>, preceded!(tag!("0x"), hex_u32));

named!(dec_u32<&[u8], u32>, map!(digit, |digits: &[u8]| {
    str::from_utf8(digits).unwrap().parse::<u32>().unwrap()
}));

named!(literal<&[u8], Word>, alt!(map!(hex, Word::Literal) | map!(dec_u32, Word::Literal)));

named!(word<&[u8], Word>, alt!(literal | map!(identifier, Word::Label)));

named!(nullary_operation<&[u8], Opcode>,
    map!(
        alt!(
            value!(NullaryOp::Psh,     tag!("PSH"))     |
            value!(NullaryOp::Pusharg, tag!("PUSHARG")) |
            value!(NullaryOp::Li,      tag!("LI"))      |
            value!(NullaryOp::Lc,      tag!("LC"))      |
            value!(NullaryOp::Si,      tag!("SI"))      |
            value!(NullaryOp::Sc,      tag!("SC"))      |
            value!(NullaryOp::Swap,    tag!("SWAP"))    |
            value!(NullaryOp::Pop,     tag!("POP"))     |
            value!(NullaryOp::Ret,     tag!("RET"))     |
            value!(NullaryOp::Eq,      tag!("EQ"))      |
            value!(NullaryOp::Ne,      tag!("NE"))      |
            value!(NullaryOp::Lt,      tag!("LT"))      |
            value!(NullaryOp::Gt,      tag!("GT"))      |
            value!(NullaryOp::Le,      tag!("LE"))      |
            value!(NullaryOp::Ge,      tag!("GE"))      |
            value!(NullaryOp::Add,     tag!("ADD"))     |
            value!(NullaryOp::Sub,     tag!("SUB"))     |
            value!(NullaryOp::Mul,     tag!("MUL"))     |
            value!(NullaryOp::Div,     tag!("DIV"))     |
            value!(NullaryOp::Mod,     tag!("MOD"))     |
            value!(NullaryOp::And,     tag!("AND"))     |
            value!(NullaryOp::Or,      tag!("OR"))      |
            value!(NullaryOp::Xor,     tag!("XOR"))
        ),
        Opcode::Nullary
    )
);

named!(unary_opcode<&[u8], UnaryOp>,
    alt!(
        value!(UnaryOp::Imm, tag!("IMM")) |
        value!(UnaryOp::Rel, tag!("REL")) |
        value!(UnaryOp::Jmp, tag!("JMP")) |
        value!(UnaryOp::Bz,  tag!("BZ"))  |
        value!(UnaryOp::Bnz, tag!("BNZ")) |
        value!(UnaryOp::Ent, tag!("ENT")) |
        value!(UnaryOp::Adj, tag!("ADJ")) |
        value!(UnaryOp::Jsr, tag!("JSR")) |
        value!(UnaryOp::Int, tag!("INT"))
    )
);

named!(unary_operation<&[u8], Opcode>,
    chain!(
        uu: unary_opcode ~
        whitespace       ~
        ww: word         ,

        ||{Opcode::Unary(uu, ww)}
    )
);

named!(operation<&[u8], Opcode>,
    alt!(
        nullary_operation |
        unary_operation
    )
);

named!(instruction<&[u8], Instruction>,
    alt!(
        map!(directive, Instruction::Directive) |
        map!(operation, Instruction::Opcode)
    )
);

named!(entry<&[u8], Entry>,
    chain!(
        ll: label?      ~
        whitespace      ~
        ii: instruction ~
        whitespace      ,

        ||{Entry{label: ll, entry: ii}}
    )
);

named!(bss< &[u8], Vec<Entry> >,
    chain!(
        tag!("bss")             ~
        whitespace              ~
        char!('{')              ~
        whitespace              ~
        ii: many0!(entry)       ~
        whitespace              ~
        char!('}')              ,

        ||{ii}
    )
);

named!(raw< &[u8], Vec<Entry> >,
    chain!(
        tag!("raw")             ~
        whitespace              ~
        char!('{')              ~
        whitespace              ~
        ii: many0!(entry)       ~
        whitespace              ~
        char!('}')              ,

        ||{ii}
    )
);

named!(parser<&[u8], Program>,
    chain!(
        whitespace ~
        b: bss     ~
        whitespace ~
        r: raw     ~
        whitespace ~
        eof        ,

        ||{Program{bss: b, raw: r}}
    )
);

#[derive(Debug)]
pub struct Error<'a> {
    position: usize,
    context: Option<&'a str>
}

fn get_error(length: usize, err: Err<&[u8]>) -> Error {
    match err {
        Err::Position(_, p) => Error {
            position: length - p.len(),
            context: str::from_utf8(&p[0..20]).ok()
        },
        _ => Error {
            position: 0,
            context: None
        }
    }
}

pub fn parse_svm(source: &[u8]) -> Result<Program, Error> {
  match parser(source) {
    IResult::Done(_, program) => Ok(program),
    IResult::Error(e) => Err(get_error(source.len(), e)),
    _ => Err(Error {
        position: 0,
        context: None
    })
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
