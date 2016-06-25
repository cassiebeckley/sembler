use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;

#[macro_use]
extern crate nom;
use nom::*;

#[derive(Debug)]
enum Directive<'a> {
    Asciz(&'a str)
}

#[derive(Debug)]
enum Word<'a> {
    Literal(u32),
    Label(&'a str)
}

#[derive(Debug)]
enum NullaryOp {
    Psh,
    Pusharg,
    Li,
    Lc,
    Si,
    Sc,
    Swap,
    Pop,
    Ret,

    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor
}

#[derive(Debug)]
enum UnaryOp {
    Imm,
    Rel,
    Jmp,
    Bz,
    Bnz,
    Ent,
    Adj,
    Jsr,

    Int
}

#[derive(Debug)]
enum Opcode<'a> {
    Nullary(NullaryOp),
    Unary(UnaryOp, Word<'a>)
}

#[derive(Debug)]
enum Instruction<'a> {
    Directive(Directive<'a>),
    Opcode(Opcode<'a>)
}

// TODO: think about names
#[derive(Debug)]
struct Entry<'a> {
    label: Option<&'a str>,
    entry: Instruction<'a>
}

#[derive(Debug)]
struct Program<'a> {
    bss: Vec<Entry<'a>>,
    raw: Vec<Entry<'a>>
}

named!(whitespace <Vec<char> >,
    many0!(
        alt!(char!(' ') | char!('\n') | char!('\r'))
    )
);

named!(label<&[u8], &str>,
    chain!(
        val: map_res!(alpha, str::from_utf8) ~
        char!(':')                           ,

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

named!(word<&[u8], Word>, alt!(map!(hex, Word::Literal) | map!(map_res!(alpha, str::from_utf8), Word::Label)));

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
        b: bss     ~
        whitespace ~
        r: raw     ~
        whitespace ~
        eof        ,

        ||{Program{bss: b, raw: r}}
    )
);

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

    println!("parsed: {:?}", parser(&source));
}
