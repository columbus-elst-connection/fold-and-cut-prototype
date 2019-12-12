extern crate prototype;

use std::io;
use prototype::figure::{closed, compose, open, Point};
use prototype::postscript::{Document, PostScript};


fn main() -> Result<(), io::Error> {
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

    let document = Document::with(figure);

    let mut out = io::stdout();
    document.to_postscript(&mut out)
}
