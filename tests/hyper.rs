use hyper::prelude::*;

#[cfg(test)]
use pretty_assertions::assert_eq;

fn gen_loc(ls: u32, cs: usize, le: u32, ce: usize) -> Loc {
    Loc {
        start: Position {
            line: ls,
            column: cs,
        },
        end: Position {
            line: le,
            column: ce,
        },
    }
}

#[test]
fn it_parses_simple_module() {
    let program = Parser::parse(
        r#"def Main = [] {
            html {}
        }"#,
    )
    .unwrap();

    assert_eq!(
        program,
        Program {
            modules: Module {
                loc: gen_loc(1, 1, 3, 10),
                statements: vec![Statement::Component(ComponentDef {
                    loc: gen_loc(1, 1, 3, 10),
                    id: Id {
                        loc: gen_loc(1, 5, 1, 9),
                        name: String::from("Main")
                    },
                    attributes: Vec::new(),
                    body: Body {
                        loc: gen_loc(1, 15, 3, 10),
                        children: vec![Child::Element(Element {
                            loc: gen_loc(2, 13, 2, 20),
                            tag: Tag::Html,
                            attributes: None,
                            body: Body {
                                loc: gen_loc(2, 18, 2, 20),
                                children: Vec::new(),
                            },
                        })]
                    }
                })]
            }
        }
    );

    let compiled = Compiler::compile(program);

    assert_eq!(compiled, "<!DOCTYPE html><html></html>");
}

// #[test]
// fn it_parses_element_with_child_text_node() {
//     assert_eq!(
//         Parser::parse(r#"h1 "Page Title""#).unwrap(),
//         Document {
//             loc: gen_loc(1, 1, 16),
//             content: Module {
//                 loc: gen_loc(1, 1, 16),
//                 body: Body {
//                     loc: gen_loc(1, 1, 16),
//                     children: vec![Child::Element(Element {
//                         loc: gen_loc(1, 1, 16),
//                         tag: Tag::H1,
//                         attributes: None,
//                         body: Some(Body {
//                             loc: gen_loc(1, 4, 16),
//                             children: vec![Child::Text(gen_loc(1, 4, 16), "Page Title".to_owned())]
//                         }),
//                     })]
//                 }
//             }
//         }
//     )
// }

// #[test]
// fn it_parses_element_with_attributes() {
//     assert_eq!(
//         Parser::parse(r#"h1 [className="the-title"; data-heading="main"] "Page Title""#).unwrap(),
//         Document {
//             loc: gen_loc(1, 1, 61),
//             content: Module {
//                 loc: gen_loc(1, 1, 61),
//                 body: Body {
//                     loc: gen_loc(1, 1, 61),
//                     children: vec![Child::Element(Element {
//                         loc: gen_loc(1, 1, 61),
//                         tag: Tag::H1,
//                         attributes: Some(Attributes {
//                             loc: gen_loc(1, 4, 48),
//                             attr: vec![
//                                 ("className".to_owned(), "the-title".to_owned()),
//                                 ("data-heading".to_owned(), "main".to_owned())
//                             ]
//                             .into_iter()
//                             .collect()
//                         }),
//                         body: Some(Body {
//                             loc: gen_loc(1, 49, 61),
//                             children: vec![Child::Text(
//                                 gen_loc(1, 49, 61),
//                                 "Page Title".to_owned()
//                             )]
//                         }),
//                     })]
//                 }
//             }
//         }
//     )
// }

// #[test]
// fn it_compiles_source() {
//     let source = r#"
//     html {
//         head {
//             title "Hyper!"
//         }
//     }
//     "#;
//     let document = Parser::parse(source).unwrap();
//     let result = Compiler::compile(document);

//     assert_eq!(
//         result,
//         "<!DOCTYPE html><html><head><title>Hyper!</title></head></html>".to_owned()
//     )
// }

// #[test]
// fn it_compiles_self_closing_tags() {
//     let source = r#"
//     html {
//         head {
//             link [rel="preload"; href="/some/asset.png"]
//             title "Hyper!"
//         }
//     }
//     "#;
//     let document = Parser::parse(source).unwrap();
//     let result = Compiler::compile(document);

//     assert_eq!(
//         result,
//         r#"<!DOCTYPE html><html><head><link rel="preload" href="/some/asset.png"><title>Hyper!</title></head></html>"#
//             .to_owned()
//     )
// }
