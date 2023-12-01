use std::io::{self, Write};

use crate::{
    modules::{Elemidx, Funcidx, Globalidx, Labelidx, Localidx, Tableidx, Typeidx},
    types::{Reftype, Valtype},
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
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match self {
            Blocktype::Empty => 0x40u8.write(w),
            Blocktype::ValueType(vt) => vt.write(w),
            Blocktype::TypeIndex(ti) => ti.write(w),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Instr {
    // Control
    Unreachable,
    Nop,
    Block(Blocktype, Box<[Instr]>),
    Loop(Blocktype, Box<[Instr]>),
    If(Blocktype, Box<[Instr]>),
    IfElse(Blocktype, Box<[Instr]>, Box<[Instr]>),
    Br(Labelidx),
    BrIf(Labelidx),
    BrTable(Vector<Labelidx>, Labelidx),
    Return,
    Call(Funcidx),
    CallIndirect(Typeidx, Tableidx),
    // Reference
    RefNull(Reftype),
    RefIsNull,
    RefFunc(Funcidx),
    // Parametric
    Drop,
    Select(Option<Vector<Valtype>>),
    // Variable
    LocalGet(Localidx),
    LocalSet(Localidx),
    LocalTee(Localidx),
    GlobalGet(Globalidx),
    GlobalSet(Globalidx),
    // Table
    TableGet(Tableidx),
    TableSet(Tableidx),
    TableInit(Elemidx, Tableidx),
    ElemDrop(Elemidx),
    TableCopy(Tableidx, Tableidx),
    TableGrow(Tableidx),
    TableSize(Tableidx),
    TableFill(Tableidx),
    // Memory
    // Numeric
    // Vector
}

impl Grammar for Instr {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match self {
            // Control
            Instr::Unreachable => 0x00u8.write(w),
            Instr::Nop => 0x01u8.write(w),
            Instr::Block(bt, instructions) => {
                0x02u8.write(w)?;
                bt.write(w)?;
                instructions.iter().map(|i| i.write(w)).collect()?;
                0x0bu8.write(w)
            }
            Instr::Loop(bt, instructions) => {
                0x03u8.write(w)?;
                bt.write(w)?;
                instructions.iter().map(|i| i.write(w)).collect()?;
                0x0bu8.write(w)
            }
            Instr::If(bt, instructions) => {
                0x04u8.write(w)?;
                bt.write(w)?;
                instructions.iter().map(|i| i.write(w)).collect()?;
                0x0bu8.write(w)
            }
            Instr::IfElse(bt, in1, in2) => {
                0x04u8.write(w)?;
                bt.write(w)?;
                in1.iter().map(|i| i.write(w)).collect()?;
                0x05u8.write(w)?;
                in2.iter().map(|i| i.write(w)).collect()?;
                0x0bu8.write(w)
            }
            Instr::Br(l) => {
                0x0cu8.write(w)?;
                l.write(w)
            }
            Instr::BrIf(l) => {
                0x0du8.write(w)?;
                l.write(w)
            }
            Instr::BrTable(l, default) => {
                0x0eu8.write(w)?;
                l.write(w)?;
                default.write(w)
            }
            Instr::Return => 0x0fu8.write(w),
            Instr::Call(f) => {
                0x10u8.write(w)?;
                f.write(w)
            }
            Instr::CallIndirect(ty, table) => {
                0x11u8.write(w)?;
                ty.write(w)?;
                table.write(w)
            }

            // Reference
            Instr::RefNull(t) => {
                0xd0u8.write(w);
                t.write(w)
            }
            Instr::RefIsNull => 0xd1u8.write(w),
            Instr::RefFunc(f) => {
                0xd2u8.write(w)?;
                f.write(w)
            }

            // Parametric
            Instr::Drop => 0x1au8.write(w),
            Instr::Select(ty) => {
                if let Some(ty) = ty {
                    0x1cu8.write(w)?;
                    ty.write(w)
                } else {
                    0x1bu8.write(w)
                }
            }

            // Variable
            Instr::LocalGet(x) => {
                0x20u8.write(w)?;
                x.write(w)
            }
            Instr::LocalSet(x) => {
                0x21u8.write(w)?;
                x.write(w)
            }
            Instr::LocalTee(x) => {
                0x22u8.write(w)?;
                x.write(w)
            }
            Instr::GlobalGet(x) => {
                0x23u8.write(w)?;
                x.write(w)
            }
            Instr::GlobalSet(x) => {
                0x24u8.write(w)?;
                x.write(w)
            }

            // Table
            Instr::TableGet(table) => {
                0x25u8.write(w)?;
                table.write(w)
            }
            Instr::TableSet(table) => {
                0x26u8.write(w)?;
                table.write(w)
            }
            Instr::TableInit(element, table) => {
                0xfcu8.write(w)?;
                12u32.write(w)?;
                element.write(w)?;
                table.write(w)
            }
            Instr::ElemDrop(element) => {
                0xfcu8.write(w)?;
                13u32.write(w)?;
                element.write(w)
            }
            Instr::TableCopy(dst, src) => {
                0xfcu8.write(w)?;
                14u32.write(w)?;
                dst.write(w)?;
                src.write(w)
            }
            Instr::TableGrow(table) => {
                0xfcu8.write(w)?;
                15u32.write(w)?;
                table.write(w)
            }
            Instr::TableSize(table) => {
                0xfcu8.write(w)?;
                16u32.write(w)?;
                table.write(w)
            }
            Instr::TableFill(table) => {
                0xfcu8.write(w)?;
                17u32.write(w)?;
                table.write(w)
            }
        }
    }
}
