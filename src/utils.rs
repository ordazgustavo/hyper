use nom::{bytes::complete::take_while, error::ParseError, IResult, Parser};
use nom_locate::{position, LocatedSpan};

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq, Clone)]
pub struct Loc {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub line: u32,
    pub column: usize,
}

impl From<nom_locate::LocatedSpan<&str>> for Position {
    fn from(span: nom_locate::LocatedSpan<&str>) -> Self {
        Self {
            line: span.location_line(),
            column: span.get_utf8_column(),
        }
    }
}

pub fn sp<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    take_while(|c| " \t\r\n".contains(c))(input)
}

pub fn located<'a, O1, O2, E, F, G>(
    mut first: F,
    mut second: G,
) -> impl FnMut(Span<'a>) -> IResult<Span, O2, E>
where
    E: ParseError<Span<'a>>,
    F: Parser<Span<'a>, O1, E>,
    G: FnMut(Loc, O1) -> O2,
{
    move |input: Span<'a>| {
        let (input, start) = position(input)?;
        let (input, o1) = first.parse(input)?;
        let (input, end) = position(input)?;
        Ok((
            input,
            second(
                Loc {
                    start: start.into(),
                    end: end.into(),
                },
                o1,
            ),
        ))
    }
}
