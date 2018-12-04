pub fn radix_sort(xs: &mut [usize]) {
    if xs.is_empty() {
        return;
    }
    let radix: usize = 10;
    let &max = xs.iter().max().unwrap();
    let k = (max as f64).log(radix as f64).ceil() as u32;

    for i in 1..=k {
        let mut buckets: Vec<Vec<usize>> = vec![vec![]; radix];

        for &x in xs.iter() {
            let i = x % radix.pow(i) / radix.pow(i - 1);
            buckets[i].push(x);
        }

        let mut flattened = buckets.into_iter().flatten().collect::<Vec<usize>>();
        xs.swap_with_slice(&mut flattened[..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut xs = [73, 22, 93, 43, 55, 14, 28, 65, 39, 81];
        radix_sort(&mut xs);
        assert_eq!(xs, [14, 22, 28, 39, 43, 55, 65, 73, 81, 93]);
    }

    #[test]
    fn test_empty() {
        let mut xs: [usize; 0] = [];
        radix_sort(&mut xs);
        assert_eq!(xs, []);
    }

    #[test]
    fn test_one_element() {
        let mut xs = [4];
        radix_sort(&mut xs);
        assert_eq!(xs, [4]);
    }
}
