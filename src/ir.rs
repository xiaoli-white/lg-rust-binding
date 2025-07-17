use crate::ir::base::IRNode;
use crate::ir::instruction::IRGoto;
use crate::ir::types::IRType;
use std::fmt::Display;

mod base;
mod instruction;
mod operand;
mod structure;
mod types;
pub struct IRConstantPoolEntry {
    pub _type: Box<dyn IRType>,
    pub value: Box<dyn Display>,
}
impl IRConstantPoolEntry {
    pub fn new(_type: Box<dyn IRType>, value: Box<dyn Display>) -> Self {
        IRConstantPoolEntry { _type, value }
    }
}
impl Display for IRConstantPoolEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entry{{type={}, value={}}}", self._type, self.value)
    }
}
impl IRNode for IRConstantPoolEntry {
    fn accept(&self, visitor: &dyn IRVisitor) {
        todo!()
    }
}
pub struct IRConstantPool {
    pub entries: Vec<IRConstantPoolEntry>,
}
impl Display for IRConstantPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .entries
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "IRConstantPool{{entries={{{}}}}}", s)
    }
}
impl IRNode for IRConstantPool {
    fn accept(&self, visitor: &dyn IRVisitor) {
        for entry in &self.entries {
            entry.accept(visitor);
        }
    }
}
pub struct IRModule {}

pub trait IRVisitor {
    fn visit(&self, ir_node: &dyn IRNode)
    where
        Self: Sized,
    {
        ir_node.accept(self)
    }
    fn visit_module(&self, ir_module: &IRModule);
    fn visit_goto(&self, ir_goto: &IRGoto);
}
