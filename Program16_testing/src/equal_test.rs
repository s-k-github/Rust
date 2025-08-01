#[cfg(test)]
mod equal_test {

    fn add_two(a: i32) -> i32 {
        a + 2
    }

    // #[test]
    fn test_add_two() {
        assert_eq!(5, add_two(3));
    }
    // #[test]
    fn test_nq() {
        assert_ne!(5, add_two(4));
    }
}
