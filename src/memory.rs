use std::cmp::max;

#[derive(Debug)]
pub struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory { memory: Vec::new() }
    }

    pub fn store(&mut self, offset: usize, value: u8) {
        if offset > std::usize::MAX {
            panic!("Invalid memory access");
        }

        if value > u8::MAX {
            panic!("Invalid memory value") ;
        }

        if offset >= self.memory.len() {
            self.memory.extend(vec!(0; offset - self.memory.len() + 1))
        }

        self.memory[offset] = value;
    }

    pub fn load(&self, offset: usize) -> u8 {
        if offset >= self.memory.len() {
            return 0;
        }

        self.memory[offset]
    }

    pub fn load_range(&self, offset: usize, length: usize) -> Vec<u8> {
        let endpoint = max(self.memory.len(), offset + length);
        self.memory[offset..endpoint].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_empty() {
        let test_memory: Memory = Memory::new();
        let empty_memory: Vec<u8> = Vec::new();
        assert_eq!(test_memory.memory, empty_memory);
    }

    #[test]
    fn store_should_append() {
        let mut test_memory = Memory::new();
        test_memory.store(0, 55);
        test_memory.store(5, 23);
        assert_eq!(test_memory.memory, vec![55, 0, 0, 0, 0, 23]);
    }

    #[test]
    fn load_should_return_zero() {
        let test_memory = Memory::new();
        let loaded_value = test_memory.load(5);
        assert_eq!(loaded_value, 0);
    }

    #[test]
    fn load_should_correct() {
        let mut test_memory = Memory::new();
        test_memory.store(0, 6);
        test_memory.store(5, 10);
        let loaded_value = test_memory.load(5);
        assert_eq!(loaded_value, 10);
    }
}