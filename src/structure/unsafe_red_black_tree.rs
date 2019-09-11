use core::cmp::Ordering;
use core::ptr;

pub struct RedBlackTree<K: Ord, V> {
    root: NodePtr<K, V>,
}

struct NodePtr<K: Ord, V>(*mut Node<K, V>);

struct Node<K: Ord, V> {
    key: K,
    value: V,
    left: NodePtr<K, V>,
    right: NodePtr<K, V>,
    color: Color,
    count: usize,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Color {
    Red,
    Black,
}

impl<K: Ord, V> Drop for RedBlackTree<K, V> {
    fn drop(&mut self) {
        self.root.clear()
    }
}

impl<K: Ord, V> RedBlackTree<K, V> {
    pub fn new() -> Self {
        Self { root: NodePtr::null() }
    }

    pub fn len(&self) -> usize {
        self.root.count()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let node_ptr = self.root.get_node_ptr(key);
        if node_ptr.is_null() {
            return None;
        }
        unsafe { Some(&(*node_ptr.0).value) }
    }

    pub fn put(&mut self, key: K, value: V) {
        self.root.set(NodePtr::put(&mut self.root.clone(), key, value));
        self.root.set_color(Color::Black);
    }
}

impl<K: Ord, V> Clone for NodePtr<K, V> {
    fn clone(&self) -> NodePtr<K, V> {
        Self(self.0)
    }
}

impl<K: Ord, V> Copy for NodePtr<K, V> {}

impl<K: Ord, V> NodePtr<K, V> {
    fn null() -> NodePtr<K, V> {
        Self(ptr::null_mut())
    }

    fn is_null(&self) -> bool {
        self.0.is_null()
    }

    fn new(key: K, value: V, color: Color) -> Self {
        let node = Node {
            key,
            value,
            left: NodePtr::null(),
            right: NodePtr::null(),
            color,
            count: 1,
        };
        Self(Box::into_raw(Box::new(node)))
    }

    fn is_red(&self) -> bool {
        if self.is_null() {
            false
        } else {
            unsafe { (*self.0).color == Color::Red }
        }
    }

    fn color(&self) -> Color {
        if self.is_null() {
            return Color::Black;
        }
        unsafe { (*self.0).color }
    }

    fn set_color(&mut self, color: Color) {
        if self.is_null() {
            return;
        }
        unsafe { (*self.0).color = color }
    }

    fn count(&self) -> usize {
        if self.is_null() {
            0
        } else {
            unsafe { (*self.0).count }
        }
    }

    fn set_count(&mut self, count: usize) {
        if self.is_null() {
            return;
        }
        unsafe { (*self.0).count = count }
    }

    fn left(&self) -> Self {
        if self.is_null() {
            return NodePtr::null();
        }
        unsafe { (*self.0).left }
    }

    fn set_left(&mut self, left: NodePtr<K, V>) {
        if self.is_null() {
            return;
        }
        unsafe { (*self.0).left = left }
    }

    fn right(&self) -> Self {
        if self.is_null() {
            return NodePtr::null();
        }
        unsafe { (*self.0).right }
    }

    fn set_right(&mut self, right: NodePtr<K, V>) {
        if self.is_null() {
            return;
        }
        unsafe { (*self.0).right = right }
    }

    fn key(&self) -> Option<&K> {
        if self.is_null() {
            return None;
        }
        unsafe { Some(&(*self.0).key) }
    }

    fn set_value(&mut self, value: V) {
        if self.is_null() {
            return;
        }
        let mut node = unsafe { Box::from_raw(self.0) };
        let _old = core::mem::replace(&mut node.value, value);
        let _ = Box::into_raw(node);
    }

    fn get_node_ptr(&self, key: &K) -> Self {
        match self.key() {
            None => Self::null(),
            Some(k) => match key.cmp(k) {
                Ordering::Less => self.left().get_node_ptr(key),
                Ordering::Greater => self.right().get_node_ptr(key),
                Ordering::Equal => self.clone(),
            },
        }
    }

    fn set(&mut self, node: NodePtr<K, V>) {
        self.0 = node.0
    }

    fn put(h: &mut NodePtr<K, V>, key: K, value: V) -> NodePtr<K, V> {
        match h.key() {
            None => return NodePtr::new(key, value, Color::Red), // null ptr
            Some(h_key) => match key.cmp(h_key) {
                Ordering::Less => h.set_left(NodePtr::put(&mut h.left(), key, value)),
                Ordering::Greater => h.set_right(NodePtr::put(&mut h.right(), key, value)),
                Ordering::Equal => h.set_value(value),
            },
        }
        if h.right().is_red() && !h.left().is_red() {
            h.set(NodePtr::rotate_left(&mut h.clone()));
        }
        if h.left().is_red() && h.left().left().is_red() {
            h.set(NodePtr::rotate_right(&mut h.clone()));
        }
        if h.left().is_red() && h.right().is_red() {
            NodePtr::flip_colors(h);
        }
        h.set_count(1 + h.left().count() + h.right().count());
        h.clone()
    }

    fn rotate_left(h: &mut NodePtr<K, V>) -> NodePtr<K, V> {
        let mut x = h.right();
        h.set_right(x.left());
        x.set_left(h.clone());
        x.set_color(h.color());
        h.set_color(Color::Red);
        x.set_count(h.count());
        h.set_count(1 + h.left().count() + h.right().count());
        x
    }

    fn rotate_right(h: &mut NodePtr<K, V>) -> NodePtr<K, V> {
        let mut x = h.left();
        h.set_left(x.right());
        x.set_right(h.clone());
        x.set_color(h.color());
        h.set_color(Color::Red);
        x.set_count(h.count());
        h.set_count(1 + h.left().count() + h.right().count());
        x
    }

    fn flip_colors(h: &mut NodePtr<K, V>) {
        h.set_color(Color::Red);
        h.left().set_color(Color::Black);
        h.right().set_color(Color::Black);
    }

    fn clear(&mut self) {
        if !self.is_null() {
            unsafe {
                self.left().clear();
                self.right().clear();
                Box::from_raw(self.0);
            }
        }
    }
}

#[cfg(test)]
impl<K, V> RedBlackTree<K, V>
where
    K: Ord + ToString,
    V: ToString,
{
    fn inorder_tree_walk_string(&self) -> String {
        self.root.inorder_tree_walk_string()
    }
}

#[cfg(test)]
impl<K, V> NodePtr<K, V>
where
    K: Ord + ToString,
    V: ToString,
{
    fn pair(&self) -> (&K, &V) {
        unsafe { (&(*self.0).key, &(*self.0).value) }
    }

    fn inorder_tree_walk_string(&self) -> String {
        let mut result: String = "".into();
        result.push_str("(");

        if !self.is_null() {
            let (key, value) = self.pair();
            result.push_str(&key.to_string());
            result.push_str(":");
            result.push_str(&value.to_string());
            result.push_str(":");
            if self.is_red() {
                result.push_str("r");
            } else {
                result.push_str("b");
            }

            let left_str = self.left().inorder_tree_walk_string();
            result.push_str(&left_str);

            let right_str = self.right().inorder_tree_walk_string();
            result.push_str(&right_str);
        }

        result.push_str(")");
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        assert_eq!(rbtree.get(&'S'), None);
        assert_eq!(rbtree.len(), 0);
        rbtree.put('S', 1);
        rbtree.put('E', 2);
        rbtree.put('A', 3);
        rbtree.put('R', 4);
        rbtree.put('C', 5);
        rbtree.put('H', 6);
        rbtree.put('X', 7);
        rbtree.put('M', 8);
        rbtree.put('P', 9);
        rbtree.put('L', 10);
        assert_eq!(rbtree.get(&'S'), Some(&1));
        assert_eq!(rbtree.get(&'E'), Some(&2));
        assert_eq!(rbtree.get(&'A'), Some(&3));
        assert_eq!(rbtree.get(&'R'), Some(&4));
        assert_eq!(rbtree.get(&'C'), Some(&5));
        assert_eq!(rbtree.get(&'H'), Some(&6));
        assert_eq!(rbtree.get(&'X'), Some(&7));
        assert_eq!(rbtree.get(&'M'), Some(&8));
        assert_eq!(rbtree.get(&'P'), Some(&9));
        assert_eq!(rbtree.get(&'L'), Some(&10));
        assert_eq!(rbtree.get(&'Z'), None);
        assert_eq!(rbtree.len(), 10);
    }

    #[test]
    fn test_inorder_tree_walk_1() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        assert_eq!(rbtree.inorder_tree_walk_string(), "()");
        rbtree.put('S', 1);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(S:1:b()())");
        rbtree.put('E', 2);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(S:1:b(E:2:r()())())");
        rbtree.put('A', 3);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(E:2:b(A:3:b()())(S:1:b()()))");
        rbtree.put('R', 4);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(E:2:b(A:3:b()())(S:1:b(R:4:r()())()))");
        rbtree.put('C', 5);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(E:2:b(C:5:b(A:3:r()())())(S:1:b(R:4:r()())()))");
        rbtree.put('H', 6);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(R:4:b(E:2:r(C:5:b(A:3:r()())())(H:6:b()()))(S:1:b()()))");
        rbtree.put('X', 7);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(R:4:b(E:2:r(C:5:b(A:3:r()())())(H:6:b()()))(X:7:b(S:1:r()())()))");
        rbtree.put('M', 8);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(R:4:b(E:2:r(C:5:b(A:3:r()())())(M:8:b(H:6:r()())()))(X:7:b(S:1:r()())()))");
        rbtree.put('P', 9);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(M:8:b(E:2:b(C:5:b(A:3:r()())())(H:6:b()()))(R:4:b(P:9:b()())(X:7:b(S:1:r()())())))");
        rbtree.put('L', 10);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(M:8:b(E:2:b(C:5:b(A:3:r()())())(L:10:b(H:6:r()())()))(R:4:b(P:9:b()())(X:7:b(S:1:r()())())))");
    }

    #[test]
    fn test_inorder_tree_walk_2() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        assert_eq!(rbtree.inorder_tree_walk_string(), "()");
        rbtree.put('A', 1);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(A:1:b()())");
        rbtree.put('C', 2);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(C:2:b(A:1:r()())())");
        rbtree.put('E', 3);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(C:2:b(A:1:b()())(E:3:b()()))");
        rbtree.put('H', 4);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(C:2:b(A:1:b()())(H:4:b(E:3:r()())()))");
        rbtree.put('L', 5);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(H:4:b(C:2:r(A:1:b()())(E:3:b()()))(L:5:b()()))");
        rbtree.put('M', 6);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(H:4:b(C:2:r(A:1:b()())(E:3:b()()))(M:6:b(L:5:r()())()))");
        rbtree.put('P', 7);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(H:4:b(C:2:b(A:1:b()())(E:3:b()()))(M:6:b(L:5:b()())(P:7:b()())))");
        rbtree.put('R', 8);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(H:4:b(C:2:b(A:1:b()())(E:3:b()()))(M:6:b(L:5:b()())(R:8:b(P:7:r()())())))");
        rbtree.put('S', 9);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(H:4:b(C:2:b(A:1:b()())(E:3:b()()))(R:8:b(M:6:r(L:5:b()())(P:7:b()()))(S:9:b()())))");
        rbtree.put('X', 10);
        assert_eq!(rbtree.inorder_tree_walk_string(), "(H:4:b(C:2:b(A:1:b()())(E:3:b()()))(R:8:b(M:6:r(L:5:b()())(P:7:b()()))(X:10:b(S:9:r()())())))");
    }
}
