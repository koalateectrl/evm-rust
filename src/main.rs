mod stack;
mod memory;
mod execution;

fn main() {
    let my_bytecode = "600660070260005360016000f3";
    let mut my_context = execution::ExecutionContext::new(my_bytecode);
    
    my_context.run();
    
}
