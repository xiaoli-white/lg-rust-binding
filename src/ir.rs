use crate::ir::base::IRNode;
use crate::ir::instruction::IRGoto;

mod base;
mod instruction;
mod operand;
mod structure;
mod types;

pub struct IRModule {}

pub trait IRVisitor {
    fn visit(&self, ir_node: &dyn IRNode) where Self: Sized {
        ir_node.accept(self)
    }
    fn visit_module(&self, ir_module: &IRModule);
    fn visit_goto(&self, ir_goto: &IRGoto);
}
