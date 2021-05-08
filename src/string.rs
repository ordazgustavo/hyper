// Taken from nom examples and adapted to use `nom_locate`'s `LocatedSpan`
// https://github.com/Geal/nom/blob/master/examples/string.rs

use nom::{
    branch::alt,
    bytes::streaming::{is_not, take_while_m_n},
    character::streaming::{char, multispace1},
    combinator::{map, map_opt, map_res, value, verify},
    error::{FromExternalError, ParseError},
    multi::fold_many0,
    sequence::{delimited, preceded},
    IResult,
};

use crate::utils::Span;

fn parse_unicode<'a, E>(input: Span<'a>) -> IResult<Span<'a>, char, E>
where
    E: ParseError<Span<'a>> + FromExternalError<Span<'a>, std::num::ParseIntError>,
{
    let parse_hex = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit());

    let parse_delimited_hex = preceded(char('u'), delimited(char('{'), parse_hex, char('}')));

    let parse_u32 = map_res(parse_delimited_hex, move |hex: Span| {
        u32::from_str_radix(hex.fragment(), 16)
    });

    map_opt(parse_u32, |value| std::char::from_u32(value))(input)
}

fn parse_escaped_char<'a, E>(input: Span<'a>) -> IResult<Span<'a>, char, E>
where
    E: ParseError<Span<'a>> + FromExternalError<Span<'a>, std::num::ParseIntError>,
{
    preceded(
        char('\\'),
        alt((
            parse_unicode,
            value('\n', char('n')),
            value('\r', char('r')),
            value('\t', char('t')),
            value('\u{08}', char('b')),
            value('\u{0C}', char('f')),
            value('\\', char('\\')),
            value('/', char('/')),
            value('"', char('"')),
        )),
    )(input)
}

fn parse_escaped_whitespace<'a, E: ParseError<Span<'a>>>(
    input: Span<'a>,
) -> IResult<Span<'a>, Span<'a>, E> {
    preceded(char('\\'), multispace1)(input)
}

fn parse_literal<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    let not_quote_slash = is_not("\"\\");

    verify(not_quote_slash, |s: &Span| !s.fragment().is_empty())(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(Span<'a>),
    EscapedChar(char),
    EscapedWS,
}

fn parse_fragment<'a, E>(input: Span<'a>) -> IResult<Span<'a>, StringFragment<'a>, E>
where
    E: ParseError<Span<'a>> + FromExternalError<Span<'a>, std::num::ParseIntError>,
{
    alt((
        map(parse_literal, StringFragment::Literal),
        map(parse_escaped_char, StringFragment::EscapedChar),
        value(StringFragment::EscapedWS, parse_escaped_whitespace),
    ))(input)
}

pub fn parse_string<'a, E>(input: Span<'a>) -> IResult<Span<'a>, String, E>
where
    E: ParseError<Span<'a>> + FromExternalError<Span<'a>, std::num::ParseIntError>,
{
    let build_string = fold_many0(parse_fragment, String::new(), |mut string, fragment| {
        match fragment {
            StringFragment::Literal(s) => string.push_str(s.fragment()),
            StringFragment::EscapedChar(c) => string.push(c),
            StringFragment::EscapedWS => {}
        }
        string
    });

    delimited(char('"'), build_string, char('"'))(input)
}
