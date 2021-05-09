use crate::ast::*;

pub struct Compiler {}

impl Compiler {
    pub fn compile(doc: Document) -> String {
        format!("{}", doc.content)
    }
}
