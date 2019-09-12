pub struct Stack<T> {
    head: NodePtr<T>,
}

struct NodePtr<T>(*mut Node<T>);

struct Node<T> {
    elem: T,
    next: NodePtr<T>,
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: NodePtr::null() }
    }

    pub fn push(&mut self, elem: T) {
        self.head.push(elem)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.elem()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.elem_mut()
    }
}

impl<T> Clone for NodePtr<T> {
    fn clone(&self) -> NodePtr<T> {
        Self(self.0)
    }
}

impl<T> Copy for NodePtr<T> {}

impl<T> NodePtr<T> {
    fn null() -> NodePtr<T> {
        Self(core::ptr::null_mut())
    }

    fn is_null(&self) -> bool {
        self.0.is_null()
    }

    fn set(&mut self, node: NodePtr<T>) {
        self.0 = node.0
    }

    fn new(elem: T) -> Self {
        let node = Node {
            elem,
            next: Self::null(),
        };
        Self(Box::into_raw(Box::new(node)))
    }

    fn elem(&self) -> Option<&T> {
        if self.is_null() {
            return None;
        }
        unsafe { Some(&(*self.0).elem) }
    }

    fn elem_mut(&mut self) -> Option<&mut T> {
        if self.is_null() {
            return None;
        }
        unsafe { Some(& mut(*self.0).elem) }
    }

    fn next(&self) -> Self {
        if self.is_null() {
            return Self::null();
        }
        unsafe { (*self.0).next }
    }

    fn set_next(&mut self, next: NodePtr<T>) {
        if self.is_null() {
            return;
        }
        unsafe { (*self.0).next = next }
    }

    fn push(&mut self, elem: T) {
        let mut node = Self::new(elem);
        node.set_next(self.clone());
        self.set(node);
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_null() {
            return None;
        }
        let node = unsafe { Box::from_raw(self.0) };
        let next = self.next();
        self.set(next);
        Some(node.elem)
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
