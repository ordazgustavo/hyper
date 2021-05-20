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
fn it_parses_element_with_child_text_node() {
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

#[test]
fn it_parses_element_with_attributes() {
    assert_eq!(
        Parser::parse(r#"h1 [className="the-title"; data-heading="main"] "Page Title""#).unwrap(),
        Document {
            loc: gen_loc(1, 1, 61),
            content: Element {
                loc: gen_loc(1, 1, 61),
                tag: Tag::H1,
                attributes: Some(Attributes {
                    loc: gen_loc(1, 4, 48),
                    attr: vec![
                        ("className".to_owned(), "the-title".to_owned()),
                        ("data-heading".to_owned(), "main".to_owned())
                    ]
                    .into_iter()
                    .collect()
                }),
                content: Some(Content {
                    loc: gen_loc(1, 49, 61),
                    children: vec![Child::Text(gen_loc(1, 49, 61), "Page Title".to_owned())]
                }),
            }
        }
    )
}

#[test]
fn it_compiles_source() {
    let source = r#"
    html {
        head {
            title "Hyper!"
        }
    }
    "#;
    let document = Parser::parse(source).unwrap();
    let result = Compiler::compile(document);

    assert_eq!(
        result,
        "<!DOCTYPE html><html><head><title>Hyper!</title></head></html>".to_owned()
    )
}

#[test]
fn it_compiles_self_closing_tags() {
    let source = r#"
    html {
        head {
            link [rel="preload"; href="/some/asset.png"]
            title "Hyper!"
        }
    }
    "#;
    let document = Parser::parse(source).unwrap();
    let result = Compiler::compile(document);

    assert_eq!(
        result,
        r#"<!DOCTYPE html><html><head><link rel="preload" href="/some/asset.png"><title>Hyper!</title></head></html>"#
            .to_owned()
    )
}
