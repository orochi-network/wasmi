use wasmi_core::UntypedValue;
use super::trace::Trace;
use crate::engine::code_map::InstructionPtr;
use crate::engine::bytecode::Instruction;

// Predator going to hunt for trace
#[derive(Debug, Clone)]
pub struct Predator {
    current_state: Trace,
    execution_trace: Vec<Trace>,
}

impl<'a> Predator {
    pub fn new( local: Vec<UntypedValue>) -> Self{
        Self {
            current_state: Trace::new( local),
            execution_trace: vec![]
        }
    }

    pub fn push(&mut self, value: UntypedValue){
        self.current_state.push(value);
    }

    pub fn pop(&mut self) -> UntypedValue  {
        self.current_state.pop()
    }

    pub fn set_instruction(&mut self, instr: Instruction){
        self.current_state.set_instruction(instr);
    }

    pub fn set_iaddr(&mut self, ptr_instr: InstructionPtr){
        self.current_state.set_iaddr(ptr_instr);
    }

    pub fn update_trace(&mut self){
        self.execution_trace.push(self.current_state.clone());
        self.current_state.inc_pc();
    }

    pub fn get_trace(&self)-> Vec<Trace>{
        self.execution_trace.clone()
    }
}
