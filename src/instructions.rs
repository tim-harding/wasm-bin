use crate::{
    modules::{Funcidx, Labelidx, Tableidx, Typeidx},
    types::Valtype,
    values::S33,
    Grammar, Vector,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Blocktype {
    Empty,
    ValueType(Valtype),
    TypeIndex(S33),
}

impl Grammar for Blocktype {
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        match self {
            Blocktype::Empty => 0x40.write(w),
            Blocktype::ValueType(vt) => vt.write(w),
            Blocktype::TypeIndex(ti) => ti.write(w),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Instr<'a> {
    Unreachable,
    Nop,
    Block(Blocktype, &'a [Instr<'a>]),
    Loop(Blocktype, &'a [Instr<'a>]),
    If(Blocktype, &'a [Instr<'a>]),
    IfElse(Blocktype, &'a [Instr<'a>], &'a [Instr<'a>]),
    Br(Labelidx),
    BrIf(Labelidx),
    BrTable(Vector<'a, Labelidx>, Labelidx),
    Return,
    Call(Funcidx),
    CallIndirect(Typeidx, Tableidx),
}
