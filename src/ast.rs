use std::collections::HashMap;
use std::fmt;

use crate::utils::Loc;

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

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let attr = if let Some(attr) = &self.attributes {
            format!(" {}", attr)
        } else {
            String::new()
        };

        let opening_tag = if self.tag == Tag::Html {
            format!("<!DOCTYPE html><{}{}>", self.tag, attr)
        } else {
            format!("<{}{}>", self.tag, attr)
        };

        let content = if let Some(content) = &self.content {
            format!("{}", content)
        } else {
            String::new()
        };

        let closing_tag = if !self.tag.is_self_closing() {
            format!("</{}>", self.tag)
        } else {
            String::new()
        };

        write!(f, "{}{}{}", opening_tag, content, closing_tag)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tag {
    Html,
    Head,
    Title,
    Body,
    H1,
}

impl Tag {
    pub fn is_self_closing(&self) -> bool {
        match self {
            Tag::Html => false,
            Tag::Head => false,
            Tag::Title => false,
            Tag::Body => false,
            Tag::H1 => false,
        }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tag::Html => write!(f, "html"),
            Tag::Head => write!(f, "head"),
            Tag::Title => write!(f, "title"),
            Tag::Body => write!(f, "body"),
            Tag::H1 => write!(f, "h1"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Attributes {
    pub loc: Loc,
    pub attr: HashMap<String, String>,
}

impl fmt::Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.attr
                .iter()
                .map(|(k, v)| format!("{}=\"{}\"", k, v))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Content {
    pub loc: Loc,
    pub children: Vec<Child>,
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.children
                .iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum Child {
    Text(Loc, String),
    Element(Element),
}

impl fmt::Display for Child {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Child::Text(_, text) => write!(f, "{}", text),
            Child::Element(element) => write!(f, "{}", element),
        }
    }
}
