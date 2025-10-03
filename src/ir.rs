use crate::ir::base::{IRControlFlowGraph, IRFunction, IRGlobalData, IRGlobalDataSection, IRNode};
use crate::ir::instruction::{
    IRAsm, IRCalculate, IRConditionalJump, IRDecrease, IRFree, IRGet, IRGoto, IRIncrease, IRInvoke,
    IRMalloc, IRNegate, IRNoOperate, IRNot, IRRealloc, IRReturn, IRSet, IRSetVirtualRegister,
    IRStackAllocate, IRTypeCast,
};
use crate::ir::operand::{IRConstant, IRMacro, IRPhi, IRVirtualRegister};
use crate::ir::structure::{IRField, IRStructure};
use crate::ir::types::{
    IRDoubleType, IRFloatType, IRIntegerType, IRPointerType, IRType, IRVoidType,
};
use indexmap::IndexMap;
use std::fmt::{Debug, Display};

pub mod base;
pub mod instruction;
pub mod operand;
pub mod structure;
pub mod types;
pub struct IRConstantPoolEntry {
    pub _type: Box<dyn IRType>,
    pub value: Box<dyn Display>,
}

impl IRConstantPoolEntry {
    pub fn new(_type: Box<dyn IRType>, value: Box<dyn Display>) -> Self {
        Self { _type, value }
    }
}

impl Display for IRConstantPoolEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entry{{type={}, value={}}}", self._type, self.value)
    }
}

impl Debug for IRConstantPoolEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("IRConstantPoolEntry");

        debug_struct.field("type", &self._type);
        debug_struct.field("value", &format!("{}", self.value));
        
        debug_struct.finish()
    }
}

impl IRNode for IRConstantPoolEntry {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_constant_pool_entry(self)
    }
}
#[derive(Debug)]
pub struct IRConstantPool {
    pub entries: Vec<Box<IRConstantPoolEntry>>,
}
impl IRConstantPool {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn push(&mut self, entry: Box<IRConstantPoolEntry>) -> usize {
        self.entries.push(entry);

        self.entries.len() - 1
    } 
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
        visitor.visit_constant_pool(self)
    }
}

#[derive(Debug)]
pub struct IRModule {
    pub structures: IndexMap<String, Box<IRStructure>>,
    pub constant_pool: Box<IRConstantPool>,
    pub global_data_section: Box<IRGlobalDataSection>,
    pub global_init_section: Box<IRControlFlowGraph>,
    pub functions: IndexMap<String, Box<IRFunction>>,
    pub name2vtable_keys: IndexMap<String, Vec<String>>,
    pub name2itable_keys: IndexMap<String, Vec<String>>,
    pub entry_point: Option<String>,
}

impl IRModule {
    pub fn new() -> Self {
        Self {
            structures: IndexMap::new(),
            constant_pool: Box::new(IRConstantPool { entries: vec![] }),
            global_data_section: Box::new(IRGlobalDataSection::new()),
            global_init_section: Box::new(IRControlFlowGraph::new()),
            functions: IndexMap::new(),
            name2vtable_keys: IndexMap::new(),
            name2itable_keys: IndexMap::new(),
            entry_point: None,
        }
    }

    #[inline]
    pub fn push_function(&mut self, func: IRFunction) {
        self.functions.insert(func.name.clone(), Box::new(func));
    }

    #[inline]
    pub fn push_struct(&mut self, structure: IRStructure) {
        self.structures.insert(structure.name.clone(), Box::new(structure));
    }
}

impl Display for IRModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .structures
            .values()
            .map(|structure| structure.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(
            f,
            "IRModule{{structures={{{}}}, constant_pool={}, global_data_section={}, global_init_section={}, functions0={{{}}}, name2vtable_keys={{{}}}, name2itable_keys={{{}}}, entry_point={}}}",
            s,
            self.constant_pool,
            self.global_data_section,
            self.global_init_section,
            self.functions
                .iter()
                .map(|(name, function)| format!("{}={}", name, function))
                .collect::<Vec<_>>()
                .join(", "),
            self.name2vtable_keys
                .iter()
                .map(|(name, keys)| format!("{}={}", name, keys.join(", ")))
                .collect::<Vec<_>>()
                .join(", "),
            self.name2itable_keys
                .iter()
                .map(|(name, keys)| format!("{}={}", name, keys.join(", ")))
                .collect::<Vec<_>>()
                .join(", "),
            self.entry_point
                .as_ref()
                .map(|entry| entry.to_string())
                .unwrap_or("null".to_string())
        )
    }
}
impl IRNode for IRModule {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_module(self)
    }
}
pub trait IRVisitor {
    fn visit_dyn(&self, ir_node: &dyn IRNode);
    fn visit(&self, ir_node: &dyn IRNode)
    where
        Self: Sized,
    {
        ir_node.accept(self);
    }
    fn visit_module(&self, ir_module: &IRModule) {
        for ir_structure in ir_module.structures.values() {
            self.visit_structure(ir_structure);
        }
        self.visit_constant_pool(&ir_module.constant_pool);
        self.visit_global_data_section(&ir_module.global_data_section);
        for ir_basic_block in ir_module.global_init_section.basic_blocks.values() {
            for ir_instruction in ir_basic_block.instructions.iter() {
                self.visit_dyn(ir_instruction.as_ref());
            }
        }
        for ir_function in ir_module.functions.values() {
            self.visit_function(ir_function);
        }
    }
    fn visit_constant_pool(&self, ir_constant_pool: &IRConstantPool) {
        for entry in ir_constant_pool.entries.iter() {
            self.visit_dyn(entry.as_ref());
        }
    }
    fn visit_constant_pool_entry(&self, ir_constant_pool_entry: &IRConstantPoolEntry) {
        self.visit_dyn(ir_constant_pool_entry._type.as_ref())
    }
    fn visit_function(&self, ir_function: &IRFunction) {
        self.visit_dyn(ir_function.return_type.as_ref());
        for ir_field in ir_function.fields.iter() {
            self.visit_field(ir_field);
        }
        for ir_basic_block in ir_function.control_flow_graph.basic_blocks.values() {
            for ir_instruction in ir_basic_block.instructions.iter() {
                self.visit_dyn(ir_instruction.as_ref());
            }
        }
    }
    fn visit_structure(&self, ir_structure: &IRStructure) {
        for ir_field in ir_structure.fields.iter() {
            self.visit_field(ir_field);
        }
    }
    fn visit_field(&self, ir_field: &IRField) {
        self.visit_dyn(ir_field._type.as_ref());
    }
    fn visit_global_data_section(&self, ir_global_data_section: &IRGlobalDataSection) {
        for ir_global_data in ir_global_data_section.data.iter() {
            self.visit_global_data(ir_global_data);
        }
    }
    fn visit_global_data(&self, ir_global_data: &IRGlobalData) {
        if let Some(size) = ir_global_data.size.as_ref() {
            self.visit_dyn(size.as_ref());
        }
        if let Some(values) = ir_global_data.values.as_ref() {
            for value in values {
                self.visit_dyn(value.as_ref());
            }
        }
    }
    fn visit_integer_type(&self, ir_integer_type: &IRIntegerType) {}
    fn visit_float_type(&self, ir_float_type: &IRFloatType) {}
    fn visit_double_type(&self, ir_double_type: &IRDoubleType) {}
    fn visit_pointer_type(&self, ir_pointer_type: &IRPointerType) {
        self.visit_dyn(ir_pointer_type.base.as_ref());
    }
    fn visit_void_type(&self, ir_void_type: &IRVoidType) {}
    fn visit_goto(&self, ir_goto: &IRGoto) {}
    fn visit_conditional_jump(&self, ir_conditional_jump: &IRConditionalJump) {
        self.visit_dyn(ir_conditional_jump._type.as_ref());
        self.visit_dyn(ir_conditional_jump.operand1.as_ref());
        if let Some(operand2) = ir_conditional_jump.operand2.as_ref() {
            self.visit_dyn(operand2.as_ref());
        }
    }
    fn visit_return(&self, ir_return: &IRReturn) {
        if let Some(operand) = ir_return.operand.as_ref() {
            self.visit_dyn(operand.as_ref());
        }
    }
    fn visit_calculate(&self, ir_calculate: &IRCalculate) {
        self.visit_dyn(ir_calculate._type.as_ref());
        self.visit_dyn(ir_calculate.operand1.as_ref());
        self.visit_dyn(ir_calculate.operand2.as_ref());
        self.visit_virtual_register(ir_calculate.target.as_ref());
    }
    fn visit_not(&self, ir_not: &IRNot) {
        self.visit_dyn(ir_not._type.as_ref());
        self.visit_dyn(ir_not.operand.as_ref());
        self.visit_virtual_register(ir_not.target.as_ref());
    }
    fn visit_negate(&self, ir_negate: &IRNegate) {
        self.visit_dyn(ir_negate._type.as_ref());
        self.visit_dyn(ir_negate.operand.as_ref());
        self.visit_virtual_register(ir_negate.target.as_ref());
    }
    fn visit_malloc(&self, ir_malloc: &IRMalloc) {
        self.visit_dyn(ir_malloc.size.as_ref());
        self.visit_virtual_register(ir_malloc.target.as_ref());
    }
    fn visit_free(&self, ir_free: &IRFree) {
        self.visit_dyn(ir_free.ptr.as_ref());
    }
    fn visit_realloc(&self, ir_realloc: &IRRealloc) {
        self.visit_dyn(ir_realloc.ptr.as_ref());
        self.visit_dyn(ir_realloc.size.as_ref());
        self.visit_virtual_register(ir_realloc.target.as_ref());
    }
    fn visit_get(&self, ir_get: &IRGet) {
        self.visit_dyn(ir_get._type.as_ref());
        self.visit_dyn(ir_get.address.as_ref());
        self.visit_virtual_register(ir_get.target.as_ref());
    }
    fn visit_set(&self, ir_set: &IRSet) {
        self.visit_dyn(ir_set._type.as_ref());
        self.visit_dyn(ir_set.address.as_ref());
        self.visit_dyn(ir_set.value.as_ref());
    }
    fn visit_set_virtual_register(&self, ir_set_virtual_register: &IRSetVirtualRegister) {
        self.visit_dyn(ir_set_virtual_register.source.as_ref());
        self.visit_virtual_register(ir_set_virtual_register.target.as_ref());
    }
    fn visit_invoke(&self, ir_invoke: &IRInvoke) {
        self.visit_dyn(ir_invoke.address.as_ref());
        for (argument_type, argument) in ir_invoke
            .argument_types
            .iter()
            .zip(ir_invoke.arguments.iter())
        {
            self.visit_dyn(argument_type.as_ref());
            self.visit_dyn(argument.as_ref());
        }
        self.visit_dyn(ir_invoke.return_type.as_ref());
        if let Some(target) = &ir_invoke.target {
            self.visit_dyn(target.as_ref());
        }
    }
    fn visit_no_operate(&self, ir_no_operate: &IRNoOperate) {}
    fn visit_increase(&self, ir_increase: &IRIncrease) {
        self.visit_dyn(ir_increase._type.as_ref());
        self.visit_dyn(ir_increase.operand.as_ref());
        if let Some(target) = &ir_increase.target {
            self.visit_dyn(target.as_ref());
        }
    }
    fn visit_decrease(&self, ir_decrease: &IRDecrease) {
        self.visit_dyn(ir_decrease._type.as_ref());
        self.visit_dyn(ir_decrease.operand.as_ref());
        if let Some(target) = &ir_decrease.target {
            self.visit_dyn(target.as_ref());
        }
    }
    fn visit_stack_allocate(&self, ir_stack_allocate: &IRStackAllocate) {
        self.visit_dyn(ir_stack_allocate.size.as_ref());
        self.visit_virtual_register(ir_stack_allocate.target.as_ref());
    }
    fn visit_type_cast(&self, ir_type_cast: &IRTypeCast) {
        self.visit_dyn(ir_type_cast.original_type.as_ref());
        self.visit_dyn(ir_type_cast.source.as_ref());
        self.visit_dyn(ir_type_cast.target_type.as_ref());
        self.visit_virtual_register(ir_type_cast.target.as_ref());
    }
    fn visit_asm(&self, ir_asm: &IRAsm) {
        for (_type, resource) in ir_asm.types.iter().zip(ir_asm.resources.iter()) {
            self.visit_dyn(_type.as_ref());
            self.visit_dyn(resource.as_ref());
        }
    }
    fn visit_constant(&self, ir_constant: &IRConstant) {}
    fn visit_virtual_register(&self, ir_virtual_register: &IRVirtualRegister) {}
    fn visit_phi(&self, ir_phi: &IRPhi) {
        self.visit_dyn(ir_phi._type.as_ref());
        for operand in ir_phi.operands.iter() {
            self.visit_dyn(operand.as_ref());
        }
    }
    fn visit_macro(&self, ir_macro: &IRMacro) {}
}

pub trait IRVisitorImpl: IRVisitor {
    fn visit_function(&self, ir_function: &IRFunction);
}

impl<T: IRVisitorImpl + Sized> IRVisitor for T {
    fn visit_dyn(&self, ir_node: &dyn IRNode) {
        ir_node.accept(self);
    }
}

pub struct IRDumper {}
impl IRVisitorImpl for IRDumper {
    fn visit_function(&self, ir_function: &IRFunction) {}
}
