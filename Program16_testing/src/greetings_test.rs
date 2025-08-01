#[cfg(test)]
mod greetings_test {

    fn add_two(a: i32) -> i32 {
        a + 2
    }

    // #[test]
    fn test_add_two() {
        let total: i32 = add_two(4);
        assert!(total == 6, "Total is not 5. It is = {}", total);
    }
}
