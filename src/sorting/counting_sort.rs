pub fn counting_sort(xs: &mut [usize]) {
    if xs.len() <= 1 {
        return;
    }
    let max = xs.iter().max().unwrap();
    let size = max + 1;

    let mut ys = vec![0; size];
    for &x in xs.iter() {
        ys[x] += 1;
    }
    let mut k = 0;
    for i in 0..size {
        for _ in 0..ys[i] {
            xs[k] = i;
            k += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut xs = [4, 5, 1, 2, 4];
        counting_sort(&mut xs);
        assert_eq!(xs, [1, 2, 4, 4, 5]);
    }

    #[test]
    fn test_empty() {
        let mut xs: [usize; 0] = [];
        counting_sort(&mut xs);
        assert_eq!(xs, []);
    }

    #[test]
    fn test_one_element() {
        let mut xs = [4];
        counting_sort(&mut xs);
        assert_eq!(xs, [4]);
    }
}
