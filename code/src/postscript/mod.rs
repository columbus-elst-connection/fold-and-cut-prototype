use std::io::{self, Write, Error};
use crate::figure::Figure;

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
    fn to_postscript(&self, w: &mut Write) -> Result<(), Error>;
}

impl<T> PostScript for Document<T>
where
    T: PostScript,
{
    fn to_postscript(&self, w: &mut Write) -> Result<(), Error> {
        self.figure.to_postscript(w)
    }
}

impl<T> PostScript for Figure<T>
where
    T: PostScript,
{
    fn to_postscript(&self, w: &mut Write) -> Result<(), Error> {
        w.write(b"[")?;
        match self {
            Figure::Open(points) => {
                w.write(b"(open)")?;
                for point in points {
                    point.to_postscript(w)?
                }
            }
            Figure::Closed(points) => {
                w.write(b"(closed)")?;
                for point in points {
                    point.to_postscript(w)?
                }
            }
            Figure::Composed(figures) => {
                w.write(b"(compose)")?;
                for figure in figures {
                    figure.to_postscript(w)?
                }
            }
        }
        w.write(b"]").map(|_| ())
    }
}

impl PostScript for u16 {
    fn to_postscript(&self, w: &mut Write) -> Result<(), Error> {
        w.write_fmt(format_args!("{}", self))
    }
}