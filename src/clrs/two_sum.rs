use std::collections::BTreeMap;

pub fn two_sum(xs: &[i32], sum: i32) -> Option<(usize, usize)> {
    let mut map = BTreeMap::new();

    for (i, x) in xs.iter().enumerate() {
        map.insert(x, i);
    }

    for (i, x) in xs.iter().enumerate() {
        let searched = sum - x;
        if let Some(&j) = map.get(&searched) {
            if j != i {
                return Some((i, j));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let xs = [2, 7, 11, 15];
        let result = two_sum(&xs, 9);
        assert!(result == Some((1, 0)) || result == Some((0, 1)));
    }
}
