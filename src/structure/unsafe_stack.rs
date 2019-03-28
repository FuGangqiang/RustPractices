use std::ptr::NonNull;

pub struct Stack<T> {
    head: Option<NonNull<Node<T>>>,
}

struct Node<T> {
    elem: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Node { elem, next: None }
    }

    fn into_element(self: Box<Self>) -> T {
        self.elem
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let mut node = Box::new(Node::new(elem));
        node.next = self.head;
        let node = Some(Box::into_raw_non_null(node));
        self.head = node;
    }

    pub fn pop(&mut self) -> Option<T> {
        let node = self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;
            node
        });
        node.map(Node::into_element)
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.as_ref().elem) }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.as_mut().elem) }
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn basics() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
}
