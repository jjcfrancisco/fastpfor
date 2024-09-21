use crate::Result;

// Finds the numbers not present in a set and generates its complement within a range.
fn negate(set: Vec<i32>, max_range: usize) -> Vec<i32> {
    let mut complement = vec![0; max_range - set.len()];

    let mut ce: i32 = 0; // complementary element
    let mut position: usize = 0;

    // Fill complement with numbers up to the values in the set
    for &v in &set {
        while ce < v {
            complement[position] = ce;
            position += 1;
            ce += 1;
        }
        ce += 1;
    }

    // Fill the remaining positions in the complement with remaining values of the complementary element
    while position < complement.len() {
        complement[position] = ce;
        position += 1;
        ce += 1;
    }

    complement // Return the resulting vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negate_basic() {
        let x = vec![1, 3, 5];
        let max = 6;
        let result = negate(x, max);
        let expected = vec![0, 2, 4];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_negate_empty_x() {
        let x = vec![];
        let max = 5;
        let result = negate(x, max);
        let expected = vec![0, 1, 2, 3, 4];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_negate_full_x() {
        let x = vec![0, 1, 2, 3, 4];
        let max = 5;
        let result = negate(x, max);
        let expected: Vec<i32> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_negate_large_max() {
        let x = vec![3, 7, 10];
        let max = 12;
        let result = negate(x, max);
        let expected = vec![0, 1, 2, 4, 5, 6, 8, 9, 11];
        assert_eq!(result, expected);
    }
}
