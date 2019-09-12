pub struct Deque<T> {
    head: NodePtr<T>,
    tail: NodePtr<T>,
}

struct NodePtr<T>(*mut Node<T>);

struct Node<T> {
    elem: T,
    next: NodePtr<T>,
    prev: NodePtr<T>,
}

impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Deque {
            head: NodePtr::null(),
            tail: NodePtr::null(),
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let mut node = NodePtr::new(elem);
        node.set_next(self.head);
        if self.head.is_null() {
            self.tail.set(node);
        } else {
            self.head.set_prev(node)
        }
        self.head.set(node);
    }

    pub fn push_back(&mut self, elem: T) {
        let mut node = NodePtr::new(elem);
        node.set_prev(self.tail);
        if self.tail.is_null() {
            self.head.set(node);
        } else {
            self.tail.set_next(node);
        }
        self.tail.set(node);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }
        let node = unsafe { Box::from_raw(self.head.0) };
        self.head.set(node.next);
        if self.head.is_null() {
            self.tail.set(NodePtr::null());
        } else {
            self.head.set_prev(NodePtr::null());
        }
        Some(node.elem)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        }
        let node = unsafe { Box::from_raw(self.tail.0) };
        self.tail.set(node.prev);
        if self.tail.is_null() {
            self.head.set(NodePtr::null());
        } else {
            self.tail.set_next(NodePtr::null());
        }
        Some(node.elem)
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.elem()
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.elem_mut()
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.tail.elem()
    }

    pub fn peek_back_mut(&mut self) -> Option<&mut T> {
        self.tail.elem_mut()
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
            prev: Self::null(),
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
        unsafe { Some(&mut (*self.0).elem) }
    }

    fn set_next(&mut self, next: NodePtr<T>) {
        if self.is_null() {
            return;
        }
        unsafe { (*self.0).next = next }
    }

    fn set_prev(&mut self, prev: NodePtr<T>) {
        if self.is_null() {
            return;
        }
        unsafe { (*self.0).prev = prev }
    }
}

pub struct IntoIter<T>(Deque<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> IntoIterator for Deque<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

#[cfg(test)]
mod test {
    use super::Deque;

    #[test]
    fn basics() {
        let mut queue = Deque::new();

        // Check empty Deque behaves right
        assert_eq!(queue.pop_front(), None);

        // Populate Deque
        queue.push_front(1);
        queue.push_front(2);
        queue.push_front(3);

        // Check normal removal
        assert_eq!(queue.pop_front(), Some(3));
        assert_eq!(queue.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        queue.push_front(4);
        queue.push_front(5);

        // Check normal removal
        assert_eq!(queue.pop_front(), Some(5));
        assert_eq!(queue.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(queue.pop_front(), Some(1));
        assert_eq!(queue.pop_front(), None);

        // ---- back -----

        // Check empty Deque behaves right
        assert_eq!(queue.pop_back(), None);

        // Populate Deque
        queue.push_back(1);
        queue.push_back(2);
        queue.push_back(3);

        // Check normal removal
        assert_eq!(queue.pop_back(), Some(3));
        assert_eq!(queue.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        queue.push_back(4);
        queue.push_back(5);

        // Check normal removal
        assert_eq!(queue.pop_back(), Some(5));
        assert_eq!(queue.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(queue.pop_back(), Some(1));
        assert_eq!(queue.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut queue = Deque::new();
        assert!(queue.peek_front().is_none());
        assert!(queue.peek_back().is_none());
        assert!(queue.peek_front_mut().is_none());
        assert!(queue.peek_back_mut().is_none());

        queue.push_front(1);
        queue.push_front(2);
        queue.push_front(3);

        assert_eq!(&*queue.peek_front().unwrap(), &3);
        assert_eq!(&mut *queue.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*queue.peek_back().unwrap(), &1);
        assert_eq!(&mut *queue.peek_back_mut().unwrap(), &mut 1);
    }

    #[test]
    fn into_iter() {
        let mut queue = Deque::new();
        queue.push_front(1);
        queue.push_front(2);
        queue.push_front(3);

        let mut iter = queue.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}
