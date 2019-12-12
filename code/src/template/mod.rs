use std::collections::HashMap;
use std::io::{self, Write};
use std::ops::Range;
use std::str::FromStr;

pub fn compile<S>(input: S) -> Result<Template, Error>
where
    S: Into<String>,
{
    let compositor = Compositor::new(input)?;
    Ok(Template::from(compositor))
}

pub struct Template {
    compositor: Compositor,
}

impl Template {
    pub fn render(&self, writer: &mut dyn Write, data: &Data) -> Result<(), io::Error> {
        self.compositor.compose(writer, data)
    }
}

impl From<Compositor> for Template {
    fn from(compositor: Compositor) -> Self {
        Self { compositor }
    }
}

struct Compositor {
    input: String,
    chunks: Chunks,
}

impl Compositor {
    fn new<S>(input: S) -> Result<Self, Error>
    where
        S: Into<String>,
    {
        let input = input.into();
        let chunks = input.parse::<Chunks>()?;
        Ok(Self { input, chunks })
    }

    fn compose(&self, writer: &mut dyn Write, data: &Data) -> Result<(), io::Error> {
        for chunk in self.chunks.iter() {
            match chunk {
                Chunk::Literal(window) => {
                    writer.write_all(&self.input[window.range()].as_bytes())?
                }
                Chunk::Variable(window) => {
                    let key = &self.input[window.range()].to_owned();
                    if let Some(value) = data.get(key) {
                        writer.write_all(value.as_bytes())?;
                    } else {
                        writer.write_all(b"{{")?;
                        writer.write_all(key.as_bytes())?;
                        writer.write_all(b"}}")?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    LiteralCanNotStartWithTwoBraces,
    UnmatchedVariable,
}

struct Chunks {
    chunks: Vec<Chunk>,
}

impl Chunks {
    fn iter(&self) -> std::slice::Iter<Chunk> {
        self.chunks.iter()
    }
}

impl From<Vec<Chunk>> for Chunks {
    fn from(chunks: Vec<Chunk>) -> Self {
        Self { chunks }
    }
}

impl FromStr for Chunks {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.as_bytes();
        let mut chunks = Vec::new();
        let mut index = 0;
        while index < input.len() {
            let (chunk, delta) = if index + 1 < input.len() && input[index..index + 2] == b"{{"[0..]
            {
                parse_variable(&input[index..], index)?
            } else {
                parse_literal(&input[index..], index)?
            };
            chunks.push(chunk);
            index += delta;
        }
        Ok(Chunks::from(chunks))
    }
}

fn parse_variable(input: &[u8], offset: usize) -> Result<(Chunk, usize), Error> {
    let mut index = 2;
    while index < input.len() {
        if index + 1 < input.len() && input[index..index + 2] == b"}}"[0..] {
            break;
        }
        index += 1;
    }
    if index + 1 < input.len() && input[index..index + 2] == b"}}"[0..] {
        Ok((
            Chunk::Variable(Window::new(offset + 2, index - 2)),
            index + 2,
        ))
    } else {
        Err(Error::UnmatchedVariable)
    }
}

fn parse_literal(input: &[u8], offset: usize) -> Result<(Chunk, usize), Error> {
    let mut index = 0;
    while index < input.len() {
        if index + 1 < input.len() && input[index..index + 2] == b"{{"[0..] {
            break;
        }
        index += 1;
    }
    if index > 0 {
        Ok((Chunk::Literal(Window::new(offset, index)), index))
    } else {
        Err(Error::LiteralCanNotStartWithTwoBraces)
    }
}

#[derive(Copy, Clone)]
enum Chunk {
    Literal(Window),
    Variable(Window),
}

#[derive(Copy, Clone)]
struct Window {
    offset: usize,
    length: usize,
}

impl Window {
    fn new(offset: usize, length: usize) -> Self {
        Window { offset, length }
    }

    fn range(&self) -> Range<usize> {
        Range {
            start: self.offset,
            end: self.offset + self.length,
        }
    }
}

pub struct Data {
    data: HashMap<String, String>,
}

impl Data {
    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

impl From<HashMap<String, String>> for Data {
    fn from(data: HashMap<String, String>) -> Self {
        Self { data }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a_string_should_be_returned_literally() {
        let template = compile("Hello, World!").expect("template to compile");
        let data = Data::from(HashMap::new());

        let mut result = Vec::new();
        template
            .render(&mut result, &data)
            .expect("template to render");

        let result = String::from_utf8(result).expect("utf8 string");
        assert_eq!(result, "Hello, World!")
    }

    #[test]
    fn a_string_with_variable_should_be_substituted() {
        let template = compile("Hello, {{subject}}!").expect("template to compile");
        let mut data: HashMap<String, String> = HashMap::new();
        data.insert("subject".to_owned(), "World".to_owned());
        let data = Data::from(data);

        let mut result = Vec::new();
        template
            .render(&mut result, &data)
            .expect("template to render");

        let result = String::from_utf8(result).expect("utf8 string");
        assert_eq!(result, "Hello, World!")
    }

    #[test]
    fn a_string_with_variables_should_be_substituted() {
        let template = compile("{{greeting}}, {{subject}}!").expect("template to compile");
        let mut data: HashMap<String, String> = HashMap::new();
        data.insert("greeting".to_owned(), "Hello".to_owned());
        data.insert("subject".to_owned(), "World".to_owned());
        let data = Data::from(data);

        let mut result = Vec::new();
        template
            .render(&mut result, &data)
            .expect("template to render");

        let result = String::from_utf8(result).expect("utf8 string");
        assert_eq!(result, "Hello, World!")
    }
}
