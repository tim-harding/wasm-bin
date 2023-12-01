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
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,
    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,
    I32TruncSatF32S,
    I32TruncSatF32U,
    I32TruncSatF64S,
    I32TruncSatF64U,
    I64TruncSatF32S,
    I64TruncSatF32U,
    I64TruncSatF64S,
    I64TruncSatF64U,
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
            Instr::I32Const(n) => {
                0x41u8.write(w)?;
                n.write(w)
            }
            Instr::I64Const(n) => {
                0x42u8.write(w)?;
                n.write(w)
            }
            Instr::F32Const(n) => {
                0x43u8.write(w)?;
                n.write(w)
            }
            Instr::F64Const(n) => {
                0x44u8.write(w)?;
                n.write(w)
            }
            Instr::I32Eqz => 0x45u8.write(w),
            Instr::I32Eq => 0x46u8.write(w),
            Instr::I32Ne => 0x47u8.write(w),
            Instr::I32LtS => 0x48u8.write(w),
            Instr::I32LtU => 0x49u8.write(w),
            Instr::I32GtS => 0x4au8.write(w),
            Instr::I32GtU => 0x4bu8.write(w),
            Instr::I32LeS => 0x4cu8.write(w),
            Instr::I32LeU => 0x4du8.write(w),
            Instr::I32GeS => 0x4eu8.write(w),
            Instr::I32GeU => 0x4fu8.write(w),
            Instr::I64Eqz => 0x50u8.write(w),
            Instr::I64Eq => 0x51u8.write(w),
            Instr::I64Ne => 0x52u8.write(w),
            Instr::I64LtS => 0x53u8.write(w),
            Instr::I64LtU => 0x54u8.write(w),
            Instr::I64GtS => 0x55u8.write(w),
            Instr::I64GtU => 0x56u8.write(w),
            Instr::I64LeS => 0x57u8.write(w),
            Instr::I64LeU => 0x58u8.write(w),
            Instr::I64GeS => 0x59u8.write(w),
            Instr::I64GeU => 0x5au8.write(w),
            Instr::F32Eq => 0x5bu8.write(w),
            Instr::F32Ne => 0x5cu8.write(w),
            Instr::F32Lt => 0x5du8.write(w),
            Instr::F32Gt => 0x5eu8.write(w),
            Instr::F32Le => 0x5fu8.write(w),
            Instr::F32Ge => 0x60u8.write(w),
            Instr::F64Eq => 0x61u8.write(w),
            Instr::F64Ne => 0x62u8.write(w),
            Instr::F64Lt => 0x63u8.write(w),
            Instr::F64Gt => 0x64u8.write(w),
            Instr::F64Le => 0x65u8.write(w),
            Instr::F64Ge => 0x66u8.write(w),
            Instr::I32Clz => 0x67u8.write(w),
            Instr::I32Ctz => 0x68u8.write(w),
            Instr::I32Popcnt => 0x69u8.write(w),
            Instr::I32Add => 0x6au8.write(w),
            Instr::I32Sub => 0x6bu8.write(w),
            Instr::I32Mul => 0x6cu8.write(w),
            Instr::I32DivS => 0x6du8.write(w),
            Instr::I32DivU => 0x6eu8.write(w),
            Instr::I32RemS => 0x6fu8.write(w),
            Instr::I32RemU => 0x70u8.write(w),
            Instr::I32And => 0x71u8.write(w),
            Instr::I32Or => 0x72u8.write(w),
            Instr::I32Xor => 0x73u8.write(w),
            Instr::I32Shl => 0x74u8.write(w),
            Instr::I32ShrS => 0x75u8.write(w),
            Instr::I32ShrU => 0x76u8.write(w),
            Instr::I32Rotl => 0x77u8.write(w),
            Instr::I32Rotr => 0x78u8.write(w),
            Instr::I64Clz => 0x79u8.write(w),
            Instr::I64Ctz => 0x7au8.write(w),
            Instr::I64Popcnt => 0x7bu8.write(w),
            Instr::I64Add => 0x7cu8.write(w),
            Instr::I64Sub => 0x7du8.write(w),
            Instr::I64Mul => 0x7eu8.write(w),
            Instr::I64DivS => 0x7fu8.write(w),
            Instr::I64DivU => 0x80u8.write(w),
            Instr::I64RemS => 0x81u8.write(w),
            Instr::I64RemU => 0x82u8.write(w),
            Instr::I64And => 0x83u8.write(w),
            Instr::I64Or => 0x84u8.write(w),
            Instr::I64Xor => 0x85u8.write(w),
            Instr::I64Shl => 0x86u8.write(w),
            Instr::I64ShrS => 0x87u8.write(w),
            Instr::I64ShrU => 0x88u8.write(w),
            Instr::I64Rotl => 0x89u8.write(w),
            Instr::I64Rotr => 0x8au8.write(w),
            Instr::F32Abs => 0x8bu8.write(w),
            Instr::F32Neg => 0x8cu8.write(w),
            Instr::F32Ceil => 0x8du8.write(w),
            Instr::F32Floor => 0x8eu8.write(w),
            Instr::F32Trunc => 0x8fu8.write(w),
            Instr::F32Nearest => 0x90u8.write(w),
            Instr::F32Sqrt => 0x91u8.write(w),
            Instr::F32Add => 0x92u8.write(w),
            Instr::F32Sub => 0x93u8.write(w),
            Instr::F32Mul => 0x94u8.write(w),
            Instr::F32Div => 0x95u8.write(w),
            Instr::F32Min => 0x96u8.write(w),
            Instr::F32Max => 0x97u8.write(w),
            Instr::F32Copysign => 0x98u8.write(w),
            Instr::F64Abs => 0x99u8.write(w),
            Instr::F64Neg => 0x9au8.write(w),
            Instr::F64Ceil => 0x9bu8.write(w),
            Instr::F64Floor => 0x9cu8.write(w),
            Instr::F64Trunc => 0x9du8.write(w),
            Instr::F64Nearest => 0x9eu8.write(w),
            Instr::F64Sqrt => 0x9fu8.write(w),
            Instr::F64Add => 0xa0u8.write(w),
            Instr::F64Sub => 0xa1u8.write(w),
            Instr::F64Mul => 0xa2u8.write(w),
            Instr::F64Div => 0xa3u8.write(w),
            Instr::F64Min => 0xa4u8.write(w),
            Instr::F64Max => 0xa5u8.write(w),
            Instr::F64Copysign => 0xa6u8.write(w),
            Instr::I32WrapI64 => 0xa7u8.write(w),
            Instr::I32TruncF32S => 0xa8u8.write(w),
            Instr::I32TruncF32U => 0xa9u8.write(w),
            Instr::I32TruncF64S => 0xaau8.write(w),
            Instr::I32TruncF64U => 0xabu8.write(w),
            Instr::I64ExtendI32S => 0xacu8.write(w),
            Instr::I64ExtendI32U => 0xadu8.write(w),
            Instr::I64TruncF32S => 0xaeu8.write(w),
            Instr::I64TruncF32U => 0xafu8.write(w),
            Instr::I64TruncF64S => 0xb0u8.write(w),
            Instr::I64TruncF64U => 0xb1u8.write(w),
            Instr::F32ConvertI32S => 0xb2u8.write(w),
            Instr::F32ConvertI32U => 0xb3u8.write(w),
            Instr::F32ConvertI64S => 0xb4u8.write(w),
            Instr::F32ConvertI64U => 0xb5u8.write(w),
            Instr::F32DemoteF64 => 0xb6u8.write(w),
            Instr::F64ConvertI32S => 0xb7u8.write(w),
            Instr::F64ConvertI32U => 0xb8u8.write(w),
            Instr::F64ConvertI64S => 0xb9u8.write(w),
            Instr::F64ConvertI64U => 0xbau8.write(w),
            Instr::F64PromoteF32 => 0xbbu8.write(w),
            Instr::I32ReinterpretF32 => 0xbcu8.write(w),
            Instr::I64ReinterpretF64 => 0xbdu8.write(w),
            Instr::F32ReinterpretI32 => 0xbeu8.write(w),
            Instr::F64ReinterpretI64 => 0xbfu8.write(w),
            Instr::I32Extend8S => 0xc0u8.write(w),
            Instr::I32Extend16S => 0xc1u8.write(w),
            Instr::I64Extend8S => 0xc2u8.write(w),
            Instr::I64Extend16S => 0xc3u8.write(w),
            Instr::I64Extend32S => 0xc4u8.write(w),
            Instr::I32TruncSatF32S => {
                0xfcu8.write(w);
                0u32.write(w)
            }
            Instr::I32TruncSatF32U => {
                0xfcu8.write(w);
                1u32.write(w)
            }
            Instr::I32TruncSatF64S => {
                0xfcu8.write(w);
                2u32.write(w)
            }
            Instr::I32TruncSatF64U => {
                0xfcu8.write(w);
                3u32.write(w)
            }
            Instr::I64TruncSatF32S => {
                0xfcu8.write(w);
                4u32.write(w)
            }
            Instr::I64TruncSatF32U => {
                0xfcu8.write(w);
                5u32.write(w)
            }
            Instr::I64TruncSatF64S => {
                0xfcu8.write(w);
                6u32.write(w)
            }
            Instr::I64TruncSatF64U => {
                0xfcu8.write(w);
                7u32.write(w)
            }
        }
    }
}
