pub fn shell_sort<T: Ord + Copy>(xs: &mut [T]) {
    if xs.is_empty() {
        return;
    }
    const STEP: usize = 3;
    let mut h: usize;
    let mut temp: usize = 1;
    loop {
        h = temp;
        temp = STEP * temp + 1;

        if temp > xs.len() {
            break;
        }
    }

    while h >= 1 {
        for i in h..xs.len() {
            for j in (h..=i).step_by(h) {
                if xs[j] < xs[j - h] {
                    xs.swap(j, j - h);
                }
            }
        }
        h /= STEP;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut xs = [4, 5, 1, 2, 4];
        shell_sort(&mut xs);
        assert_eq!(xs, [1, 2, 4, 4, 5]);
    }

    #[test]
    fn test_empty() {
        let mut xs: [i32; 0] = [];
        shell_sort(&mut xs);
        assert_eq!(xs, []);
    }

    #[test]
    fn test_one_element() {
        let mut xs = [4];
        shell_sort(&mut xs);
        assert_eq!(xs, [4]);
    }
}
