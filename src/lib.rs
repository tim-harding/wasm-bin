pub mod types;
pub mod values;

use std::io::{self, Write};

pub trait Grammar {
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()>;
}

// https://webassembly.github.io/spec/core/binary/conventions.html#vectors
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector<'a, T>(pub &'a [T]);

impl<'a, T> Grammar for Vector<'a, T>
where
    T: Grammar,
{
    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        (self.0.len() as u32).write(w)?;
        self.0.into_iter().map(|t| t.write(w)).collect()
    }
}
