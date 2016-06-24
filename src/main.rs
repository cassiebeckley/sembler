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
enum Opcode<'a> {
    Imm(Word<'a>),
    Rel(Word<'a>),
    Psh,
    Pusharg,
    Li,
    Lc,
    Si,
    Sc,
    Swap,
    Pop,
    Jmp(Word<'a>),
    Bz(Word<'a>),
    Bnz(Word<'a>),
    Ent(Word<'a>),
    Adj(Word<'a>),
    Jsr(Word<'a>),
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
    Xor,

    Int(Word<'a>)
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

named!(opcode<&[u8], Opcode>,
    alt!(
        map!(preceded!(tag!("IMM"), preceded!(whitespace, word)), Opcode::Imm) |
        map!(preceded!(tag!("REL"), preceded!(whitespace, word)), Opcode::Rel) |
        value!(Opcode::Psh, tag!("PSH")) |
        value!(Opcode::Pusharg, tag!("PUSHARG")) |
        value!(Opcode::Li, tag!("LI")) |
        value!(Opcode::Lc, tag!("LC")) |
        value!(Opcode::Si, tag!("SI")) |
        value!(Opcode::Sc, tag!("SC")) |
        value!(Opcode::Swap, tag!("SWAP")) |
        value!(Opcode::Pop, tag!("POP")) |
        map!(preceded!(tag!("JMP"), preceded!(whitespace, word)), Opcode::Jmp) |
        map!(preceded!(tag!("BZ"), preceded!(whitespace, word)), Opcode::Bz) |
        map!(preceded!(tag!("BNZ"), preceded!(whitespace, word)), Opcode::Bnz) |
        map!(preceded!(tag!("ENT"), preceded!(whitespace, word)), Opcode::Ent) |
        map!(preceded!(tag!("ADJ"), preceded!(whitespace, word)), Opcode::Adj) |
        map!(preceded!(tag!("JSR"), preceded!(whitespace, word)), Opcode::Jsr) |
        value!(Opcode::Ret, tag!("RET")) |

        value!(Opcode::Eq, tag!("EQ")) |
        value!(Opcode::Ne, tag!("NE")) |
        value!(Opcode::Lt, tag!("LT")) |
        value!(Opcode::Gt, tag!("GT")) |
        value!(Opcode::Le, tag!("LE")) |
        value!(Opcode::Ge, tag!("GE")) |
        value!(Opcode::Add, tag!("ADD")) |
        value!(Opcode::Sub, tag!("SUB")) |
        value!(Opcode::Mul, tag!("MUL")) |
        value!(Opcode::Div, tag!("DIV")) |
        value!(Opcode::Mod, tag!("MOD")) |
        value!(Opcode::And, tag!("AND")) |
        value!(Opcode::Or, tag!("OR")) |
        value!(Opcode::Xor, tag!("XOR")) |

        map!(preceded!(tag!("INT"), preceded!(whitespace, word)), Opcode::Int)
    )
);

named!(instruction<&[u8], Instruction>,
    alt!(
        map!(directive, Instruction::Directive) |
        map!(opcode, Instruction::Opcode)
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
        panic!("Usage: stockembler <source_file>");
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
