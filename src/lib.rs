use nom::{branch::alt, bytes::complete::tag, IResult};

mod utils;

use utils::*;

#[derive(Debug, PartialEq)]
pub enum Tag {
    Html(Loc),
    Head(Loc),
    Body(Loc),
}

pub fn tag_html(input: Span) -> IResult<Span, Tag> {
    located(tag("html"), |loc, _| Tag::Html(loc))(input)
}
pub fn tag_head(input: Span) -> IResult<Span, Tag> {
    located(tag("head"), |loc, _| Tag::Head(loc))(input)
}
pub fn tag_body(input: Span) -> IResult<Span, Tag> {
    located(tag("body"), |loc, _| Tag::Body(loc))(input)
}
pub fn parse_tag(input: Span) -> IResult<Span, Tag> {
    alt((tag_html, tag_head, tag_body))(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_works() {
        assert_eq!(
            parse_tag("html".into()).unwrap().1,
            Tag::Html(Loc {
                start: Position { line: 1, column: 1 },
                end: Position { line: 1, column: 5 },
            })
        )
    }
}
