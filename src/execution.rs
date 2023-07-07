use std::collections::HashMap;

use hex;
use crate::stack;
use crate::memory;
use ethnum::U256;

pub struct ExecutionContext {
    pub code: Vec<u8>,
    stack: stack::Stack,
    memory: memory::Memory,
    pub pc: usize,
    pub stopped: bool,
    instruction_map: HashMap<u128, String>,
}

impl ExecutionContext {
    pub fn new(bytecode_as_str: &str) -> ExecutionContext {
        let bytecode_as_bytes: Vec<u8> = hex::decode(bytecode_as_str).unwrap();
        let mut instruction_map_: HashMap<u128, String> = HashMap::new();
        instruction_map_.insert(0, "STOP".to_string());
        instruction_map_.insert(1, "ADD".to_string());
        instruction_map_.insert(2, "MUL".to_string());
        instruction_map_.insert(96, "PUSH1".to_string());

        ExecutionContext { 
            code: bytecode_as_bytes, 
            stack: stack::Stack::new(), 
            memory: memory::Memory::new(), 
            pc: 0, 
            stopped: false,
            instruction_map: instruction_map_,
        }
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn read_code(&mut self, num_bytes: usize) -> U256 {
        let value = Self::vec_to_u256(&self.code, self.pc, self.pc + num_bytes);
        self.pc += num_bytes;
        value
    }

    // Helper function
    fn vec_to_u256(bytes: &Vec<u8>, start: usize, end: usize) -> U256 {
        let mut result = U256::new(0);
        for i in start..end {
            result = (result << 8) | U256::new(bytes[i].into());
        }
        result
    }

    fn decode_opcode(&mut self) -> u128 {
        // Follow section 9.4.1 of Ethereum yellow paper, STOP if pc is outside of code
        if self.pc >= self.code.len() {
            return 0;
        }
        
        let pc_before = self.pc;
        let opcode = self.read_code(1).as_u128();

        // PRINT SOME INFO
        println!("{} @ pc={}", self.instruction_map.get(&opcode).unwrap(), pc_before);

        opcode
    }

    pub fn run(&mut self) {
        while !self.stopped {
            let instruction = self.decode_opcode();
            self.run_instruction(&instruction);
        }
    }

    pub fn run_instruction(&mut self, instruction: &u128) {
        match instruction {
            0 => self.stop(),
            1 => {
                    let first_pop = self.stack.pop().unwrap();
                    let second_pop = self.stack.pop().unwrap();
                    self.stack.push(first_pop + second_pop);
            },
            2 => {
                    let first_pop = self.stack.pop().unwrap();
                    let second_pop = self.stack.pop().unwrap();
                    self.stack.push(first_pop * second_pop);
            },
            96 => {
                let read_value = self.read_code(1);
                self.stack.push(read_value);
            }, 
            _ => panic!("Invalid instruction"),
        }

        println!("{:?}", self.stack);
        println!("{:?}", self.memory);
        println!("");
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytecode_legit_should_parse() {
        let test_ec = ExecutionContext::new("48656c6c6f20776f726c6421");
        assert_eq!(test_ec.code, vec![72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33]);
    }

    #[test]
    #[should_panic]
    fn bytecode_bad_should_fail() {
        ExecutionContext::new("48656c6c6f20776f726c642");
    }

    #[test]
    fn stopped_should_stop() {
        let mut test_ec = ExecutionContext::new("");
        assert_eq!(test_ec.stopped, false);
        test_ec.stop();
        assert_eq!(test_ec.stopped, true);
    }

    #[test]
    fn read_code_should_read() {
        let mut test_ec = ExecutionContext::new("600660070260005360016000f3");
        assert_eq!(test_ec.read_code(1), 96);
        assert_eq!(test_ec.read_code(1), 6);
        assert_eq!(test_ec.read_code(3), 6293250);
    }
}