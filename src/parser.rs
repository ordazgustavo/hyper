use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{alpha1, alphanumeric1},
        streaming::char,
    },
    combinator::{map, opt, recognize, value},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};
use std::collections::HashMap;

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

fn id(input: Span) -> IResult<Span, Span> {
    recognize(pair(
        alt((alpha1, tag("-"))),
        many0(alt((alphanumeric1, tag("-")))),
    ))(input)
}

fn key_value(input: Span) -> IResult<Span, (Span, String)> {
    separated_pair(id, preceded(sp, char('=')), preceded(sp, parse_string))(input)
}

fn parse_attr(input: Span) -> IResult<Span, HashMap<String, String>> {
    map(
        separated_list1(preceded(sp, char(';')), preceded(sp, key_value)),
        |tuple_vec| {
            tuple_vec
                .into_iter()
                .map(|(k, v)| (String::from(*k.fragment()), v))
                .collect()
        },
    )(input)
}

fn parse_attributes(input: Span) -> IResult<Span, Attributes> {
    located(delimited(char('['), parse_attr, char(']')), |loc, attr| {
        Attributes { loc, attr }
    })(input)
}

fn parse_element(input: Span) -> IResult<Span, Element> {
    located(
        tuple((
            parse_tag,
            preceded(sp, opt(parse_attributes)),
            preceded(sp, opt(parse_content)),
        )),
        |loc, (tag, attributes, content)| Element {
            loc,
            tag,
            attributes,
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
