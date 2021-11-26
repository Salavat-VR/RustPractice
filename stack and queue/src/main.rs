#[allow(dead_code)]
fn main() {}

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

struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue { queue: Vec::new() }
    }

    fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }

    fn length(&self) -> usize {
        self.queue.len()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut my_stack = Stack { stack: Vec::new() };
        my_stack.push(5);
        my_stack.push(2);
        my_stack.push(10);
        my_stack.push(6);

        my_stack.pop();
        assert_eq!(my_stack.peek().unwrap(), &10);
    }

    fn test_queue() {
        let mut my_queue = Queue { queue: Vec::new() };

        my_queue.enqueue(1);
        my_queue.enqueue(2);
        my_queue.enqueue(4);
        my_queue.dequeue();

        assert_eq!(my_queue.peek().unwrap(), &2);
    }
}
