pub fn bubble_sort<T: Ord + Copy>(xs: &mut [T]) {
    for i in 0..xs.len() {
        for j in 0..xs.len() - i - 1 {
            if xs[j + 1] < xs[j] {
                xs.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut xs = [4, 5, 1, 2, 4];
        bubble_sort(&mut xs);
        assert_eq!(xs, [1, 2, 4, 4, 5]);
    }

    #[test]
    fn test_empty() {
        let mut xs: [i32; 0] = [];
        bubble_sort(&mut xs);
        assert_eq!(xs, []);
    }

    #[test]
    fn test_one_element() {
        let mut xs = [4];
        bubble_sort(&mut xs);
        assert_eq!(xs, [4]);
    }
}
