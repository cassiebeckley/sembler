use std::fmt;

#[derive(Debug)]
pub enum Directive<'a> {
    Asciz(&'a str)
}

impl<'a> fmt::Display for Directive<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Directive::Asciz(string) => write!(f, ".asciz \"{}\"", string)
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

impl fmt::Display for NullaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &NullaryOp::Psh => write!(f, "PSH"),
            &NullaryOp::Pusharg => write!(f, "PUSHARG"),
            &NullaryOp::Li => write!(f, "LI"),
            &NullaryOp::Lc => write!(f, "LC"),
            &NullaryOp::Si => write!(f, "SI"),
            &NullaryOp::Sc => write!(f, "SC"),
            &NullaryOp::Swap => write!(f, "SWAP"),
            &NullaryOp::Pop => write!(f, "POP"),
            &NullaryOp::Ret => write!(f, "RET"),
            &NullaryOp::Eq => write!(f, "EQ"),
            &NullaryOp::Ne => write!(f, "NE"),
            &NullaryOp::Lt => write!(f, "LT"),
            &NullaryOp::Gt => write!(f, "GT"),
            &NullaryOp::Le => write!(f, "LE"),
            &NullaryOp::Ge => write!(f, "GE"),
            &NullaryOp::Add => write!(f, "ADD"),
            &NullaryOp::Sub => write!(f, "SUB"),
            &NullaryOp::Mul => write!(f, "MUL"),
            &NullaryOp::Div => write!(f, "DIV"),
            &NullaryOp::Mod => write!(f, "MOD"),
            &NullaryOp::And => write!(f, "AND"),
            &NullaryOp::Or => write!(f, "OR"),
            &NullaryOp::Xor => write!(f, "XOR")
        }
    }
}

#[derive(Debug)]
pub enum UnaryOp {
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

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &UnaryOp::Imm => write!(f, "IMM"),

            &UnaryOp::Rel => write!(f, "REL"),
            &UnaryOp::Jmp => write!(f, "JMP"),
            &UnaryOp::Bz => write!(f, "BZ"),
            &UnaryOp::Bnz => write!(f, "BNZ"),
            &UnaryOp::Ent => write!(f, "ENT"),
            &UnaryOp::Adj => write!(f, "ADJ"),
            &UnaryOp::Jsr => write!(f, "JSR"),
            &UnaryOp::Int => write!(f, "INT")
        }
    }
}

#[derive(Debug)]
pub enum Opcode<'a> {
    Nullary(NullaryOp),
    Unary(UnaryOp, Word<'a>)
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

impl<'a> fmt::Display for Instruction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Directive(ref directive) => write!(f, "{}", directive),
            Instruction::Opcode(ref opcode) => write!(f, "{}", opcode)
        }
    }
}

// TODO: think about names
#[derive(Debug)]
pub struct Entry<'a> {
    pub label: Option<&'a str>,
    pub entry: Instruction<'a>
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
