use crate::ir::IRVisitor;
use crate::ir::base::IRNode;
use crate::ir::types::IRType;
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct IRField {
    pub name: String,
    pub _type: Box<dyn IRType>,
}

impl IRField {
    pub fn new(name: String, _type: Box<dyn IRType>) -> Self {
        Self { name, _type }
    }
}
impl Display for IRField {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "IRField{{name={}, type={}}}", self.name, self._type)
    }
}

impl IRNode for IRField {
    fn accept(&self, visitor: &dyn IRVisitor) {
        todo!()
    }
}

pub struct IRStructure {
    pub name: String,
    pub fields: Vec<IRField>,
}

impl IRStructure {
    fn new(name: String, fields: Vec<IRField>) -> Self {
        Self { name, fields }
    }
}
impl Display for IRStructure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IRStructure{{name={}, fields={{{}}}}}",
            self.name,
            self.fields
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
impl IRNode for IRStructure {
    fn accept(&self, visitor: &dyn IRVisitor) {
        todo!()
    }
}