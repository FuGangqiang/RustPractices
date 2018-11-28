pub fn merge_sort<T: Ord + Copy>(xs: &mut [T]) {
    if xs.is_empty() {
        return;
    }
    recurisive_sort(xs, 0, xs.len() - 1)
}

fn recurisive_sort<T: Ord + Copy>(xs: &mut [T], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    recurisive_sort(xs, lo, mid);
    recurisive_sort(xs, mid + 1, hi);
    merge(xs, lo, mid, hi);
}

fn merge<T: Ord + Copy>(xs: &mut [T], lo: usize, mid: usize, hi: usize) {
    let mut temp = vec![];
    temp.reserve_exact(hi - lo + 1);
    for k in lo..=hi {
        temp.push(xs[k])
    }

    let mut i = 0;
    let mut j = mid - lo + 1;
    for k in lo..=hi {
        if i > mid - lo {
            xs[k] = temp[j];
            j += 1;
        } else if j > hi - lo {
            xs[k] = temp[i];
            i += 1;
        } else if temp[j] < temp[i] {
            xs[k] = temp[j];
            j += 1;
        } else {
            xs[k] = temp[i];
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_sort() {
        let mut xs = [4, 5, 1, 2, 4];
        merge_sort(&mut xs);
        assert_eq!(xs, [1, 2, 4, 4, 5]);
    }

    #[test]
    fn test_even_sort() {
        let mut xs = [4, 5, 1, 2, 4, 2];
        merge_sort(&mut xs);
        assert_eq!(xs, [1, 2, 2, 4, 4, 5]);
    }

    #[test]
    fn test_empty() {
        let mut xs: [i32; 0] = [];
        merge_sort(&mut xs);
        assert_eq!(xs, []);
    }

    #[test]
    fn test_one_element() {
        let mut xs = [4];
        merge_sort(&mut xs);
        assert_eq!(xs, [4]);
    }
}
