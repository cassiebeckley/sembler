#[derive(Debug)]
pub enum Directive<'a> {
    Asciz(&'a str)
}

#[derive(Debug)]
pub enum Word<'a> {
    Literal(u32),
    Label(&'a str)
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

#[derive(Debug)]
pub enum Opcode<'a> {
    Nullary(NullaryOp),
    Unary(UnaryOp, Word<'a>)
}

#[derive(Debug)]
pub enum Instruction<'a> {
    Directive(Directive<'a>),
    Opcode(Opcode<'a>)
}

// TODO: think about names
#[derive(Debug)]
pub struct Entry<'a> {
    pub label: Option<&'a str>,
    pub entry: Instruction<'a>
}

#[derive(Debug)]
pub struct Program<'a> {
    pub bss: Vec<Entry<'a>>,
    pub raw: Vec<Entry<'a>>
}
