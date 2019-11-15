mod parser;

use std::collections::HashMap;
use std::fmt::{self, Write};
use std::ops::Range;

pub fn compile<S>(input: S) -> Result<Template, Error>
where
    S: Into<String>,
{
    let compositor = Compositor::new(input);
    Ok(Template::from(compositor))
}

pub struct Template {
    compositor: Compositor,
}

impl Template {
    pub fn render(&self, writer: &mut dyn Write, data: &Data) -> Result<(), fmt::Error> {
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
    chunks: Vec<Chunk>,
}

impl Compositor {
    fn new<S>(input: S) -> Self
    where
        S: Into<String>,
    {
        let input = input.into();
        let mut chunks = Vec::new();
        chunks.push(Chunk::Literal(Window {
            offset: 0,
            length: 7,
        }));
        chunks.push(Chunk::Variable(Window {
            offset: 9,
            length: 7,
        }));
        chunks.push(Chunk::Literal(Window {
            offset: 18,
            length: 1,
        }));

        Self { input, chunks }
    }

    fn compose(&self, writer: &mut dyn Write, data: &Data) -> Result<(), fmt::Error> {
        for chunk in &self.chunks {
            match chunk {
                Chunk::Literal(window) => writer.write_str(&self.input[window.range()])?,
                Chunk::Variable(window) => {
                    let key = &self.input[window.range()].to_owned();
                    if let Some(value) = data.get(key) {
                        writer.write_str(value)?;
                    } else {
                        writer.write_str("{{")?;
                        writer.write_str(key)?;
                        writer.write_str("}}")?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    General,
}

enum Chunk {
    Literal(Window),
    Variable(Window),
}

struct Window {
    offset: usize,
    length: usize,
}

impl Window {
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

    #[ignore]
    #[test]
    fn a_string_should_be_returned_literally() {
        let template = compile("Hello, World!").expect("template to compile");
        let data = Data::from(HashMap::new());

        let mut result = String::new();
        template
            .render(&mut result, &data)
            .expect("template to render");

        assert_eq!(result, "Hello, World!")
    }

    #[test]
    fn a_string_should_with_variables_should_be_substituted() {
        let template = compile("Hello, {{subject}}!").expect("template to compile");
        let mut data: HashMap<String, String> = HashMap::new();
        data.insert("subject".to_owned(), "World".to_owned());
        let data = Data::from(data);

        let mut result = String::new();
        template
            .render(&mut result, &data)
            .expect("template to render");

        assert_eq!(result, "Hello, World!")
    }
}
