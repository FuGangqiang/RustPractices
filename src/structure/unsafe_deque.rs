use std::ptr::NonNull;

pub struct Deque<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

struct Node<T> {
    elem: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Node {
            elem,
            next: None,
            prev: None,
        }
    }

    fn into_element(self: Box<Self>) -> T {
        self.elem
    }
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Deque {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let mut node = Box::new(Node::new(elem));
        unsafe {
            node.next = self.head;
            let node = Some(Box::into_raw_non_null(node));
            match self.head {
                None => self.tail = node,
                Some(mut head) => head.as_mut().prev = node,
            }
            self.head = node;
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let mut node = Box::new(Node::new(elem));
        unsafe {
            node.prev = self.tail;
            let node = Some(Box::into_raw_non_null(node));
            match self.tail {
                None => self.head = node,
                Some(mut tail) => tail.as_mut().next = node,
            }
            self.tail = node;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let node = self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;
            match self.head {
                None => self.tail = None,
                Some(mut head) => head.as_mut().prev = None,
            }
            node
        });
        node.map(Node::into_element)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let node = self.tail.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.tail = node.prev;
            match self.tail {
                None => self.head = None,
                Some(mut tail) => tail.as_mut().next = None,
            }
            node
        });
        node.map(Node::into_element)
    }

    pub fn peek_front(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.as_ref().elem) }
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.as_mut().elem) }
    }

    pub fn peek_back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.as_mut().map(|node| &mut node.as_mut().elem) }
    }

    pub fn peek_back(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|node| &node.as_ref().elem) }
    }
}

impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
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
