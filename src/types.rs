use crate::{Grammar, Vector};
use std::io::{self, Write};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Numtype {
    I32 = 0x7f,
    I64 = 0x7e,
    F32 = 0x7d,
    F64 = 0x7c,
}

impl Grammar for Numtype {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        ((*self) as u8).write(w)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Vectype {
    V128 = 0x7b,
}

impl Grammar for Vectype {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        ((*self) as u8).write(w)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Reftype {
    Funcref = 0x70,
    Externref = 0x6f,
}

impl Grammar for Reftype {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        ((*self) as u8).write(w)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Valtype {
    Numtype(Numtype),
    Vectype(Vectype),
    Reftype(Reftype),
}

impl Grammar for Valtype {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let b = match *self {
            Valtype::Numtype(n) => n as u8,
            Valtype::Vectype(n) => n as u8,
            Valtype::Reftype(n) => n as u8,
        };
        b.write(w)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Resulttype<'a>(pub Vector<'a, Valtype>);

impl<'a> Grammar for Resulttype<'a> {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.0.write(w)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Functype<'a> {
    pub parameters: Resulttype<'a>,
    pub results: Resulttype<'a>,
}

impl<'a> Grammar for Functype<'a> {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        0x60.write(w)?;
        self.parameters.write(w)?;
        self.results.write(w)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Limits {
    Min(u32),
    MinMax(u32, u32),
}

impl Grammar for Limits {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match self {
            Limits::Min(min) => {
                0x00.write(w)?;
                min.write(w)
            }
            Limits::MinMax(min, max) => {
                0x01.write(w)?;
                min.write(w)?;
                max.write(w)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Memtype(pub Limits);

impl Grammar for Memtype {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.0.write(w)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tabletype {
    pub element_type: Reftype,
    pub limits: Limits,
}

impl Grammar for Tabletype {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.element_type.write(w)?;
        self.limits.write(w)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mut {
    Const = 0x00,
    Var = 0x01,
}

impl Grammar for Mut {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        ((*self) as u8).write(w)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Globaltype {
    pub ty: Valtype,
    pub mutability: Mut,
}

impl Grammar for Globaltype {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.ty.write(w)?;
        self.mutability.write(w)
    }
}
