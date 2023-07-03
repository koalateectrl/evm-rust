use ethnum::U256;

#[derive(Debug)]
pub struct Stack {
    stack: Vec<U256>,
    max_depth: usize,
}

impl Stack {
    pub fn new(max_depth: usize) -> Stack {
        let stack: Vec<U256> = Vec::new();
        Stack { stack, max_depth }
    }

    pub fn push(&mut self, item: U256) {

        if (self.stack.len() + 1) > self.max_depth {
            panic!("Stack overflow");
        }

        self.stack.push(item);
    }

    pub fn pop(&mut self) -> Option<U256> {
        if self.stack.len() == 0 {
            panic!("Stack underflow");
        }

        self.stack.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_empty() {
        let test_stack = Stack::new(1024);
        let empty_stack: Vec<U256> = Vec::new();
        assert_eq!(test_stack.stack, empty_stack);
    }

    #[test]
    fn push_should_append() {
        let mut test_stack = Stack::new(1024);
        test_stack.push(U256::new(25));
        test_stack.push(U256::new(45));
        assert_eq!(test_stack.stack, vec![25, 45]);
    }

    #[test]
    #[should_panic(expected = "Stack underflow")]
    fn pop_empty_should_panic() {
        let mut test_stack = Stack::new(1024);
        test_stack.pop();
    }

    #[test]
    fn pop_value_should_correct() {
        let mut test_stack = Stack::new(1024);
        test_stack.push(U256::new(56));
        test_stack.push(U256::new(99));
        match test_stack.pop() {
            Some(value) => assert_eq!(99, value),
            None => panic!("Pushed 2 items, popped only 1"),
        }
        
    }
}