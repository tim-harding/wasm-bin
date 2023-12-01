use std::io::{self, Write};

trait Grammar {
    fn write_bytes<W: Write>(&self, w: &mut W) -> io::Result<()>;
}

pub struct Byte(u8);

impl Grammar for Byte {
    fn write_bytes<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&[self.0])
    }
}

impl Grammar for u64 {
    fn write_bytes<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let _ = leb128::write::unsigned(w, *self)?;
        Ok(())
    }
}

impl Grammar for u32 {
    fn write_bytes<W: Write>(&self, w: &mut W) -> io::Result<()> {
        ((*self) as u64).write_bytes(w)
    }
}

impl Grammar for i64 {
    fn write_bytes<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let _ = leb128::write::signed(w, *self)?;
        Ok(())
    }
}

impl Grammar for i32 {
    fn write_bytes<W: Write>(&self, w: &mut W) -> io::Result<()> {
        ((*self) as i64).write_bytes(w)
    }
}

impl Grammar for i16 {
    fn write_bytes<W: Write>(&self, w: &mut W) -> io::Result<()> {
        ((*self) as i64).write_bytes(w)
    }
}

impl Grammar for i8 {
    fn write_bytes<W: Write>(&self, w: &mut W) -> io::Result<()> {
        ((*self) as i64).write_bytes(w)
    }
}

impl Grammar for f64 {
    fn write_bytes<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&self.to_le_bytes())
    }
}

impl Grammar for f32 {
    fn write_bytes<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&self.to_le_bytes())
    }
}

pub enum Unsigned {
    U32(u32),
    U64(u64),
}

impl Grammar for Unsigned {
    fn write_bytes<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let n = match *self {
            Unsigned::U32(n) => n as u64,
            Unsigned::U64(n) => n,
        };
        n.write_bytes(w)
    }
}

pub enum Signed {
    S32(i32),
    S64(i64),
}

impl Grammar for Signed {
    fn write_bytes<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let n = match *self {
            Signed::S32(n) => n as i64,
            Signed::S64(n) => n,
        };
        n.write_bytes(w)
    }
}

pub enum Uninterpreted {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}

impl Grammar for Uninterpreted {
    fn write_bytes<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let n = match *self {
            Uninterpreted::I8(n) => n as i64,
            Uninterpreted::I16(n) => n as i64,
            Uninterpreted::I32(n) => n as i64,
            Uninterpreted::I64(n) => n,
        };
        n.write_bytes(w)
    }
}

pub enum Float {
    F32(f32),
    F64(f64),
}

impl Grammar for Float {
    fn write_bytes<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match *self {
            Float::F32(n) => n.write_bytes(w),
            Float::F64(n) => n.write_bytes(w),
        }
    }
}

pub struct Name<'a>(&'a str);

impl<'a> Grammar for Name<'a> {
    fn write_bytes<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(self.0.as_bytes())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Numtype {
    I32,
    I64,
    F32,
    F64,
}

impl From<Numtype> for u8 {
    fn from(value: Numtype) -> Self {
        match value {
            Numtype::I32 => 0x7f,
            Numtype::I64 => 0x7e,
            Numtype::F32 => 0x7d,
            Numtype::F64 => 0x7c,
        }
    }
}

impl Grammar for Numtype {
    fn write_bytes<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&[(*self).into()])
    }
}
