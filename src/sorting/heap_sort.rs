pub fn heap_sort<T: Ord + Copy>(xs: &mut [T]) {
    if xs.len() <= 1 {
        return;
    }
    build_max_heap(xs);
    let mut heap_size = xs.len();

    for i in (1..xs.len()).rev() {
        xs.swap(0, i);
        heap_size -= 1;
        max_heapify_iterative(xs, 0, heap_size);
    }
}

fn build_max_heap<T: Ord + Copy>(xs: &mut [T]) {
    for i in (0..=parent(xs.len() - 1)).rev() {
        max_heapify_iterative(xs, i, xs.len());
    }
}

#[allow(dead_code)]
// sink element recursively
fn max_heapify_recursive<T: Ord + Copy>(xs: &mut [T], i: usize, heap_size: usize) {
    let l = left(i);
    let r = right(i);
    let mut largest = i;
    if l < heap_size && xs[l] > xs[largest] {
        largest = l;
    }
    if r < heap_size && xs[r] > xs[largest] {
        largest = r;
    }
    if largest != i {
        xs.swap(i, largest);
        max_heapify_recursive(xs, largest, heap_size);
    }
}

// sink element iteratively
fn max_heapify_iterative<T: Ord + Copy>(xs: &mut [T], i: usize, heap_size: usize) {
    let mut i = i;
    while i < heap_size {
        let l = left(i);
        let r = right(i);
        let mut largest = i;
        if l < heap_size && xs[l] > xs[largest] {
            largest = l;
        }
        if r < heap_size && xs[r] > xs[largest] {
            largest = r;
        }
        if largest != i {
            xs.swap(i, largest);
            i = largest;
        } else {
            break;
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
    fn test_sort() {
        let mut xs = [4, 5, 1, 2, 4];
        heap_sort(&mut xs);
        assert_eq!(xs, [1, 2, 4, 4, 5]);
    }

    #[test]
    fn test_empty() {
        let mut xs: [i32; 0] = [];
        heap_sort(&mut xs);
        assert_eq!(xs, []);
    }

    #[test]
    fn test_one_element() {
        let mut xs = [4];
        heap_sort(&mut xs);
        assert_eq!(xs, [4]);
    }
}
