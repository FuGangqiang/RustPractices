pub fn quick_sort<T: Ord + Copy>(xs: &mut [T]) {
    if xs.is_empty() {
        return;
    }
    quick_sort2(xs, 0, xs.len() - 1);
}

fn quick_sort2<T: Ord + Copy>(xs: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }
    let j = partition(xs, low, high);
    quick_sort2(xs, low, j - 1);
    quick_sort2(xs, j + 1, high);
}

fn partition<T: Ord + Copy>(xs: &mut [T], low: usize, high: usize) -> usize {
    let v = xs[low];
    let mut i = low;
    let mut j = high + 1;
    loop {
        loop {
            i += 1;
            if !(i < high && xs[i] < v) {
                break;
            }
        }

        loop {
            j -= 1;
            if !(j > low && xs[j] > v) {
                break;
            }
        }

        if i >= j {
            break;
        }

        xs.swap(i, j);
    }

    xs.swap(low, j);
    return j;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut xs = [4, 5, 1, 2, 4];
        quick_sort(&mut xs);
        assert_eq!(xs, [1, 2, 4, 4, 5]);
    }

    #[test]
    fn test_empty() {
        let mut xs: [i32; 0] = [];
        quick_sort(&mut xs);
        assert_eq!(xs, []);
    }

    #[test]
    fn test_one_element() {
        let mut xs = [4];
        quick_sort(&mut xs);
        assert_eq!(xs, [4]);
    }
}
