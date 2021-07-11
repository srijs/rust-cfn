use std::io::{Result, Write};
use std::fmt::Arguments;

static PADDING: &str = "                            ";

pub struct Printer {
    inner: Box<dyn Write>,
    indent: usize
}

impl Printer {
    pub fn new<W: Write + 'static>(w: W) -> Printer {
        Printer { inner: Box::new(w), indent: 0 }
    }

    pub fn line(&mut self, args: Arguments) -> Result<()> {
        writeln!(self.inner, "{}{}", &PADDING[0..self.indent], args)
    }

    pub fn newline(&mut self) -> Result<()> {
        self.inner.write_all(b"\n")
    }

    pub fn block<F>(&mut self, args: Arguments, f: F) -> Result<()>
        where F: FnOnce(&mut Printer) -> Result<()>
    {
        writeln!(self.inner, "{}{} {{", &PADDING[0..self.indent], args)?;
        self.indent += 4;
        f(self)?;
        self.indent -= 4;
        writeln!(self.inner, "{}}}", &PADDING[0..self.indent])
    }
}
