pub fn find_max_subarray_recursive(xs: &[i32]) -> (usize, usize, i32) {
    assert!(!xs.is_empty(), "xs must be not emplt");
    return find_max_subarray_recursive_2(xs, 0, xs.len() - 1);
}

fn find_max_subarray_recursive_2(xs: &[i32], low: usize, high: usize) -> (usize, usize, i32) {
    if high <= low {
        return (low, high, xs[low]);
    }

    let mid = low + (high - low) / 2;
    let (left_low, left_high, left_sum) = find_max_subarray_recursive_2(xs, low, mid);
    let (right_low, right_high, right_sum) = find_max_subarray_recursive_2(xs, mid + 1, high);
    let (cross_low, cross_high, cross_sum) = find_max_crossing_subarray(xs, low, mid, high);

    if left_sum >= right_sum && left_sum >= cross_sum {
        return (left_low, left_high, left_sum);
    } else if right_sum >= left_sum && right_sum >= cross_sum {
        return (right_low, right_high, right_sum);
    } else {
        return (cross_low, cross_high, cross_sum);
    }
}

fn find_max_crossing_subarray(
    xs: &[i32],
    low: usize,
    mid: usize,
    high: usize,
) -> (usize, usize, i32) {
    let mut max_left = mid;
    let mut max_right = mid;

    let mut sum = 0i32;
    let mut left_sum = xs[mid];
    for i in (low..=mid).rev() {
        sum += xs[i];
        if sum > left_sum {
            left_sum = sum;
            max_left = i;
        }
    }

    sum = 0;
    let mut right_sum = xs[mid];
    for i in mid + 1..=high {
        sum += xs[i];
        if sum > right_sum {
            right_sum = sum;
            max_right = i;
        }
    }

    return (max_left, max_right, left_sum + right_sum);
}

pub fn find_max_subarray_iterative(xs: &[i32]) -> (usize, usize, i32) {
    assert!(!xs.is_empty(), "xs must be not emplt");
    let mut left = 0;
    let mut right = 0;
    let mut sum = xs[0];
    let mut temp_left = 0;
    let mut temp_sum = xs[0];

    for i in 1..xs.len() {
        temp_sum += xs[i];
        if temp_sum > sum {
            sum = temp_sum;
            left = temp_left;
            right = i;
        }
        if temp_sum < 0 {
            temp_sum = 0;
            temp_left = i + 1;
        }
    }

    return (left, right, sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_subarray_recursive() {
        let xs = [
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
        ];
        let (left, right, sum) = find_max_subarray_recursive(&xs);
        assert_eq!(left, 7);
        assert_eq!(right, 10);
        assert_eq!(sum, 43);
    }

    #[test]
    fn test_find_max_subarray_iterative() {
        let xs = [
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
        ];
        let (left, right, sum) = find_max_subarray_iterative(&xs);
        assert_eq!(left, 7);
        assert_eq!(right, 10);
        assert_eq!(sum, 43);
    }
}
