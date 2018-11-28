pub fn insert_sort<T: Ord + Copy>(xs: &mut [T]) {
    for j in 1..xs.len() {
        let key = xs[j];
        let mut i = j;
        while i > 0 && xs[i - 1] > key {
            xs[i] = xs[i - 1];
            i -= 1;
        }
        xs[i] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut xs = [4, 5, 1, 2, 4];
        insert_sort(&mut xs);
        assert_eq!(xs, [1, 2, 4, 4, 5]);
    }

    #[test]
    fn test_empty() {
        let mut xs: [i32; 0] = [];
        insert_sort(&mut xs);
        assert_eq!(xs, []);
    }

    #[test]
    fn test_one_element() {
        let mut xs = [4];
        insert_sort(&mut xs);
        assert_eq!(xs, [4]);
    }
}
