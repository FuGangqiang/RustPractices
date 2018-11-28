pub fn selection_sort<T: Ord + Copy>(xs: &mut [T]) {
    if xs.is_empty() {
        return;
    }
    for j in 0..xs.len() - 1 {
        let mut min = j;
        for i in j..xs.len() {
            if xs[i] < xs[min] {
                min = i;
            }
        }
        if min != j {
            xs.swap(min, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut xs = [4, 5, 1, 2, 4];
        selection_sort(&mut xs);
        assert_eq!(xs, [1, 2, 4, 4, 5]);
    }

    #[test]
    fn test_empty() {
        let mut xs: [i32; 0] = [];
        selection_sort(&mut xs);
        assert_eq!(xs, []);
    }

    #[test]
    fn test_one_element() {
        let mut xs = [4];
        selection_sort(&mut xs);
        assert_eq!(xs, [4]);
    }
}
