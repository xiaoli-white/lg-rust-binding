use crate::ir::IRVisitor;
use crate::ir::instruction::IRInstruction;
use crate::ir::operand::IROperand;
use crate::ir::structure::IRField;
use crate::ir::types::IRType;
use indexmap::IndexMap;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Debug;
use dyn_clone::{clone_trait_object, DynClone};

pub trait IRNode: fmt::Display+DynClone+Debug {
    fn accept(&self, visitor: &dyn IRVisitor);
}
clone_trait_object!(IRNode);
#[derive(Clone, Debug)]
pub enum IRCondition {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    IfTrue,
    IfFalse,
}
impl fmt::Display for IRCondition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            IRCondition::Equal => "e".to_string(),
            IRCondition::NotEqual => "ne".to_string(),
            IRCondition::Less => "l".to_string(),
            IRCondition::LessEqual => "le".to_string(),
            IRCondition::Greater => "g".to_string(),
            IRCondition::GreaterEqual => "ge".to_string(),
            IRCondition::IfTrue => "if_true".to_string(),
            IRCondition::IfFalse => "if_false".to_string(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug)]
pub struct IRGlobalData {
    pub name: String,
    pub size: Option<Box<dyn IROperand>>,
    pub values: Option<Vec<Box<dyn IROperand>>>,
}
impl IRGlobalData {
    pub fn new(
        name: String,
        size: Option<Box<dyn IROperand>>,
        values: Option<Vec<Box<dyn IROperand>>>,
    ) -> Self {
        IRGlobalData { name, size, values }
    }
}
impl fmt::Display for IRGlobalData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(size) = &self.size {
            write!(f, ", size={}", size)?;
        }
        if let Some(values) = &self.values {
            write!(
                f,
                ", values=[{}]",
                values
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        Ok(())
    }
}
impl IRNode for IRGlobalData {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_global_data(self);
    }
}
#[derive(Clone, Debug)]
pub struct IRGlobalDataSection {
    pub data: Vec<IRGlobalData>,
}
impl IRGlobalDataSection {
    pub fn new() -> Self {
        Self { data: vec![] }
    }
}
impl fmt::Display for IRGlobalDataSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IRGlobalDataSection{{data=[{}]}}",
            self.data
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
impl IRNode for IRGlobalDataSection {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_global_data_section(self);
    }
}
#[derive(Clone, Debug)]
pub struct IRBasicBlock {
    pub name: String,
    pub instructions: Vec<Box<dyn IRInstruction>>,
}
impl IRBasicBlock {
    pub fn new(name: String) -> Self {
        Self {
            name,
            instructions: vec![],
        }
    }
}
impl fmt::Display for IRBasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IRBasicBlock[name={}, instructions=[{}]]",
            self.name,
            self.instructions
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
impl IRNode for IRBasicBlock {
    fn accept(&self, visitor: &dyn IRVisitor) {
        for instruction in &self.instructions {
            instruction.accept(visitor);
        }
    }
}
#[derive(Clone, Debug)]
pub struct IRControlFlowGraph {
    pub basic_blocks: IndexMap<String, Box<IRBasicBlock>>,
    pub out_edges: BTreeMap<Box<IRBasicBlock>, Vec<Box<IRBasicBlock>>>,
    pub in_edges: BTreeMap<Box<IRBasicBlock>, Vec<Box<IRBasicBlock>>>,
}
impl IRControlFlowGraph {
    pub fn new() -> Self {
        let basic_blocks = IndexMap::new();
        let out_edges = BTreeMap::new();
        let in_edges = BTreeMap::new();
        Self {
            basic_blocks,
            out_edges,
            in_edges,
        }
    }
    pub fn add_basic_block(&mut self, basic_block: Box<IRBasicBlock>) {
        self.basic_blocks
            .insert(basic_block.name.clone(), basic_block);
    }
}
impl fmt::Display for IRControlFlowGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IRControlFlowGraph{{basicBlocks=[{}], outEdges=[], inEdges=[]}}",
            self.basic_blocks
                .values()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
#[derive(Clone, Debug)]
pub struct IRFunction {
    pub return_type: Box<dyn IRType>,
    pub name: String,
    pub arguments_count: usize,
    pub fields: Vec<Box<IRField>>,
    pub control_flow_graph: Box<IRControlFlowGraph>,
}
impl IRFunction {
    pub fn new(
        return_type: Box<dyn IRType>,
        name: String,
        arguments_count: usize,
        fields: Vec<Box<IRField>>,
        control_flow_graph: Box<IRControlFlowGraph>,
    ) -> Self {
        IRFunction {
            return_type,
            name,
            arguments_count,
            fields,
            control_flow_graph,
        }
    }
}
impl fmt::Display for IRFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IRFunction{{returnType={}, name='{}', argumentsCount={}, fields=[{}], controlFlowGraph={}}}",
            self.return_type,
            self.name,
            self.arguments_count,
            self.fields
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.control_flow_graph
        )
    }
}
impl IRNode for IRFunction {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_function(self);
    }
}
