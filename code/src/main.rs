extern crate prototype;

use prototype::figure::{closed, compose, open, Point};
use prototype::postscript::{self, Document, PostScript};
use prototype::template::{self, compile};
use std::convert::From;
use std::fs;
use std::io;

fn main() -> Result<(), Error> {
    let figure = compose(vec![
        open(vec![
            Point::from([100u16, 100]),
            Point::from([900, 100]),
            Point::from([900, 900]),
            Point::from([100, 900]),
        ]),
        closed(vec![
            Point::from([200u16, 200]),
            Point::from([800, 200]),
            Point::from([800, 800]),
            Point::from([200, 800]),
        ]),
    ]);

    let source = fs::read_to_string("assets/tpl/diagram.tpl")?;
    let template = compile(source)?;
    let mut document = Document::with(template);
    document.embed(figure);

    let mut out = io::stdout();
    document.to_postscript(&mut out)?;
    Ok(())
}

#[derive(Debug)]
enum Error {
    IO(io::Error),
    Template(template::Error),
    PostScript(postscript::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<template::Error> for Error {
    fn from(error: template::Error) -> Self {
        Error::Template(error)
    }
}

impl From<postscript::Error> for Error {
    fn from(error: postscript::Error) -> Self {
        Error::PostScript(error)
    }
}
