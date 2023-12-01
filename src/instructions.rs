use std::{
    io::{self, Write},
    process::id,
};

use crate::{
    modules::{Dataidx, Elemidx, Funcidx, Globalidx, Labelidx, Localidx, Tableidx, Typeidx},
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Memarg {
    pub align: u32,
    pub offset: u32,
}

impl Grammar for Memarg {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.align.write(w)?;
        self.offset.write(w)
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
    I32Load(Memarg),
    I64Load(Memarg),
    F32Load(Memarg),
    F64Load(Memarg),
    I32Load8S(Memarg),
    I32Load8U(Memarg),
    I32Load16S(Memarg),
    I32Load16U(Memarg),
    I64Load8S(Memarg),
    I64Load8U(Memarg),
    I64Load16S(Memarg),
    I64Load16U(Memarg),
    I64Load32S(Memarg),
    I64Load32U(Memarg),
    I32Store(Memarg),
    I64Store(Memarg),
    F32Store(Memarg),
    F64Store(Memarg),
    I32Store8(Memarg),
    I32Store16(Memarg),
    I64Store8(Memarg),
    I64Store16(Memarg),
    I64Store32(Memarg),
    MemorySize,
    MemoryGrow,
    MemoryInit(Dataidx),
    DataDrop(Dataidx),
    MemoryCopy,
    MemoryFill,
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

            // Memory
            Instr::I32Load(m) => {
                0x28u8.write(w)?;
                m.write(w)
            }
            Instr::I64Load(m) => {
                0x29u8.write(w)?;
                m.write(w)
            }
            Instr::F32Load(m) => {
                0x2au8.write(w)?;
                m.write(w)
            }
            Instr::F64Load(m) => {
                0x2bu8.write(w)?;
                m.write(w)
            }
            Instr::I32Load8S(m) => {
                0x2cu8.write(w)?;
                m.write(w)
            }
            Instr::I32Load8U(m) => {
                0x2du8.write(w)?;
                m.write(w)
            }
            Instr::I32Load16S(m) => {
                0x2eu8.write(w)?;
                m.write(w)
            }
            Instr::I32Load16U(m) => {
                0x2fu8.write(w)?;
                m.write(w)
            }
            Instr::I64Load8S(m) => {
                0x30u8.write(w)?;
                m.write(w)
            }
            Instr::I64Load8U(m) => {
                0x31u8.write(w)?;
                m.write(w)
            }
            Instr::I64Load16S(m) => {
                0x32u8.write(w)?;
                m.write(w)
            }
            Instr::I64Load16U(m) => {
                0x33u8.write(w)?;
                m.write(w)
            }
            Instr::I64Load32S(m) => {
                0x34u8.write(w)?;
                m.write(w)
            }
            Instr::I64Load32U(m) => {
                0x35u8.write(w)?;
                m.write(w)
            }
            Instr::I32Store(m) => {
                0x36u8.write(w)?;
                m.write(w)
            }
            Instr::I64Store(m) => {
                0x37u8.write(w)?;
                m.write(w)
            }
            Instr::F32Store(m) => {
                0x38u8.write(w)?;
                m.write(w)
            }
            Instr::F64Store(m) => {
                0x39u8.write(w)?;
                m.write(w)
            }
            Instr::I32Store8(m) => {
                0x3au8.write(w)?;
                m.write(w)
            }
            Instr::I32Store16(m) => {
                0x3bu8.write(w)?;
                m.write(w)
            }
            Instr::I64Store8(m) => {
                0x3cu8.write(w)?;
                m.write(w)
            }
            Instr::I64Store16(m) => {
                0x3du8.write(w)?;
                m.write(w)
            }
            Instr::I64Store32(m) => {
                0x3eu8.write(w)?;
                m.write(w)
            }
            Instr::MemorySize => w.write_all(&[0x3f, 0x00]),
            Instr::MemoryGrow => w.write_all(&[0x40, 0x00]),
            Instr::MemoryInit(idx) => {
                0xfcu8.write(w)?;
                8u32.write(w)?;
                idx.write(w)?;
                0x00.write(w)
            }
            Instr::DataDrop(idx) => {
                0xfcu8.write(w)?;
                9u32.write(w)?;
                idx.write(w)
            }
            Instr::MemoryCopy => {
                0xfcu8.write(w)?;
                10u32.write(w)?;
                w.write_all(&[0x00, 0x00])
            }
            Instr::MemoryFill => {
                0xfcu8.write(w)?;
                11u32.write(w)?;
                0x00u8.write(w)
            }
        }
    }
}
