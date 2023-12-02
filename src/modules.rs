use crate::{
    instructions::Expr,
    types::{Functype, Globaltype, Memtype, Tabletype},
    values::Name,
    write_all, Grammar, Vector,
};
use std::io::{self, Write};

macro_rules! idx {
    ($t:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $t(u32);

        impl Grammar for $t {
            fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
                self.0.write(w)
            }
        }
    };
}

idx!(Typeidx);
idx!(Funcidx);
idx!(Tableidx);
idx!(Memidx);
idx!(Globalidx);
idx!(Elemidx);
idx!(Dataidx);
idx!(Localidx);
idx!(Labelidx);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Section<const N: u8, T>(pub T);

impl<const N: u8, T> Grammar for Section<N, T>
where
    T: Grammar,
{
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let mut buf = vec![];
        self.0.write(&mut buf)?;
        N.write(w)?;
        (buf.len() as u32).write(w)?;
        buf.as_slice().write(w)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Custom {
    pub name: Name,
    pub contents: Box<[u8]>,
}

impl Grammar for Custom {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        write_all!(w, self.name, self.contents)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Importdesc {
    Func(Typeidx),
    Table(Tabletype),
    Mem(Memtype),
    Global(Globaltype),
}

impl Grammar for Importdesc {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match self {
            Importdesc::Func(x) => write_all!(w, 0x00u8, x),
            Importdesc::Table(tt) => write_all!(w, 0x01u8, tt),
            Importdesc::Mem(mt) => write_all!(w, 0x02u8, mt),
            Importdesc::Global(gt) => write_all!(w, 0x03u8, gt),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Import {
    pub r#mod: Name,
    pub nm: Name,
    pub d: Importdesc,
}

impl Grammar for Import {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        write_all!(w, self.r#mod, self.nm, self.d)
    }
}

macro_rules! section {
    ($i:ident, $n:expr, $t:ty) => {
        #[derive(Debug, Clone, PartialEq, PartialOrd)]
        pub struct $i(pub Section<$n, $t>);

        impl Grammar for $i {
            fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
                self.0.write(w)
            }
        }
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Table(pub Tabletype);

impl Grammar for Table {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.0.write(w)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mem(pub Memtype);

impl Grammar for Mem {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.0.write(w)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Global {
    pub gt: Globaltype,
    pub e: Expr,
}

impl Grammar for Global {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        write_all!(w, self.gt, self.e)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Exportdesc {
    Func(Funcidx),
    Table(Tableidx),
    Mem(Memidx),
    Global(Globalidx),
}

impl Grammar for Exportdesc {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match self {
            Exportdesc::Func(x) => write_all!(w, 0x00u8, x),
            Exportdesc::Table(x) => write_all!(w, 0x01u8, x),
            Exportdesc::Mem(x) => write_all!(w, 0x02u8, x),
            Exportdesc::Global(x) => write_all!(w, 0x03u8, x),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Export {
    pub nm: Name,
    pub d: Exportdesc,
}

impl Grammar for Export {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        write_all!(w, self.nm, self.d)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Start(pub Funcidx);

impl Grammar for Start {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.0.write(w)
    }
}

section!(Customsec, 0, Custom);
section!(Typesec, 1, Vector<Functype>);
section!(Importsec, 2, Vector<Import>);
section!(Funcsec, 3, Vector<Typeidx>);
section!(Tablesec, 4, Vector<Table>);
section!(Memsec, 5, Vector<Mem>);
section!(Globalsec, 6, Vector<Global>);
section!(Exportsec, 7, Vector<Export>);
section!(Startsec, 8, Start);
