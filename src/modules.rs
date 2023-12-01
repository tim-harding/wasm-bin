use crate::Grammar;
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
