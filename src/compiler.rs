use crate::ast::*;

pub struct Compiler {}

impl Compiler {
    pub fn compile(program: Program) -> String {
        format!("{}", program.modules)
    }
}
