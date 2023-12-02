pub mod instructions;
pub mod modules;
pub mod types;
pub mod values;

use std::io::{self, Write};

pub trait Grammar {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()>;
}

#[macro_export]
macro_rules! write_all {
    ($w:expr, $($e:expr),*) => {
        {
            $($e.write($w)?;)*
            Ok(())
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector<T>(pub Box<[T]>);

impl<T> Grammar for Vector<T>
where
    T: Grammar,
{
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        (self.0.len() as u32).write(w)?;
        self.0.as_ref().write(w)
    }
}

impl<T> Grammar for &[T]
where
    T: Grammar,
{
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.iter().map(|i| i.write(w)).collect()
    }
}

impl<T> Grammar for Box<[T]>
where
    T: Grammar,
{
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.as_ref().write(w)
    }
}

impl<T, const N: usize> Grammar for [T; N]
where
    T: Grammar,
{
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.as_ref().write(w)
    }
}

impl<T> Grammar for Option<T>
where
    T: Grammar,
{
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match self {
            Some(s) => s.write(w),
            None => Ok(()),
        }
    }
}
