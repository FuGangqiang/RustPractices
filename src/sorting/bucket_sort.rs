use crate::sorting::insert_sort::insert_sort;

pub fn bucket_sort(xs: &mut [usize]) {
    if xs.is_empty() {
        return;
    }
    let total_size = xs.iter().max().unwrap() + 1;
    let bucket_step = 3;
    let buckets_size = (total_size + bucket_step - 1) / bucket_step;

    let mut buckets: Vec<Vec<usize>> = vec![vec![]; buckets_size];
    for &x in xs.iter() {
        buckets[x / bucket_step].push(x);
    }
    for i in 0..buckets_size {
        insert_sort(&mut buckets[i]);
    }
    let mut j = 0;
    for bucket in buckets {
        for item in bucket {
            xs[j] = item;
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut xs = [4, 5, 1, 2, 4];
        bucket_sort(&mut xs);
        assert_eq!(xs, [1, 2, 4, 4, 5]);
    }

    #[test]
    fn test_empty() {
        let mut xs: [usize; 0] = [];
        bucket_sort(&mut xs);
        assert_eq!(xs, []);
    }

    #[test]
    fn test_one_element() {
        let mut xs = [4];
        bucket_sort(&mut xs);
        assert_eq!(xs, [4]);
    }
}
