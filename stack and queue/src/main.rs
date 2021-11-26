#[allow(dead_code)]
fn main() {
    let mut my_stack = Stack { stack: Vec::new()};
    my_stack.push(5);
    my_stack.push(2);
    my_stack.push(10);
    my_stack.push(6);

    my_stack.pop();
    assert_eq!(my_stack.peek().unwrap(), &10);
}

struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { stack: Vec::new() }
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn length(&self) -> usize {
        self.stack.len()
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}
