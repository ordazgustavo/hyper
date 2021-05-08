use nom::{branch::alt, bytes::complete::tag, combinator::value, IResult};
use std::collections::HashMap;

mod utils;

use utils::*;

#[derive(Debug, PartialEq)]
pub struct Document {
    pub loc: Loc,
    pub content: Element,
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub loc: Loc,
    pub tag: Tag,
    pub attributes: Option<Attributes>,
    pub content: Option<Content>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tag {
    Html,
    Head,
    Title,
    Body,
    H1,
}

#[derive(Debug, PartialEq)]
pub struct Attributes {
    pub loc: Loc,
    pub attr: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub struct Content {
    pub loc: Loc,
    pub children: Vec<Child>,
}

#[derive(Debug, PartialEq)]
pub enum Child {
    Text(Loc),
    Element(Element),
}

fn parse_tag(input: Span) -> IResult<Span, Tag> {
    alt((
        value(Tag::Html, tag("html")),
        value(Tag::Head, tag("head")),
        value(Tag::Title, tag("title")),
        value(Tag::Body, tag("body")),
        value(Tag::H1, tag("h1")),
    ))(input)
}

fn parse_element(input: Span) -> IResult<Span, Element> {
    located(parse_tag, |loc, tag| Element {
        loc,
        tag,
        attributes: None,
        content: None,
    })(input)
}

pub struct Parser {}

impl Parser {
    pub fn parse(source: &str) -> Result<Document, String> {
        let result =
            located(parse_element, |loc, content| Document { loc, content })(source.into());
        match result {
            Ok((_, content)) => Ok(content),
            Err(_) => Err(format!("Failed to parse source")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_works() {
        assert_eq!(
            Parser::parse("html").unwrap(),
            Document {
                loc: Loc {
                    start: Position { line: 1, column: 1 },
                    end: Position { line: 1, column: 5 },
                },
                content: Element {
                    loc: Loc {
                        start: Position { line: 1, column: 1 },
                        end: Position { line: 1, column: 5 },
                    },
                    tag: Tag::Html,
                    attributes: None,
                    content: None,
                }
            }
        )
    }
}
