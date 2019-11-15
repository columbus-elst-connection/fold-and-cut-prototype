use crate::figure::Figure;
use std::fmt::{self, Display, Formatter};

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

impl<T> Display for Document<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.figure)
    }
}
