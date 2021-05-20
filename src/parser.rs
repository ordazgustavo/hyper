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
        alt((
            value(Tag::Html, tag("html")),
            value(Tag::Base, tag("base")),
            value(Tag::Head, tag("head")),
            value(Tag::Link, tag("link")),
            value(Tag::Meta, tag("meta")),
            value(Tag::Style, tag("style")),
            value(Tag::Title, tag("title")),
            value(Tag::Body, tag("body")),
            value(Tag::Address, tag("address")),
            value(Tag::Article, tag("article")),
            value(Tag::Aside, tag("aside")),
        )),
        alt((
            value(Tag::Footer, tag("footer")),
            value(Tag::Header, tag("header")),
            value(Tag::H1, tag("h1")),
            value(Tag::H2, tag("h2")),
            value(Tag::H3, tag("h3")),
            value(Tag::H4, tag("h4")),
            value(Tag::H5, tag("h5")),
            value(Tag::H6, tag("h6")),
            value(Tag::Main, tag("main")),
            value(Tag::Nav, tag("nav")),
            value(Tag::Section, tag("section")),
        )),
        alt((
            value(Tag::Blockquote, tag("blockquote")),
            value(Tag::Dd, tag("dd")),
            value(Tag::Div, tag("div")),
            value(Tag::Dl, tag("dl")),
            value(Tag::Dt, tag("dt")),
            value(Tag::Figcaption, tag("figcaption")),
            value(Tag::Figure, tag("figure")),
            value(Tag::Hr, tag("hr")),
            value(Tag::Li, tag("li")),
            value(Tag::Ol, tag("ol")),
            value(Tag::P, tag("P")),
        )),
        alt((
            value(Tag::Pre, tag("pre")),
            value(Tag::Ul, tag("ul")),
            value(Tag::A, tag("A")),
            value(Tag::Abbr, tag("abbr")),
            value(Tag::B, tag("B")),
            value(Tag::Bdi, tag("bdi")),
            value(Tag::Bdo, tag("bdo")),
            value(Tag::Br, tag("br")),
            value(Tag::Cite, tag("cite")),
            value(Tag::Code, tag("code")),
            value(Tag::Data, tag("data")),
        )),
        alt((
            value(Tag::Dfm, tag("dfm")),
            value(Tag::Em, tag("em")),
            value(Tag::I, tag("I")),
            value(Tag::Kbd, tag("kbd")),
            value(Tag::Mark, tag("mark")),
            value(Tag::Q, tag("Q")),
            value(Tag::Rb, tag("rb")),
            value(Tag::Rp, tag("rp")),
            value(Tag::Rt, tag("rt")),
            value(Tag::Rtc, tag("rtc")),
            value(Tag::Ruby, tag("ruby")),
        )),
        alt((
            value(Tag::S, tag("S")),
            value(Tag::Samp, tag("samp")),
            value(Tag::Small, tag("small")),
            value(Tag::Span, tag("span")),
            value(Tag::Strong, tag("strong")),
            value(Tag::Sub, tag("sub")),
            value(Tag::Sup, tag("sup")),
            value(Tag::Time, tag("time")),
            value(Tag::U, tag("U")),
            value(Tag::Var, tag("var")),
            value(Tag::Wbr, tag("wbr")),
        )),
        alt((
            value(Tag::Area, tag("area")),
            value(Tag::Audio, tag("audio")),
            value(Tag::Img, tag("img")),
            value(Tag::Map, tag("map")),
            value(Tag::Track, tag("track")),
            value(Tag::Video, tag("video")),
            value(Tag::Embed, tag("embed")),
            value(Tag::Iframe, tag("iframe")),
            value(Tag::Object, tag("object")),
            value(Tag::Param, tag("param")),
            value(Tag::Picture, tag("picture")),
        )),
        alt((
            value(Tag::Portal, tag("portal")),
            value(Tag::Source, tag("source")),
            value(Tag::Svg, tag("svg")),
            value(Tag::Math, tag("math")),
            value(Tag::Canvas, tag("canvas")),
            value(Tag::Noscript, tag("noscript")),
            value(Tag::Script, tag("script")),
            value(Tag::Del, tag("del")),
            value(Tag::Ins, tag("ins")),
            value(Tag::Caption, tag("caption")),
            value(Tag::Col, tag("col")),
        )),
        alt((
            value(Tag::Colgroup, tag("colgroup")),
            value(Tag::Table, tag("table")),
            value(Tag::Tbody, tag("tbody")),
            value(Tag::Td, tag("td")),
            value(Tag::Tfoot, tag("tfoot")),
            value(Tag::Th, tag("th")),
            value(Tag::Thead, tag("thead")),
            value(Tag::Tr, tag("tr")),
            value(Tag::Button, tag("button")),
            value(Tag::Datalist, tag("datalist")),
            value(Tag::Fieldset, tag("fieldset")),
        )),
        alt((
            value(Tag::Form, tag("form")),
            value(Tag::Input, tag("input")),
            value(Tag::Label, tag("label")),
            value(Tag::Legend, tag("legend")),
            value(Tag::Meter, tag("meter")),
            value(Tag::Otgroup, tag("otgroup")),
            value(Tag::Option, tag("option")),
            value(Tag::Output, tag("output")),
            value(Tag::Progress, tag("progress")),
            value(Tag::Select, tag("select")),
            value(Tag::Textarea, tag("textarea")),
        )),
        alt((
            value(Tag::Details, tag("details")),
            value(Tag::Dialog, tag("dialog")),
            value(Tag::Menu, tag("menu")),
            value(Tag::Summary, tag("summary")),
            value(Tag::Slot, tag("slot")),
            value(Tag::Template, tag("template")),
        )),
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
        delimited(
            sp,
            tuple((
                parse_tag,
                preceded(sp, opt(parse_attributes)),
                preceded(sp, opt(parse_content)),
            )),
            opt(sp),
        ),
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
