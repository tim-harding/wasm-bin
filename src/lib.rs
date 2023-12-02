pub mod instructions;
pub mod modules;
pub mod types;
pub mod values;

use std::io::{self, Write};

pub trait Grammar {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()>;
}

// https://webassembly.github.io/spec/core/binary/conventions.html#vectors
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector<T>(pub Box<[T]>);

impl<T> Grammar for Vector<T>
where
    T: Grammar,
{
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        (self.0.len() as u32).write(w)?;
        self.0.into_iter().map(|t| t.write(w)).collect()
    }
}
