use crate::figure::Figure;
use crate::template::{Data, Template};
use std::collections::HashMap;
use std::convert::From;
use std::io::{self, Write};

pub struct Document<T> {
    template: Template,
    figure: Option<Figure<T>>,
}

impl<T> Document<T> {
    pub fn with(template: Template) -> Self {
        Self {
            template,
            figure: None,
        }
    }

    pub fn embed<F>(&mut self, figure: F)
    where
        F: Into<Figure<T>>,
    {
        self.figure = Some(figure.into())
    }
}

pub trait PostScript {
    fn to_postscript(&self, w: &mut dyn Write) -> Result<(), Error>;
}

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Encoding(std::string::FromUtf8Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::Encoding(error)
    }
}

impl<T> PostScript for Document<T>
where
    T: PostScript,
{
    fn to_postscript(&self, w: &mut dyn Write) -> Result<(), Error> {
        let mut data = HashMap::new();
        for figure in self.figure.iter() {
            let mut postscript = Vec::new();
            figure.to_postscript(&mut postscript)?;
            let postscript = String::from_utf8(postscript)?;
            data.insert("figure".to_owned(), postscript);
        }
        let data = Data::from(data);
        self.template.render(w, &data)?;
        Ok(())
    }
}

impl<T> PostScript for Figure<T>
where
    T: PostScript,
{
    fn to_postscript(&self, w: &mut dyn Write) -> Result<(), Error> {
        w.write_all(b"[")?;
        match self {
            Figure::Open(points) => {
                w.write_all(b"(open)")?;
                points.to_postscript(w)?;
            }
            Figure::Closed(points) => {
                w.write_all(b"(closed)")?;
                points.to_postscript(w)?;
            }
            Figure::Composed(figures) => {
                w.write_all(b"(compose)")?;
                figures.to_postscript(w)?;
            }
        }
        w.write_all(b"]")?;
        Ok(())
    }
}

impl<T> PostScript for Vec<T>
where
    T: PostScript,
{
    fn to_postscript(&self, w: &mut dyn Write) -> Result<(), Error> {
        w.write_all(b"[")?;
        for p in self {
            p.to_postscript(w)?
        }
        w.write_all(b"]")?;
        Ok(())
    }
}

impl PostScript for u16 {
    fn to_postscript(&self, w: &mut dyn Write) -> Result<(), Error> {
        w.write_fmt(format_args!("{}", self))?;
        Ok(())
    }
}
