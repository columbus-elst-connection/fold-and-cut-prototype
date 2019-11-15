use std::cmp::Ordering;

pub trait Parser<T> {
    fn parse<'a>(&self, input: &'a str) -> Result<(T, &'a str), Error> {
        self._parse(0, input)
    }

    fn _parse<'a>(&self, offset: usize, input: &'a str) -> Result<(T, &'a str), Error>;
}

#[derive(Debug, PartialEq)]
pub struct Window {
    offset: usize,
    length: usize,
}

impl Window {
    pub fn end(&self) -> usize {
        self.offset + self.length
    }
}

impl From<(usize, usize)> for Window {
    fn from(data: (usize, usize)) -> Self {
        let (offset, length) = data;
        Window { offset, length }
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    ExpectedLiteral(String),
    UnexpectedCharacter(char),
    UnexpectedEOF,
    ExpectedOneOfToParse,
    ExpectedToAvoid(String),
}

struct Literal<'p>(&'p str);

impl<'p> Parser<Window> for Literal<'p> {
    fn _parse<'a>(&self, offset: usize, input: &'a str) -> Result<(Window, &'a str), Error> {
        if input.starts_with(self.0) {
            let length = self.0.len();
            let remainder = &input[length..];
            Ok((Window::from((offset, length)), remainder))
        } else {
            Err(Error::ExpectedLiteral(self.0.to_owned()))
        }
    }
}

fn literal(match_exactly: &str) -> Literal {
    Literal(match_exactly)
}

pub struct Any<F>
where
    F: Fn(char) -> bool + Sized,
{
    predicate: F,
}

impl<F> Parser<Window> for Any<F>
where
    F: Fn(char) -> bool + Sized,
{
    fn _parse<'a>(&self, offset: usize, input: &'a str) -> Result<(Window, &'a str), Error> {
        if let Some(character) = input.chars().next() {
            if (self.predicate)(character) {
                Ok((Window::from((offset, 1)), &input[1..]))
            } else {
                Err(Error::UnexpectedCharacter(character))
            }
        } else {
            Err(Error::UnexpectedEOF)
        }
    }
}

struct Avoid<'p>(&'p str);

impl<'p> Parser<Window> for Avoid<'p> {
    fn _parse<'a>(&self, offset: usize, input: &'a str) -> Result<(Window, &'a str), Error> {
        let mut local_offset: usize = 0;

        while local_offset < input.len() && !input[local_offset..].starts_with(self.0) {
            local_offset += 1;
        }
        if local_offset != 0 {
            Ok((Window::from((offset, local_offset)), &input[(offset+local_offset)..]))
        } else {
            Err(Error::ExpectedToAvoid(self.0.to_owned()))
        }
    }
}

fn avoid(match_exactly: &str) -> Avoid {
    Avoid(match_exactly)
}


struct Map<I, O, P, F> where P: Parser<I> + Sized, F: Fn(I) -> O + Sized {
    parser: P,
    map: F,
    marker: std::marker::PhantomData<I>
}

impl<I, O, P, F> Parser<O> for Map<I, O, P, F> where P: Parser<I> + Sized, F: Fn(I) -> O + Sized {
    fn _parse<'a>(&self, offset: usize, input: &'a str) -> Result<(O, &'a str), Error> {
        let attempt = self.parser._parse(offset, input);
        attempt.map(|(v, rest)| ((self.map)(v),rest))
    }
}

fn map<I, O, P, F>(parser: P, map: F) -> impl Parser<O> where P: Parser<I> + Sized, F: Fn(I) -> O + Sized {
    Map { parser, map, marker: std::marker::PhantomData }
}

fn any<F>(predicate: F) -> Any<F>
where
    F: Fn(char) -> bool + Sized,
{
    Any { predicate }
}

struct OneOf<T, P> where P : Parser<T> + Sized {
    options: Vec<P>,
    marker: std::marker::PhantomData<T>,
}

impl<T, P> Parser<T> for OneOf<T, P> where P: Parser<T> + Sized {
    fn _parse<'a>(&self, offset: usize, input: &'a str) -> Result<(T, &'a str), Error> {
        for ref parser in &self.options {
            let attempt = parser._parse(offset, input);
            if attempt.is_ok() {
                return attempt
            } 
        }
        Err(Error::ExpectedOneOfToParse)
    }
}

fn one_of<T, P>(options: Vec<P>) -> OneOf<T, P> where P: Parser<T> + Sized {
    OneOf { options, marker: std::marker::PhantomData }
}

struct Between<T, P>
where
    P: Parser<T> + Sized,
{
    lower_limit: u64,
    upper_limit: Limit,
    parser: P,
    marker: std::marker::PhantomData<T>
}

impl<T, P> Parser<Vec<T>> for Between<T, P> where P: Parser<T> + Sized {
    fn _parse<'a>(&self, offset: usize, input: &'a str) -> Result<(Vec<T>, &'a str), Error> {
        let mut result = Vec::new();
        let mut current_offset = offset;
        let mut source = input;
        let mut count = 0;
        while count < self.lower_limit {
            let attempt = self.parser._parse(current_offset, source);
            match attempt {
                Ok((value, rest)) => {
                    current_offset = input.len() - rest.len();
                    result.push(value);
                    source = rest;
                }

                Err(e) => {
                    return Err(e);
                }
            }
            count += 1;
        }
        while Limit::At(count) < self.upper_limit {
            let attempt = self.parser._parse(current_offset, source);
            match attempt {
                Ok((value, rest)) => {
                    current_offset = input.len() - rest.len();
                    result.push(value);
                    source = rest;
                }

                Err(_) => {
                    break;
                }
            }
            count += 1;
        }
        Ok((result, source))
    }
}

impl<T, P> Between<T, P> where P: Parser<T> + Sized {
    fn new(lower_limit: u64, upper_limit: Limit, parser: P) -> Self {
        Self { lower_limit, upper_limit, parser, marker: std::marker::PhantomData }
    }
}

pub fn between<T>(lower_limit: u64, upper_limit: u64, parser: impl Parser<T>) -> impl Parser<Vec<T>> {
    Between::new(lower_limit, Limit::At(upper_limit), parser)
}

pub fn at_least<T>(lower_limit: u64, parser: impl Parser<T>) -> impl Parser<Vec<T>> {
    Between::new(lower_limit, Limit::Infinity, parser)
}

pub fn many<T>(parser: impl Parser<T>) -> impl Parser<Vec<T>> {
    at_least(0, parser)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Limit {
    At(u64),
    Infinity,
}

impl Ord for Limit {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Limit::At(u) => match other {
                Limit::At(v) => u.cmp(v),

                Limit::Infinity => Ordering::Less,
            },
            Limit::Infinity => match other {
                Limit::Infinity => Ordering::Equal,
                _ => Ordering::Greater,
            },
        }
    }
}

impl PartialOrd for Limit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn literal_should_parse() {
        let input = "{{subject}}";
        let parser = literal("{{");

        let result = parser.parse(&input);
        
        assert_eq!(result, Ok((Window::from((0, 2)), "subject}}")))
    }

    #[test]
    fn any_should_parse() {
        let input = "abc";
        let parser = any(|c| c.is_ascii_alphabetic());

        let result = parser.parse(&input);
        
        assert_eq!(result, Ok((Window::from((0, 1)), "bc")))
    }

     #[test]
    fn avoid_should_parse() {
        let input = "abc{{subject}}";
        let parser = avoid("{{");

        let result = parser.parse(&input);
        
        assert_eq!(result, Ok((Window::from((0,3)), "{{subject}}")))
    }

   #[test]
    fn limit_should_compare() {
        assert_eq!(Ordering::Less, Limit::At(0).cmp(&Limit::At(1)));
        assert_eq!(Ordering::Equal, Limit::At(1).cmp(&Limit::At(1)));
        assert_eq!(Ordering::Greater, Limit::At(1).cmp(&Limit::At(0)));
        assert_eq!(Ordering::Less, Limit::At(0).cmp(&Limit::Infinity));
        assert_eq!(Ordering::Greater, Limit::Infinity.cmp(&Limit::At(0)));
        assert_eq!(Ordering::Equal, Limit::Infinity.cmp(&Limit::Infinity));
    }

    #[test]
    fn map_should_parse() {
        let input = "abc";
        let parser = map(any(|c| c.is_ascii_alphabetic()), |w| w.end());

        let result = parser.parse(&input);
        
        assert_eq!(result, Ok((1, "bc")))
    }

    #[test]
    fn one_of_should_parse() {
        let input = "abc";
        let parser = one_of(vec![literal("c"), literal("b"), literal("a")]);

        let result = parser.parse(&input);
        
        assert_eq!(result, Ok((Window::from((0,1)), "bc")))
    }

    #[test]
    fn many_should_parse() {
        let input = "abc{{subject}}";
        let parser = many(any(|c| c.is_ascii_alphabetic()));

        let result = parser.parse(&input);
        
        assert_eq!(result, Ok((vec![Window::from((0,1)), Window::from((1,1)), Window::from((2,1))], "{{subject}}")))
    }
}
