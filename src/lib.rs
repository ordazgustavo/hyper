use nom::{
    branch::alt,
    bytes::complete::tag,
    character::streaming::char,
    combinator::{map, opt, value},
    multi::many1,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use std::collections::HashMap;

mod string;
mod utils;

use string::parse_string;
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
    Text(Loc, String),
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

fn parse_text_node(input: Span) -> IResult<Span, Vec<Child>> {
    located(parse_string, |loc, value| vec![Child::Text(loc, value)])(input)
}

fn parse_child_element(input: Span) -> IResult<Span, Vec<Child>> {
    delimited(
        char('{'),
        many1(map(parse_element, Child::Element)),
        char('}'),
    )(input)
}

fn parse_content(input: Span) -> IResult<Span, Content> {
    located(
        alt((parse_text_node, parse_child_element)),
        |loc, children| Content { loc, children },
    )(input)
}

fn parse_element(input: Span) -> IResult<Span, Element> {
    located(
        tuple((parse_tag, preceded(sp, opt(parse_content)))),
        |loc, (tag, content)| Element {
            loc,
            tag,
            attributes: None,
            content,
        },
    )(input)
}

pub struct Parser {}

impl Parser {
    pub fn parse(source: &str) -> Result<Document, String> {
        let result =
            located(parse_element, |loc, content| Document { loc, content })(source.into());
        match result {
            Ok((_, content)) => Ok(content),
            Err(e) => Err(format!("Failed to parse source {}", e)),
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
            Parser::parse("html {}").unwrap(),
            Document {
                loc: Loc {
                    start: Position { line: 1, column: 1 },
                    end: Position { line: 1, column: 6 },
                },
                content: Element {
                    loc: Loc {
                        start: Position { line: 1, column: 1 },
                        end: Position { line: 1, column: 6 },
                    },
                    tag: Tag::Html,
                    attributes: None,
                    content: None,
                }
            }
        )
    }

    #[test]
    fn it_parses_tag_with_child_text_node() {
        assert_eq!(
            Parser::parse(r#"h1 "Page Title""#).unwrap(),
            Document {
                loc: Loc {
                    start: Position { line: 1, column: 1 },
                    end: Position {
                        line: 1,
                        column: 16
                    },
                },
                content: Element {
                    loc: Loc {
                        start: Position { line: 1, column: 1 },
                        end: Position {
                            line: 1,
                            column: 16
                        },
                    },
                    tag: Tag::H1,
                    attributes: None,
                    content: Some(Content {
                        loc: Loc {
                            start: Position { line: 1, column: 4 },
                            end: Position {
                                line: 1,
                                column: 16
                            },
                        },
                        children: vec![Child::Text(
                            Loc {
                                start: Position { line: 1, column: 4 },
                                end: Position {
                                    line: 1,
                                    column: 16
                                },
                            },
                            "Page Title".to_owned()
                        )]
                    }),
                }
            }
        )
    }
}
