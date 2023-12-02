use std::io::{self, Write};

use crate::{
    modules::{Dataidx, Elemidx, Funcidx, Globalidx, Labelidx, Localidx, Tableidx, Typeidx},
    types::{Reftype, Valtype},
    values::S33,
    write_all, Grammar, Vector,
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct S33(pub i64);

impl S33 {
    pub fn new(n: i64) -> Option<Self> {
        (64 - n.leading_zeros() <= 33 && n >= 0).then_some(Self(n))
    }
}

impl Grammar for S33 {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.0.write(w)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Instr {
    Opcode(Opcode),
    // Control
    Block(Blocktype, Box<[Instr]>),
    Loop(Blocktype, Box<[Instr]>),
    If(Blocktype, Box<[Instr]>),
    IfElse(Blocktype, Box<[Instr]>, Box<[Instr]>),
    Br(Labelidx),
    BrIf(Labelidx),
    BrTable(Vector<Labelidx>, Labelidx),
    Call(Funcidx),
    CallIndirect(Typeidx, Tableidx),
    // Reference
    RefNull(Reftype),
    RefFunc(Funcidx),
    // Parametric
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
    MemoryMemarg(MemoryMemarg, Memarg),
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
            Instr::Opcode(op) => op.write(w),
            // Control
            Instr::Block(bt, r#in) => write_all!(w, 0x02u8, bt, r#in, 0x0bu8),
            Instr::Loop(bt, r#in) => write_all!(w, 0x03u8, bt, r#in, 0x0bu8),
            Instr::If(bt, r#in) => write_all!(w, 0x04u8, bt, r#in, 0x0bu8),
            Instr::IfElse(bt, in1, in2) => write_all!(w, 0x04u8, bt, in1, 0x05u8, in2, 0x0bu8),
            Instr::Br(l) => write_all!(w, 0x0cu8, l),
            Instr::BrIf(l) => write_all!(w, 0x0du8, l),
            Instr::BrTable(l, default) => write_all!(w, 0x0eu8, l, default),
            Instr::Call(x) => write_all!(w, 0x10u8, x),
            Instr::CallIndirect(y, x) => write_all!(w, 0x11u8, y, x),

            // Reference
            Instr::RefNull(t) => write_all!(w, 0xd0u8, t),
            Instr::RefFunc(x) => write_all!(w, 0xd2u8, x),

            // Parametric
            Instr::Select(t) => {
                if let Some(t) = t {
                    write_all!(w, 0x1cu8, t)
                } else {
                    0x1bu8.write(w)
                }
            }

            // Variable
            Instr::LocalGet(x) => write_all!(w, 0x20u8, x),
            Instr::LocalSet(x) => write_all!(w, 0x21u8, x),
            Instr::LocalTee(x) => write_all!(w, 0x22u8, x),
            Instr::GlobalGet(x) => write_all!(w, 0x23u8, x),
            Instr::GlobalSet(x) => write_all!(w, 0x24u8, x),

            // Table
            Instr::TableGet(x) => write_all!(w, 0x25u8, x),
            Instr::TableSet(x) => write_all!(w, 0x26u8, x),
            Instr::TableInit(y, x) => write_all!(w, 0xfcu8, 12u32, y, x),
            Instr::ElemDrop(x) => write_all!(w, 0xfcu8, 13u32, x),
            Instr::TableCopy(x, y) => write_all!(w, 0xfcu8, 14u32, x, y),
            Instr::TableGrow(x) => write_all!(w, 0xfcu8, 15u32, x),
            Instr::TableSize(x) => write_all!(w, 0xfcu8, 16u32, x),
            Instr::TableFill(x) => write_all!(w, 0xfcu8, 17u32, x),

            // Memory
            Instr::MemoryMemarg(op, m) => write_all!(w, op, m),
            Instr::MemorySize => w.write_all(&[0x3f, 0x00]),
            Instr::MemoryGrow => w.write_all(&[0x40, 0x00]),
            Instr::MemoryInit(x) => write_all!(w, 0xfcu8, 8u32, x, 0x00),
            Instr::DataDrop(x) => write_all!(w, 0xfcu8, 9u32, x),
            Instr::MemoryCopy => write_all!(w, 0xfcu8, 10u32, 0x00u8, 0x00u8),
            Instr::MemoryFill => write_all!(w, 0xfcu8, 11u32, 0x00u8),

            // Numeric
            Instr::I32Const(n) => write_all!(w, 0x41u8, n),
            Instr::I64Const(n) => write_all!(w, 0x42u8, n),
            Instr::F32Const(n) => write_all!(w, 0x43u8, n),
            Instr::F64Const(n) => write_all!(w, 0x44u8, n),
            Instr::TruncSat(op) => write_all!(w, 0xfcu8, op),

            // Vector
            Instr::VectorMemarg(op, m) => write_all!(w, 0xfdu8, op, m),
            Instr::VectorMemargLaneidx(op, m, l) => write_all!(w, 0xfdu8, op, m, l),
            Instr::V128Const(b) => write_all!(w, 0xfdu8, 12u32, b.as_slice()),
            Instr::I8x16Shuffle(l) => write_all!(w, 0xfdu8, 13u32, l.as_slice()),
            Instr::VectorLaneidx(op, l) => write_all!(w, 0xfdu8, op, l),
            Instr::VectorNoImmediate(op) => write_all!(w, 0xfdu8, op),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryMemarg {
    I32Load = 0x28,
    I64Load,
    F32Load,
    F64Load,
    I32Load8S,
    I32Load8U,
    I32Load16S,
    I32Load16U,
    I64Load8S,
    I64Load8U,
    I64Load16S,
    I64Load16U,
    I64Load32S,
    I64Load32U,
    I32Store,
    I64Store,
    F32Store,
    F64Store,
    I32Store8,
    I32Store16,
    I64Store8,
    I64Store16,
    I64Store32,
}

impl Grammar for MemoryMemarg {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        (*self as u8).write(w)
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    Unreachable = 0x00,
    Nop = 0x01u8,
    Return = 0x0f,
    RefIsNull = 0xd1,
    Drop = 0x1a,
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

impl Grammar for Opcode {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        (*self as u8).write(w)
    }
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

impl Grammar for TruncSat {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        (*self as u32).write(w)
    }
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

impl Grammar for VectorMemarg {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        (*self as u32).write(w)
    }
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

impl Grammar for VectorMemargLaneidx {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        (*self as u32).write(w)
    }
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

impl Grammar for VectorLaneidx {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        (*self as u32).write(w)
    }
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

impl Grammar for VectorNoImmediate {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        (*self as u32).write(w)
    }
}
