
pub fn uncompress(array_in: usize, array_out: usize) -> usize {
    array_in + array_out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = uncompress(2, 2);
        assert_eq!(result, 4);
    }
}
