#[derive(Default)]
pub struct MaxHeap<T: Ord> {
    heap: Vec<T>,
}

impl<T: Ord> MaxHeap<T> {
    pub fn new() -> MaxHeap<T> {
        MaxHeap { heap: vec![] }
    }

    pub fn init(mut self, xs: Vec<T>) -> MaxHeap<T> {
        self.heap = xs;
        for i in (0..self.len()).rev() {
            self.sink(i);
        }
        self
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn maxmium(&self) -> Option<&T> {
        self.heap.first()
    }

    pub fn pop_max(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let max = self.heap.swap_remove(0);
        if !self.is_empty() {
            self.sink(0);
        }

        Some(max)
    }

    pub fn push(&mut self, x: T) {
        self.heap.push(x);
        self.swim(self.len() - 1);
    }

    fn sink(&mut self, i: usize) {
        let mut i = i;
        while i < self.len() {
            let l = left(i);
            let r = right(i);
            let mut largest = i;
            if l < self.len() && self.heap[l] > self.heap[largest] {
                largest = l;
            }
            if r < self.len() && self.heap[r] > self.heap[largest] {
                largest = r;
            }
            if largest != i {
                self.heap.swap(i, largest);
                i = largest;
            } else {
                break;
            }
        }
    }

    fn swim(&mut self, i: usize) {
        let mut i = i;
        while i != 0 && self.heap[parent(i)] < self.heap[i] {
            self.heap.swap(i, parent(i));
            i = parent(i);
        }
    }
}

fn parent(i: usize) -> usize {
    (i - 1) / 2
}

fn left(i: usize) -> usize {
    2 * i + 1
}

fn right(i: usize) -> usize {
    2 * i + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let xs = vec![4, 5, 1, 2, 4];
        let mut heap = MaxHeap::new().init(xs);
        assert_eq!(5, heap.len());
        assert_eq!(Some(&5), heap.maxmium());
        assert_eq!(Some(5), heap.pop_max());
        assert_eq!(Some(4), heap.pop_max());
        assert_eq!(Some(4), heap.pop_max());
        assert_eq!(Some(2), heap.pop_max());
        assert_eq!(Some(1), heap.pop_max());
        assert_eq!(None, heap.pop_max());
    }

    #[test]
    fn test_push_pop() {
        let mut heap = MaxHeap::new();
        assert_eq!(0, heap.len());
        heap.push(4);
        heap.push(5);
        heap.push(1);
        heap.push(2);
        heap.push(4);
        assert_eq!(5, heap.len());
        assert_eq!(Some(&5), heap.maxmium());
        assert_eq!(Some(5), heap.pop_max());
        assert_eq!(Some(4), heap.pop_max());
        assert_eq!(Some(4), heap.pop_max());
        assert_eq!(Some(2), heap.pop_max());
        assert_eq!(Some(1), heap.pop_max());
        assert_eq!(None, heap.pop_max());
    }
}
