use crate::Grammar;
use std::io::{self, Write};

impl Grammar for u8 {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&[*self])
    }
}

impl Grammar for u64 {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let _ = leb128::write::unsigned(w, *self)?;
        Ok(())
    }
}

impl Grammar for u32 {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        ((*self) as u64).write(w)
    }
}

impl Grammar for i64 {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let _ = leb128::write::signed(w, *self)?;
        Ok(())
    }
}

impl Grammar for i32 {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        ((*self) as i64).write(w)
    }
}

impl Grammar for f32 {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&self.to_le_bytes())
    }
}

impl Grammar for f64 {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&self.to_le_bytes())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct S33(pub i64);

impl S33 {
    pub fn new(n: i64) -> Option<Self> {
        (64 - n.leading_zeros() <= 33).then_some(Self(n))
    }
}

impl Grammar for S33 {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.0.write(w)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Unsigned {
    U32(u32),
    U64(u64),
}

impl Grammar for Unsigned {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let n = match *self {
            Unsigned::U32(n) => n as u64,
            Unsigned::U64(n) => n,
        };
        n.write(w)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Signed {
    S32(i32),
    S64(i64),
}

impl Grammar for Signed {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let n = match *self {
            Signed::S32(n) => n as i64,
            Signed::S64(n) => n,
        };
        n.write(w)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Uninterpreted {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}

impl Grammar for Uninterpreted {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let n = match *self {
            Uninterpreted::I8(n) => n as i64,
            Uninterpreted::I16(n) => n as i64,
            Uninterpreted::I32(n) => n as i64,
            Uninterpreted::I64(n) => n,
        };
        n.write(w)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Float {
    F32(f32),
    F64(f64),
}

impl Grammar for Float {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match *self {
            Float::F32(n) => n.write(w),
            Float::F64(n) => n.write(w),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name(String);

impl Grammar for Name {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(self.0.as_bytes())
    }
}
