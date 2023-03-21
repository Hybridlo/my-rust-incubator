use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
struct SharedStack<T> {
    inner_data: Rc<RefCell<Vec<T>>>
}

impl<T> SharedStack<T> {
    /// creates an empty stack
    pub fn new() -> Self {
        Self { inner_data: Default::default() }
    }

    pub fn push(&self, item: T) {
        let mut vec_borrow = self.inner_data.borrow_mut();
        vec_borrow.push(item);
    }

    pub fn pop(&self) -> Option<T> {
        let mut vec_borrow = self.inner_data.borrow_mut();
        
        vec_borrow.pop()
    }

    /// we can't provide a normal peek, since borrowing from a shared
    /// collection is not possible through RefCell (because then we could
    /// take a shared ref from the vec, and then try to mutate it later)
    pub fn peek_check(&self, f: impl FnOnce(&T) -> bool) -> bool {
        let vec_borrow = self.inner_data.borrow();

        vec_borrow.last().map_or(false, f)
    }

    pub fn peek_inspect(&self, f: impl FnOnce(&T)) {
        let vec_borrow = self.inner_data.borrow();

        if let Some(item) = vec_borrow.last() {
            f(item)
        }
    }

    pub fn is_empty(&self) -> bool {
        let vec_borrow = self.inner_data.borrow();

        vec_borrow.is_empty()
    }
}

fn main() {
    let stack = SharedStack::new();
    let stack2 = stack.clone();

    stack.push(15);
    stack.push(20);

    stack2.peek_inspect(|item| println!("{}", item));

    stack2.push(1);

    assert!(stack.peek_check(|item| *item == 1), "Peek returned false");

    stack2.pop();
    stack2.pop();
    stack2.pop();

    assert!(stack.is_empty(), "Stack is not empty");
}
