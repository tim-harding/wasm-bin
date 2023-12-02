use std::io::{self, Write};

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Laneidx(pub u8);

impl Grammar for Laneidx {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.0.write(w)
    }
}

pub struct Expr(pub Box<[Instr]>);

impl Grammar for Expr {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.0
            .iter()
            .map(|i| i.write(w))
            .collect::<io::Result<()>>()?;
        0x0bu8.write(w)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
    Numeric(Numeric),
    TruncSat(TruncSat),

    // Vector
    V128Const([u8; 16]),
    I8x16Shuffle([Laneidx; 16]),
    VectorMemarg(VectorMemarg, Memarg),
    VectorMemargLaneidx(VectorMemargLaneidx, Memarg, Laneidx),
    VectorLaneidx(VectorMemarg, Laneidx),
    VectorNoImmediate(VectorNoImmediate),
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
                instructions
                    .iter()
                    .map(|i| i.write(w))
                    .collect::<io::Result<()>>()?;
                0x0bu8.write(w)
            }
            Instr::Loop(bt, instructions) => {
                0x03u8.write(w)?;
                bt.write(w)?;
                instructions
                    .iter()
                    .map(|i| i.write(w))
                    .collect::<io::Result<()>>()?;
                0x0bu8.write(w)
            }
            Instr::If(bt, instructions) => {
                0x04u8.write(w)?;
                bt.write(w)?;
                instructions
                    .iter()
                    .map(|i| i.write(w))
                    .collect::<io::Result<()>>()?;
                0x0bu8.write(w)
            }
            Instr::IfElse(bt, in1, in2) => {
                0x04u8.write(w)?;
                bt.write(w)?;
                in1.iter().map(|i| i.write(w)).collect::<io::Result<()>>()?;
                0x05u8.write(w)?;
                in2.iter().map(|i| i.write(w)).collect::<io::Result<()>>()?;
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
                0xd0u8.write(w)?;
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
            Instr::Numeric(opcode) => (*opcode as u8).write(w),
            Instr::TruncSat(opcode) => {
                0xfcu8.write(w)?;
                (*opcode as u32).write(w)
            }

            // Vector
            Instr::VectorMemarg(opcode, m) => {
                0xfdu8.write(w)?;
                (*opcode as u32).write(w)?;
                m.write(w)
            }
            Instr::VectorMemargLaneidx(opcode, m, l) => {
                0xfdu8.write(w)?;
                (*opcode as u32).write(w)?;
                m.write(w)?;
                l.write(w)
            }
            Instr::V128Const(b) => {
                0xfdu8.write(w)?;
                12u32.write(w)?;
                b.iter().map(|b| b.write(w)).collect()
            }
            Instr::I8x16Shuffle(l) => {
                0xfdu8.write(w)?;
                13u32.write(w)?;
                l.iter().map(|l| l.write(w)).collect()
            }
            Instr::VectorLaneidx(opcode, lane) => {
                0xfdu8.write(w)?;
                (*opcode as u32).write(w)?;
                lane.write(w)
            }
            Instr::VectorNoImmediate(opcode) => {
                0xfdu8.write(w)?;
                (*opcode as u32).write(w)
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Numeric {
    I32Eqz = 0x45,
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
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TruncSat {
    I32TruncSatF32S = 0,
    I32TruncSatF32U,
    I32TruncSatF64S,
    I32TruncSatF64U,
    I64TruncSatF32S,
    I64TruncSatF32U,
    I64TruncSatF64S,
    I64TruncSatF64U,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VectorMemarg {
    V128Load = 0,
    V128Load8x8S,
    V128Load8x8U,
    V128Load16x4S,
    V128Load16x4U,
    V128Load32x2S,
    V128Load32x2U,
    V128Load8Splat,
    V128Load16Splat,
    V128Load32Splat,
    V128Load64Splat,
    V128Load32Zero = 92,
    V128Load64Zero,
    V128Store = 11,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VectorMemargLaneidx {
    V128Load8Lane = 84,
    V128Load16Lane,
    V128Load32Lane,
    V128Load64Lane,
    V128Store8Lane,
    V128Store16Lane,
    V128Store32Lane,
    V128Store64Lane,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VectorLaneidx {
    I8x16ExtractLaneS = 21,
    I8x16ExtractLaneU,
    I8x16ReplaceLane,
    I16x8ExtractLaneS,
    I16x8ExtractLaneU,
    I16x8ReplaceLane,
    I32x4ExtractLane,
    I32x4ReplaceLane,
    I64x2ExtractLane,
    I64x2ReplaceLane,
    F32x4ExtractLane,
    F32x4ReplaceLane,
    F64x2ExtractLane,
    F64x2ReplaceLane,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VectorNoImmediate {
    //
    I8x16Swizzle = 14,
    I8x16Splat,
    I16x8Splat,
    I32x4Splat,
    I64x2Splat,
    F32x4Splat,
    F64x2Splat,
    //
    I8x16Eq = 35,
    I8x16Ne,
    I8x16LtS,
    I8x16LtU,
    I8x16GtS,
    I8x16GtU,
    I8x16LeS,
    I8x16LeU,
    I8x16GeS,
    I8x16GeU,
    //
    I16x8Eq = 45,
    I16x8Ne,
    I16x8LtS,
    I16x8LtU,
    I16x8GtS,
    I16x8GtU,
    I16x8LeS,
    I16x8LeU,
    I16x8GeS,
    I16x8GeU,
    //
    I32x4Eq = 55,
    I32x4Ne,
    I32x4LtS,
    I32x4LtU,
    I32x4GtS,
    I32x4GtU,
    I32x4LeS,
    I32x4LeU,
    I32x4GeS,
    I32x4GeU,
    //
    I64x2Eq = 214,
    I64x2Ne,
    I64x2LtS,
    I64x2GtS,
    I64x2LeS,
    I64x2GeS,
    //
    F32x4Eq = 65,
    F32x4Ne,
    F32x4LtS,
    F32x4GtS,
    F32x4LeS,
    F32x4GeS,
    //
    F64x2Eq = 71,
    F64x2Ne,
    F64x2LtS,
    F64x2GtS,
    F64x2LeS,
    F64x2GeS,
    //
    V128Not = 77,
    V128And,
    V128AndNot,
    V128Or,
    V128Xor,
    V128Bitselect,
    V128AnyTrue,
    //
    I8x16Abs = 96,
    I8x16Neg,
    I8x16Popcnt,
    I8x16AllTrue,
    I8x16Bitmask,
    I8x16NarrowI16x8S,
    I8x16NarrowI16x8U,
    I8x16Shl = 107,
    I8x16ShrS,
    I8x16ShrU,
    I8x16Add,
    I8x16AddSatS,
    I8x16AddSatU,
    I8x16Sub,
    I8x16SubSatS,
    I8x16SubSatU,
    I8x16MinS = 118,
    I8x16MinU,
    I8x16MaxS,
    I8x16MaxU,
    I8x16AvgrU = 123,
    //
    I16x8ExtaddPairwise = 124,
    I16x8Abs,
    I16x8Neg = 128,
    I16x8Q15MulrSatS,
    I16x8AllTrue,
    I16x8Bitmask,
    I16x8NarrowI32x4S,
    I16x8NarrowI32x4U,
    I16x8ExtendLowI8x16S,
    I16x8ExtendHighI8x16S,
    I16x8ExtendLowI8x16U,
    I16x8ExtendHighI8x16U,
    I16x8Shl,
    I16x8ShrS,
    I16x8ShrU,
    I16x8Add,
    I16x8AddSatS,
    I16x8AddSatU,
    I16x8Sub,
    I16x8SubSatS,
    I16x8SubSatU,
    I16x8Mul = 149,
    I16x8MinS,
    I16x8MinU,
    I16x8MaxS,
    I16x8MaxU,
    I16x8AvgrU = 155,
    I16x8ExtmulLowI8x16S,
    I16x8ExtmulHighI8x16S,
    I16x8ExtmulLowI8x16U,
    I16x8ExtmulHighI8x16U,
    //
    I32x4ExtaddPairwiseS = 126,
    I32x4ExtaddPairwiseU,
    I32x4Abs = 160,
    I32x4Neg,
    I32x4Q15MulrSatS,
    I32x4AllTrue = 163,
    I32x4Bitmask,
    I32x4ExtendLowI8x16S = 167,
    I32x4ExtendHighI8x16S,
    I32x4ExtendLowI8x16U,
    I32x4ExtendHighI8x16U,
    I32x4Shl,
    I32x4ShrS,
    I32x4ShrU,
    I32x4Add,
    I32x4AddSatS,
    I32x4AddSatU,
    I32x4Sub = 177,
    I32x4Mul,
    I32x4MinS,
    I32x4MinU,
    I32x4MaxS,
    I32x4MaxU,
    I32x4AvgrU,
    I32x4ExtmulLowI8x16S = 188,
    I32x4ExtmulHighI8x16S,
    I32x4ExtmulLowI8x16U,
    I32x4ExtmulHighI8x16U,
    //
    I64x2Abs = 192,
    I64x2Neg,
    I64x2AllTrue = 195,
    I64x2Bitmask,
    I64x2ExtendLowI32x4S = 199,
    I64x2ExtendHighI32x4S,
    I64x2ExtendLowI32x4U,
    I64x2ExtendHighI32x4U,
    I64x2Shl,
    I64x2ShrS,
    I64x2ShrU,
    I64x2Add,
    I64x2Sub = 209,
    I64x2Mul = 213,
    I64x2ExtlowLowI32x4S = 220,
    I64x2ExtlowHighI32x4S,
    I64x2ExtlowLowI32x4U,
    I64x2ExtlowHighI32x4U,
    //
    F32x4Ceil = 103,
    F32x4Floor,
    F32x4Trunc,
    F32x4Nearest,
    F32x4Abs = 224,
    F32x4Neg,
    F32x4Sqrt,
    F32x4Add,
    F32x4Sub,
    F32x4Mul,
    F32x4Div,
    F32x4Min,
    F32x4Max,
    F32x4Pmin,
    F32x4Pmax,
    //
    F64x2Ceil = 116,
    F64x2Floor,
    F64x2Trunc = 122,
    F64x2Nearest = 148,
    F64x2Abs = 236,
    F64x2Neg,
    F64x2Sqrt = 239,
    F64x2Add,
    F64x2Sub,
    F64x2Mul,
    F64x2Div,
    F64x2Min,
    F64x2Max,
    F64x2Pmin,
    F64x2Pmax,
    //
    I32x4TruncSatF32x4S = 248,
    I32x4TruncSatF32x4U,
    F32x4ConvertI32x4S,
    F32x4ConvertI32x4U,
    I32x4TruncSatF64x2SZero,
    I32x4TruncSatF64x2UZero,
    F64x2ConvertLowI32x4S,
    F64x2ConvertLowI32x4U,
    F32x4DemoteF64x2Zero = 94,
    F64x2PromoteLowF32x4,
}
