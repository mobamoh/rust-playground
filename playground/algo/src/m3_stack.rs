/*
   can be implemented using dynamic arrays
   Push/Pop/Peek (Top) => Big O(1)
   LIFO Data Structure
*/

#[derive(Debug)]
struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn test_stack() {
        let mut stack = Stack::<i32>::new();

        println!("Pushing to the stack");
        stack.push(3);
        stack.push(6);
        stack.push(9);
        stack.push(12);
        dbg!("{}",&stack);

        println!("Popping from the stack");
        stack.pop();
        dbg!("{}",&stack);

        println!("Peeking from the stack");
        let last_element = stack.peek();
        dbg!("{}",&last_element);

    }
}
