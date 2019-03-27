use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct Deque<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
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
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub struct IntoIter<T>(Deque<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
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

        // Check empty queue behaves right
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
