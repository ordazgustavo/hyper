use hyper::prelude::*;

#[cfg(test)]
use pretty_assertions::assert_eq;

fn gen_loc(l: u32, cs: usize, ce: usize) -> Loc {
    Loc {
        start: Position {
            line: l,
            column: cs,
        },
        end: Position {
            line: l,
            column: ce,
        },
    }
}

#[test]
fn it_works() {
    assert_eq!(
        Parser::parse("html {}").unwrap(),
        Document {
            loc: gen_loc(1, 1, 6),
            content: Element {
                loc: gen_loc(1, 1, 6),
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
            loc: gen_loc(1, 1, 16),
            content: Element {
                loc: gen_loc(1, 1, 16),
                tag: Tag::H1,
                attributes: None,
                content: Some(Content {
                    loc: gen_loc(1, 4, 16),
                    children: vec![Child::Text(gen_loc(1, 4, 16), "Page Title".to_owned())]
                }),
            }
        }
    )
}
