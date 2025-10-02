use crate::ir::IRVisitor;
use crate::ir::base::IRNode;
use crate::ir::types::IRType;
use std::fmt;
use std::fmt::{Display, Formatter};

pub trait IROperand: IRNode {}
pub struct IRVirtualRegister {
    pub name: String,
}

impl IRVirtualRegister {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Display for IRVirtualRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "%{}", self.name)
    }
}

impl IRNode for IRVirtualRegister {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_virtual_register(self);
    }
}

impl IROperand for IRVirtualRegister {}

pub struct IRConstant {
    pub index: i32,
}
impl IRConstant {
    pub fn new(index: i32) -> Self {
        Self { index }
    }
}

impl Display for IRConstant {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "${}", self.index)
    }
}
impl IRNode for IRConstant {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_constant(self);
    }
}
impl IROperand for IRConstant {}

pub struct IRMacro {
    pub name: String,
    pub args: Vec<String>,
    pub additional_operands: Vec<Box<dyn IROperand>>,
}

impl IRMacro {
    pub fn new(
        name: String,
        args: Vec<String>,
        additional_operands: Vec<Box<dyn IROperand>>,
    ) -> Self {
        Self {
            name,
            args,
            additional_operands,
        }
    }
}
impl Display for IRMacro {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`{}([{}], [{}])",
            self.name,
            self.args.join(", "),
            self.additional_operands
                .iter()
                .map(|operand| operand.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
impl IRNode for IRMacro {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_macro(self);
    }
}
impl IROperand for IRMacro {}

pub struct IRPhi {
    pub _type: Box<dyn IRType>,
    pub labels: Vec<String>,
    pub operands: Vec<Box<dyn IROperand>>,
}
impl IRPhi {
    pub fn new(
        _type: Box<dyn IRType>,
        labels: Vec<String>,
        operands: Vec<Box<dyn IROperand>>,
    ) -> Self {
        Self {
            _type,
            labels,
            operands,
        }
    }
}
impl Display for IRPhi {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = format!("phi {} ", self._type.to_string())
            + &self
                .labels
                .iter()
                .zip(self.operands.iter())
                .map(|(label, operand)| format!("[{}, {}]", label, operand))
                .collect::<Vec<String>>()
                .join(", ");
        write!(f, "{}", s)
    }
}

impl IRNode for IRPhi {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_phi(self);
    }
}
impl IROperand for IRPhi {}

pub struct IRVirtualTable {
    pub functions: Vec<String>,
}

impl IRVirtualTable {
    pub fn new(functions: Vec<String>) -> Self {
        Self { functions }
    }
}

impl Display for IRVirtualTable {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "IRVirtualTable{{functions={{{}}}}}",
            self.functions.join(", ")
        )
    }
}
impl IRNode for IRVirtualTable {
    fn accept(&self, visitor: &dyn IRVisitor) {
        todo!()
    }
}
impl IROperand for IRVirtualTable {}
pub struct IRInterfaceTableEntry {
    pub name: String,
    pub functions: Vec<String>,
}
impl IRInterfaceTableEntry {
    pub fn new(name: String, functions: Vec<String>) -> Self {
        Self { name, functions }
    }
}
impl Display for IRInterfaceTableEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Entry[name={}, functions={{{}}}]",
            self.name,
            self.functions.join(", ")
        )
    }
}
pub struct IRInterfaceTable {
    pub entries: Vec<IRInterfaceTableEntry>,
}

impl IRInterfaceTable {
    pub fn new(entries: Vec<IRInterfaceTableEntry>) -> Self {
        Self { entries }
    }
}

impl Display for IRInterfaceTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IRInterfaceTable{{entries={{{}}}}}",
            self.entries
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
impl IRNode for IRInterfaceTable {
    fn accept(&self, visitor: &dyn IRVisitor) {
        todo!()
    }
}
impl IROperand for IRInterfaceTable {}
