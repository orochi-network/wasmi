use crate::engine::bytecode::Instruction;
use crate::engine::code_map::InstructionPtr;
use core::fmt::Debug;
use std::ptr;
use wasmi_core::UntypedValue;

#[derive(Clone, Debug)]
pub enum MemoryAction {
    None,
    // Init,
    // Read,
    // Write,
}

#[derive(Debug, Clone)]
pub struct Trace {
    iaddr: InstructionPtr,
    program_counter: u64,
    opcode: Instruction,
    stack: Vec<UntypedValue>,
    stack_depth: usize,
    local: Vec<UntypedValue>,
    local_depth: usize,
    calling_frame: Vec<fn()>,
    calling_frame_depth: usize,
    action: MemoryAction,
    memory_address: u64,
    memory: Vec<u8>,
}

impl Trace {
    pub fn new() -> Self {
        Self {
            // Program
            iaddr: InstructionPtr::new(ptr::null()),
            program_counter: 0,
            opcode: Instruction::Unreachable,
            // Stack
            stack: vec![],
            stack_depth: 0,
            // Local
            local: vec![],
            local_depth: 0,
            // Calling frame
            calling_frame: vec![],
            calling_frame_depth: 0,
            // Memory
            memory_address: 0,
            action: MemoryAction::None,
            memory: vec![],
        }
    }

    pub fn set_local(&mut self, local: Vec<UntypedValue>) {
        self.local = local;
        self.local_depth = self.local.len();
    }

    pub fn inc_pc(&mut self) {
        self.program_counter += 1;
    }

    pub fn push(&mut self, value: UntypedValue) {
        self.stack.push(value);
        self.stack_depth = self.stack.len();
    }

    pub fn pop(&mut self) -> UntypedValue {
        self.stack_depth = self.stack.len();
        self.stack.pop().expect("The stack is empty")
    }

    pub fn set_instruction(&mut self, instr: Instruction) {
        self.opcode = instr;
    }

    pub fn set_iaddr(&mut self, ptr_instr: InstructionPtr) {
        self.iaddr = ptr_instr;
    }
}
