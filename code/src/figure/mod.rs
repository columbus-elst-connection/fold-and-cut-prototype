use crate::postscript::{self, PostScript};
use std::io::Write;

pub fn open<T>(points: Vec<Point<T>>) -> Figure<T> {
    Figure::Open(points)
}

pub fn closed<T>(points: Vec<Point<T>>) -> Figure<T> {
    Figure::Closed(points)
}

pub fn compose<T>(points: Vec<Figure<T>>) -> Figure<T> {
    Figure::Composed(points)
}

pub enum Figure<T> {
    Open(Vec<Point<T>>),
    Closed(Vec<Point<T>>),
    Composed(Vec<Figure<T>>),
}

#[derive(Debug, PartialEq)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> From<[T; 2]> for Point<T> {
    fn from(data: [T; 2]) -> Self {
        let [x, y] = data;
        Self { x, y }
    }
}

impl<T> PostScript for Point<T>
where
    T: PostScript,
{
    fn to_postscript(&self, w: &mut dyn Write) -> Result<(), postscript::Error> {
        w.write_all(b"[")?;
        self.x.to_postscript(w)?;
        w.write_all(b" ")?;
        self.y.to_postscript(w)?;
        w.write_all(b"]")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn points_are_equal_when_coordinates_are_equal() {
        let u: Point<u16> = Point::from([0, 1]);
        let v: Point<u16> = Point::from([0, 1]);

        assert_eq!(u, v);
    }

    #[test]
    fn points_are_different_when_coordinates_are_unequal() {
        let u: Point<u16> = Point::from([0, 1]);
        let v: Point<u16> = Point::from([0, 0]);
        let w: Point<u16> = Point::from([1, 0]);

        assert!(u != v);
        assert!(u != w);
        assert!(v != w);
    }
}
