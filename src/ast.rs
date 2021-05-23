use std::collections::HashMap;
use std::fmt;

use crate::utils::Loc;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub modules: Module,
}

#[derive(Debug, PartialEq)]
pub struct Module {
    pub loc: Loc,
    pub statements: Vec<Statement>,
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.statements
                .iter()
                .map(|s| format!("{}", s))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Import,
    Component(ComponentDef),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Import => write!(f, ""),
            Statement::Component(e) => write!(f, "{}", e.body),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ComponentDef {
    pub loc: Loc,
    pub id: Id,
    pub attributes: Vec<Id>,
    pub body: Body,
}

impl fmt::Display for ComponentDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.body)
    }
}

#[derive(Debug, PartialEq)]
pub struct Id {
    pub loc: Loc,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct Body {
    pub loc: Loc,
    pub children: Vec<Child>,
}

impl fmt::Display for Body {
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
    Text(TextNode),
    Element(Element),
    Component(ComponentExpr),
}

impl fmt::Display for Child {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Child::Text(text) => write!(f, "{}", text.value),
            Child::Element(element) => write!(f, "{}", element),
            Child::Component(component) => {
                write!(f, "{}", component)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub loc: Loc,
    pub tag: Tag,
    pub attributes: Option<Attributes>,
    pub body: Body,
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

        let content = format!("{}", &self.body);

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
    // Main root
    Html,

    // Document metadata
    Base,
    Head,
    Link,
    Meta,
    Style,
    Title,

    // Sectioning root
    Body,

    // Content sectioning
    Address,
    Article,
    Aside,
    Footer,
    Header,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Main,
    Nav,
    Section,

    // Text content
    Blockquote,
    Dd,
    Div,
    Dl,
    Dt,
    Figcaption,
    Figure,
    Hr,
    Li,
    Ol,
    P,
    Pre,
    Ul,

    // Inline text semantics
    A,
    Abbr,
    B,
    Bdi,
    Bdo,
    Br,
    Cite,
    Code,
    Data,
    Dfm,
    Em,
    I,
    Kbd,
    Mark,
    Q,
    Rb,
    Rp,
    Rt,
    Rtc,
    Ruby,
    S,
    Samp,
    Small,
    Span,
    Strong,
    Sub,
    Sup,
    Time,
    U,
    Var,
    Wbr,

    // Image and multimedia
    Area,
    Audio,
    Img,
    Map,
    Track,
    Video,

    // Embeded content
    Embed,
    Iframe,
    Object,
    Param,
    Picture,
    Portal,
    Source,

    // SVG and MathML
    Svg,
    Math,

    // Scripting
    Canvas,
    Noscript,
    Script,

    // Demarcating edits
    Del,
    Ins,

    // Table content
    Caption,
    Col,
    Colgroup,
    Table,
    Tbody,
    Td,
    Tfoot,
    Th,
    Thead,
    Tr,

    // Forms
    Button,
    Datalist,
    Fieldset,
    Form,
    Input,
    Label,
    Legend,
    Meter,
    Otgroup,
    Option,
    Output,
    Progress,
    Select,
    Textarea,

    // Interactive elements
    Details,
    Dialog,
    Menu,
    Summary,

    // Web Components
    Slot,
    Template,
}

impl Tag {
    pub fn is_self_closing(&self) -> bool {
        match self {
            Tag::Area => true,
            Tag::Base => true,
            Tag::Br => true,
            Tag::Col => true,
            Tag::Embed => true,
            Tag::Hr => true,
            Tag::Img => true,
            Tag::Input => true,
            Tag::Link => true,
            Tag::Meta => true,
            Tag::Param => true,
            Tag::Source => true,
            Tag::Track => true,
            Tag::Wbr => true,
            _ => false,
        }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tag::Link => write!(f, "link"),
            Tag::Meta => write!(f, "meta"),
            Tag::Style => write!(f, "style"),
            Tag::Html => write!(f, "html"),
            Tag::Head => write!(f, "head"),
            Tag::Title => write!(f, "title"),
            Tag::Body => write!(f, "body"),
            Tag::Base => write!(f, "base"),
            Tag::Address => write!(f, "address"),
            Tag::Article => write!(f, "article"),
            Tag::Aside => write!(f, "aside"),
            Tag::Footer => write!(f, "footer"),
            Tag::Header => write!(f, "header"),
            Tag::H1 => write!(f, "h1"),
            Tag::H2 => write!(f, "h2"),
            Tag::H3 => write!(f, "h3"),
            Tag::H4 => write!(f, "h4"),
            Tag::H5 => write!(f, "h5"),
            Tag::H6 => write!(f, "h6"),
            Tag::Main => write!(f, "main"),
            Tag::Nav => write!(f, "nav"),
            Tag::Section => write!(f, "section"),
            Tag::Blockquote => write!(f, "blockquote"),
            Tag::Dd => write!(f, "dd"),
            Tag::Div => write!(f, "div"),
            Tag::Dl => write!(f, "dl"),
            Tag::Dt => write!(f, "dt"),
            Tag::Figcaption => write!(f, "figcaption"),
            Tag::Figure => write!(f, "figure"),
            Tag::Hr => write!(f, "hr"),
            Tag::Li => write!(f, "li"),
            Tag::Ol => write!(f, "ol"),
            Tag::P => write!(f, "p"),
            Tag::Pre => write!(f, "pre"),
            Tag::Ul => write!(f, "ul"),
            Tag::A => write!(f, "q"),
            Tag::Abbr => write!(f, "abbr"),
            Tag::B => write!(f, "B"),
            Tag::Bdi => write!(f, "bdi"),
            Tag::Bdo => write!(f, "bdo"),
            Tag::Br => write!(f, "br"),
            Tag::Cite => write!(f, "cite"),
            Tag::Code => write!(f, "code"),
            Tag::Data => write!(f, "data"),
            Tag::Dfm => write!(f, "dfm"),
            Tag::Em => write!(f, "em"),
            Tag::I => write!(f, "I"),
            Tag::Kbd => write!(f, "kbd"),
            Tag::Mark => write!(f, "mark"),
            Tag::Q => write!(f, "q"),
            Tag::Rb => write!(f, "rb"),
            Tag::Rp => write!(f, "rp"),
            Tag::Rt => write!(f, "rt"),
            Tag::Rtc => write!(f, "rtc"),
            Tag::Ruby => write!(f, "ruby"),
            Tag::S => write!(f, "S"),
            Tag::Samp => write!(f, "samp"),
            Tag::Small => write!(f, "small"),
            Tag::Span => write!(f, "span"),
            Tag::Strong => write!(f, "strong"),
            Tag::Sub => write!(f, "sub"),
            Tag::Sup => write!(f, "sup"),
            Tag::Time => write!(f, "time"),
            Tag::U => write!(f, "U"),
            Tag::Var => write!(f, "var"),
            Tag::Wbr => write!(f, "wbr"),
            Tag::Area => write!(f, "area"),
            Tag::Audio => write!(f, "audio"),
            Tag::Img => write!(f, "img"),
            Tag::Map => write!(f, "map"),
            Tag::Track => write!(f, "track"),
            Tag::Video => write!(f, "video"),
            Tag::Embed => write!(f, "embed"),
            Tag::Iframe => write!(f, "iframe"),
            Tag::Object => write!(f, "object"),
            Tag::Param => write!(f, "param"),
            Tag::Picture => write!(f, "picture"),
            Tag::Portal => write!(f, "portal"),
            Tag::Source => write!(f, "source"),
            Tag::Svg => write!(f, "svg"),
            Tag::Math => write!(f, "math"),
            Tag::Canvas => write!(f, "canvas"),
            Tag::Noscript => write!(f, "noscript"),
            Tag::Script => write!(f, "script"),
            Tag::Del => write!(f, "del"),
            Tag::Ins => write!(f, "ins"),
            Tag::Caption => write!(f, "caption"),
            Tag::Col => write!(f, "col"),
            Tag::Colgroup => write!(f, "colgroup"),
            Tag::Table => write!(f, "table"),
            Tag::Tbody => write!(f, "tbody"),
            Tag::Td => write!(f, "td"),
            Tag::Tfoot => write!(f, "tfoot"),
            Tag::Th => write!(f, "th"),
            Tag::Thead => write!(f, "thead"),
            Tag::Tr => write!(f, "tr"),
            Tag::Button => write!(f, "button"),
            Tag::Datalist => write!(f, "datalist"),
            Tag::Fieldset => write!(f, "fieldset"),
            Tag::Form => write!(f, "form"),
            Tag::Input => write!(f, "input"),
            Tag::Label => write!(f, "label"),
            Tag::Legend => write!(f, "legend"),
            Tag::Meter => write!(f, "meter"),
            Tag::Otgroup => write!(f, "otgroup"),
            Tag::Option => write!(f, "option"),
            Tag::Output => write!(f, "output"),
            Tag::Progress => write!(f, "progress"),
            Tag::Select => write!(f, "select"),
            Tag::Textarea => write!(f, "textarea"),
            Tag::Details => write!(f, "details"),
            Tag::Dialog => write!(f, "dialog"),
            Tag::Menu => write!(f, "menu"),
            Tag::Summary => write!(f, "summary"),
            Tag::Slot => write!(f, "slot"),
            Tag::Template => write!(f, "template"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ComponentExpr {
    pub loc: Loc,
    pub id: Id,
    pub attributes: Option<Attributes>,
    pub body: Option<Body>,
}

impl fmt::Display for ComponentExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.body
                .as_ref()
                .map_or_else(|| String::default(), |b| format!("{}", b))
        )
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
pub struct TextNode {
    pub loc: Loc,
    pub value: String,
}
