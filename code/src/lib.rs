use std::fmt::{self, Display, Formatter};

pub enum Figure<T> {
    Open(Vec<Point<T>>),
    Closed(Vec<Point<T>>),
    Composed(Vec<Figure<T>>),
}

impl<T> Display for Figure<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[")?;
        match self {
            Figure::Open(points) => {
                write!(f, "(open)")?;
                for point in points {
                    write!(f, " {}", point)?;
                }
            }
            Figure::Closed(points) => {
                write!(f, "(closed)")?;
                for point in points {
                    write!(f, " {}", point)?;
                }
            }
            Figure::Composed(figures) => {
                write!(f, "(compose)")?;
                for figure in figures {
                    write!(f, " {}", figure)?;
                }
            }
        }
        write!(f, "]")
    }
}

pub fn open<T>(points: Vec<Point<T>>) -> Figure<T> {
    Figure::Open(points)
}

pub fn closed<T>(points: Vec<Point<T>>) -> Figure<T> {
    Figure::Closed(points)
}

pub fn compose<T>(points: Vec<Figure<T>>) -> Figure<T> {
    Figure::Composed(points)
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

impl<T> Display for Point<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{} {}]", self.x, self.y)
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
