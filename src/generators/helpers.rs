use crate::{Error, Result};
use rand::Rng;

fn generate_set() -> Result<Vec<i32>> {
    let n = 128 * 1024;
    let max_range = 1 << 24;

    let mut rng = rand::thread_rng();
    let mut unique_numbers = Vec::new();

    while unique_numbers.len() < n {
        let v = rng.gen_range(0..max_range);
        unique_numbers.push(v);
    }

    Ok(unique_numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_set() {
        let set = generate_set().unwrap();
        assert_eq!(set.len(), 128 * 1024);
    }
}
