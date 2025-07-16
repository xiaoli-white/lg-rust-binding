use crate::ir::instruction::IRGoto;

mod instruction;
mod base;
mod types;
mod operand;

pub struct IRModule {}

pub trait IRVisitor {
    fn visit(&self);
    fn visit_module(&self, module: &IRModule);
    fn visit_goto(&self, goto: &IRGoto);
}
