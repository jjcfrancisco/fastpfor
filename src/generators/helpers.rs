use crate::{Error, Result};
use rand::Rng;

fn generate_uniform_hash(n: usize, max_range: usize) -> Result<Vec<i32>> {
    if n > max_range {
        return Err(Error::MaxNotPossible("N > max, not possible".to_string()));
    }

    let mut rng = rand::thread_rng();
    let mut unique_numbers_unsorted = Vec::new();

    // Generate unique random numbers
    while unique_numbers_unsorted.len() < n {
        let v = rng.gen_range(0..max_range as i32);
        unique_numbers_unsorted.push(v);
    }

    // Collect and sort the unique numbers
    let mut unique_numbers: Vec<i32> = unique_numbers_unsorted.into_iter().collect();
    unique_numbers.sort();

    Ok(unique_numbers)
}

fn generate_uniform_bitmap(n: usize, max_range: usize) -> Result<Vec<i32>> {
    if n > max_range {
        return Err(Error::MaxNotPossible("N > max, not possible".to_string()));
    }

    let mut rng = rand::thread_rng();
    let mut unique_numbers: Vec<i32> = Vec::new();

    while unique_numbers.len() < n {
        let v = rng.gen_range(0..max_range as i32);
        if !unique_numbers.contains(&v) {
            unique_numbers.push(v);
        }
    }

    Ok(unique_numbers)
}

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

    #[test]
    fn test_generate_uniform_bitmap_success() {
        let result = generate_uniform_bitmap(5, 10);
        assert!(result.is_ok());
        let bitmap = result.unwrap();
        assert_eq!(bitmap.len(), 5);
        assert!(bitmap.iter().all(|&num| num < 10));
    }

    #[test]
    fn test_generate_uniform_bitmap_more_than_max() {
        let result = generate_uniform_bitmap(10, 5);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(
            error.to_string(),
            "MaxNotPossible(\"N > max, not possible\")"
        );
    }

    #[test]
    fn test_generate_uniform_bitmap_exactly_max() {
        let result = generate_uniform_bitmap(10, 10);
        assert!(result.is_ok());
        let bitmap = result.unwrap();
        assert_eq!(bitmap.len(), 10);
        assert!(bitmap.iter().all(|&num| num < 10));
    }

    #[test]
    fn test_generate_uniform_bitmap_empty() {
        let result = generate_uniform_bitmap(0, 10);
        assert!(result.is_ok());
        let bitmap = result.unwrap();
        assert!(bitmap.is_empty()); // Should return an empty vector
    }

    #[test]
    fn test_generate_uniform_hash_success() {
        let result = generate_uniform_hash(5, 10);
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert_eq!(hash.len(), 5);
        assert!(hash.iter().all(|&num| num >= 0 && num < 10));
        assert!(is_sorted(&hash));
    }

    #[test]
    fn test_generate_uniform_hash_more_than_max() {
        let result = generate_uniform_hash(10, 5);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(
            error.to_string(),
            "MaxNotPossible(\"N > max, not possible\")"
        );
    }

    #[test]
    fn test_generate_uniform_hash_exactly_max() {
        let result = generate_uniform_hash(7, 10);
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert_eq!(hash.len(), 7);
        assert!(hash.iter().all(|&num| num >= 0 && num < 10));
        assert!(is_sorted(&hash));
    }

    #[test]
    fn test_generate_uniform_hash_empty() {
        let result = generate_uniform_hash(0, 10);
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert!(hash.is_empty());
    }

    fn is_sorted(vec: &[i32]) -> bool {
        vec.windows(2).all(|w| w[0] <= w[1])
    }
}
