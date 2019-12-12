use crate::figure::Figure;
use std::io::{Error, Write};

pub struct Document<T> {
    figure: Figure<T>,
}

impl<T> Document<T> {
    pub fn with<F>(figure: F) -> Self
    where
        F: Into<Figure<T>>,
    {
        Self {
            figure: figure.into(),
        }
    }
}

pub trait PostScript {
    fn to_postscript(&self, w: &mut dyn Write) -> Result<(), Error>;
}

impl<T> PostScript for Document<T>
where
    T: PostScript,
{
    fn to_postscript(&self, w: &mut dyn Write) -> Result<(), Error> {
        self.figure.to_postscript(w)
    }
}

impl<T> PostScript for Figure<T>
where
    T: PostScript,
{
    fn to_postscript(&self, w: &mut dyn Write) -> Result<(), Error> {
        w.write(b"[")?;
        match self {
            Figure::Open(points) => {
                w.write(b"(open)")?;
                points.to_postscript(w)?;
            }
            Figure::Closed(points) => {
                w.write(b"(closed)")?;
                points.to_postscript(w)?;
            }
            Figure::Composed(figures) => {
                w.write(b"(compose)")?;
                figures.to_postscript(w)?;
            }
        }
        w.write(b"]").map(|_| ())
    }
}

impl<T> PostScript for Vec<T>
where
    T: PostScript,
{
    fn to_postscript(&self, w: &mut dyn Write) -> Result<(), Error> {
        w.write(b"[")?;
        for p in self {
            p.to_postscript(w)?
        }
        w.write(b"]").map(|_| ())
    }
}

impl PostScript for u16 {
    fn to_postscript(&self, w: &mut dyn Write) -> Result<(), Error> {
        w.write_fmt(format_args!("{}", self))
    }
}
