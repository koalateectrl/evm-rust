use ethnum::U256;

#[derive(Debug)]
pub struct Memory {
    memory: Vec<U256>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory { memory: Vec::new() }
    }

    pub fn store(&mut self, offset: usize, value: U256) {
        if offset > std::usize::MAX {
            panic!("Invalid memory access");
        }

        if value < 0 || value > U256::MAX {
            panic!("Invalid memory value") ;
        }

        if offset >= self.memory.len() {
            self.memory.extend(vec!(U256::new(0); offset - self.memory.len() + 1))
        }

        self.memory[offset] = value;
    }

    pub fn load(&self, offset: usize) -> U256 {
        if offset >= self.memory.len() {
            return U256::new(0);
        }

        self.memory[offset]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_empty() {
        let test_memory: Memory = Memory::new();
        let empty_memory: Vec<U256> = Vec::new();
        assert_eq!(test_memory.memory, empty_memory);
    }

    #[test]
    fn store_should_append() {
        let mut test_memory = Memory::new();
        test_memory.store(0, U256::new(1364));
        test_memory.store(5, U256::new(7777));
        assert_eq!(test_memory.memory, vec![1364, 0, 0, 0, 0, 7777]);
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
        test_memory.store(0, U256::new(1364));
        test_memory.store(5, U256::new(7777));
        let loaded_value = test_memory.load(5);
        assert_eq!(loaded_value, 7777);
    }
}