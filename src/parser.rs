use nom::{
    branch::alt,
    bytes::complete::tag,
    character::streaming::char,
    combinator::{map, opt, value},
    multi::many1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::{ast::*, string::parse_string, utils::*};

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
