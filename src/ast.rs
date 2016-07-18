use std::fmt;

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<Result<u8, &str>>;
}

#[derive(Debug)]
pub enum Directive<'a> {
    Asciz(&'a str),
    Ascii(&'a str),
    Db(u8),
    Dw(u32)
}

impl<'a> ToBytes for Directive<'a> {
    fn to_bytes(&self) -> Vec<Result<u8, &str>> {
        match *self {
            Directive::Asciz(s) => {
                let mut bytes: Vec<Result<u8, &str>> =
                  s.as_bytes()
                   .iter()
                   .map(|b| Ok(b.clone()))
                   .collect();

                bytes.push(Ok(0));
                bytes
            },
            Directive::Ascii(s) => s.as_bytes()
                                    .iter()
                                    .map(|b| Ok(b.clone()))
                                    .collect(),
            Directive::Db(byte) => vec![Ok(byte)],
            Directive::Dw(word) => vec![
                Ok((word >> 24) as u8),
                Ok((word >> 16) as u8),
                Ok((word >> 8) as u8),
                Ok(word as u8)
            ],
        }
    }
}

impl<'a> fmt::Display for Directive<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Directive::Asciz(string) => write!(f, ".asciz \"{}\"", string),
            &Directive::Ascii(string) => write!(f, ".ascii \"{}\"", string),
            &Directive::Db(byte)      => write!(f, ".db \"0x{:x}\"", byte),
            &Directive::Dw(word)      => write!(f, ".dw \"0x{:x}\"", word)
        }
    }
}

#[derive(Debug)]
pub enum Word<'a> {
    Literal(u32),
    Label(&'a str)
}

impl<'a> fmt::Display for Word<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Word::Literal(arg) => write!(f, "0x{:x}", arg),
            &Word::Label(label) => write!(f, "{}", label),
        }
    }
}

#[derive(Debug)]
pub enum NullaryOp {
    Swap,
    Pop,
    Ret,
    Retp,
    Li,
    Lc,
    Si,
    Sc,
    Psh,
    Or,
    Xor,
    And,
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
    Pusharg
}

impl ToBytes for NullaryOp {
    fn to_bytes(&self) -> Vec<Result<u8, &str>> {
        let byte = match *self {
            NullaryOp::Swap    => 0x03,
            NullaryOp::Pop     => 0x04,
            NullaryOp::Ret     => 0x0c,
            NullaryOp::Retp    => 0x38,
            NullaryOp::Li      => 0x0d,
            NullaryOp::Lc      => 0x0e,
            NullaryOp::Si      => 0x0f,
            NullaryOp::Sc      => 0x10,
            NullaryOp::Psh     => 0x11,
            NullaryOp::Or      => 0x12,
            NullaryOp::Xor     => 0x13,
            NullaryOp::And     => 0x14,
            NullaryOp::Eq      => 0x15,
            NullaryOp::Ne      => 0x16,
            NullaryOp::Lt      => 0x17,
            NullaryOp::Gt      => 0x18,
            NullaryOp::Le      => 0x19,
            NullaryOp::Ge      => 0x1a,
            NullaryOp::Add     => 0x1d,
            NullaryOp::Sub     => 0x1e,
            NullaryOp::Mul     => 0x1f,
            NullaryOp::Div     => 0x20,
            NullaryOp::Mod     => 0x21,
            NullaryOp::Pusharg => 0x34,
        };

        vec![Ok(byte)]
    }
}

impl fmt::Display for NullaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NullaryOp::Swap    => write!(f, "SWAP"),
            NullaryOp::Pop     => write!(f, "POP"),
            NullaryOp::Ret     => write!(f, "RET"),
            NullaryOp::Retp    => write!(f, "RETP"),
            NullaryOp::Li      => write!(f, "LI"),
            NullaryOp::Lc      => write!(f, "LC"),
            NullaryOp::Si      => write!(f, "SI"),
            NullaryOp::Sc      => write!(f, "SC"),
            NullaryOp::Psh     => write!(f, "PSH"),
            NullaryOp::Or      => write!(f, "OR"),
            NullaryOp::Xor     => write!(f, "XOR"),
            NullaryOp::And     => write!(f, "AND"),
            NullaryOp::Eq      => write!(f, "EQ"),
            NullaryOp::Ne      => write!(f, "NE"),
            NullaryOp::Lt      => write!(f, "LT"),
            NullaryOp::Gt      => write!(f, "GT"),
            NullaryOp::Le      => write!(f, "LE"),
            NullaryOp::Ge      => write!(f, "GE"),
            NullaryOp::Add     => write!(f, "ADD"),
            NullaryOp::Sub     => write!(f, "SUB"),
            NullaryOp::Mul     => write!(f, "MUL"),
            NullaryOp::Div     => write!(f, "DIV"),
            NullaryOp::Mod     => write!(f, "MOD"),
            NullaryOp::Pusharg => write!(f, "PUSHARG")
        }
    }
}

#[derive(Debug)]
pub enum UnaryOp {
    Rel,
    Imm,
    Jmp,
    Jsr,
    Jsrp,
    Bz,
    Bnz,
    Ent,
    Adj,
    Int
}

impl UnaryOp {
    fn to_byte(&self) -> u8 {
        match *self {
            UnaryOp::Rel  => 0x02,
            UnaryOp::Imm  => 0x05,
            UnaryOp::Jmp  => 0x06,
            UnaryOp::Jsr  => 0x07,
            UnaryOp::Jsrp => 0x37,
            UnaryOp::Bz   => 0x08,
            UnaryOp::Bnz  => 0x09,
            UnaryOp::Ent  => 0x0a,
            UnaryOp::Adj  => 0x0b,
            UnaryOp::Int  => 0x22
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnaryOp::Rel  => write!(f, "REL"),
            UnaryOp::Imm  => write!(f, "IMM"),
            UnaryOp::Jmp  => write!(f, "JMP"),
            UnaryOp::Jsr  => write!(f, "JSR"),
            UnaryOp::Jsrp => write!(f, "JSRP"),
            UnaryOp::Bz   => write!(f, "BZ"),
            UnaryOp::Bnz  => write!(f, "BNZ"),
            UnaryOp::Ent  => write!(f, "ENT"),
            UnaryOp::Adj  => write!(f, "ADJ"),
            UnaryOp::Int  => write!(f, "INT")
        }
    }
}

#[derive(Debug)]
pub enum Opcode<'a> {
    Nullary(NullaryOp),
    Unary(UnaryOp, Word<'a>)
}

impl<'a> ToBytes for Opcode<'a> {
    fn to_bytes(&self) -> Vec<Result<u8, &str>> {
        match *self {
            Opcode::Nullary(ref n) => n.to_bytes(),
            Opcode::Unary(ref u, Word::Literal(word)) => vec![
                Ok(u.to_byte()),
                Ok((word >> 24) as u8),
                Ok((word >> 16) as u8),
                Ok((word >> 8) as u8),
                Ok(word as u8)
            ],
            Opcode::Unary(ref u, Word::Label(label)) => vec![Ok(u.to_byte()), Err(label)],
        }
    }
}

impl<'a> fmt::Display for Opcode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Opcode::Nullary(ref nop) => write!(f, "{}", nop),
            Opcode::Unary(ref uop, ref word) => write!(f, "{} {}", uop, word),
        }
    }
}

#[derive(Debug)]
pub enum Instruction<'a> {
    Directive(Directive<'a>),
    Opcode(Opcode<'a>)
}

impl<'a> ToBytes for Instruction<'a> {
    fn to_bytes(&self) -> Vec<Result<u8, &str>> {
        match *self {
            Instruction::Directive(ref d) => d.to_bytes(),
            Instruction::Opcode(ref o) => o.to_bytes(),
        }
    }
}

impl<'a> fmt::Display for Instruction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Directive(ref directive) => write!(f, "{}", directive),
            Instruction::Opcode(ref opcode) => write!(f, "{}", opcode)
        }
    }
}

#[derive(Debug)]
pub struct Entry<'a> {
    pub label: Option<&'a str>,
    pub entry: Instruction<'a>
}

impl<'a> ToBytes for Entry<'a> {
    fn to_bytes(&self) -> Vec<Result<u8, &str>> {
        return self.entry.to_bytes();
    }
}

impl<'a> fmt::Display for Entry<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(label) = self.label {
            try!(write!(f, "{}:\n", label));
        }

        write!(f, "    {}", self.entry)
    }
}

#[derive(Debug)]
pub struct Program<'a> {
    pub bss: Vec<Entry<'a>>,
    pub raw: Vec<Entry<'a>>
}

impl<'a> fmt::Display for Program<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "bss {{\n"));

        for entry in &self.bss {
            try!(write!(f, "{}\n", entry));
        }

        try!(write!(f, "}}\n\nraw {{\n"));

        for entry in &self.raw {
            try!(write!(f, "{}\n", entry));
        }

        write!(f, "}}")
    }
}
