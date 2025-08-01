#[cfg(test)]
mod show_output {

    fn print_passed_value(a: i32) -> i32 {
        println!("Passed : {}", a);
        10
    }

    #[test]
    fn test_pass() {
        let passed_value: i32 = print_passed_value(4);
        assert_eq!(10, passed_value);
    }
    #[test]
    fn test_fail() {
        let passed_value: i32 = print_passed_value(13);
        assert_eq!(11, passed_value);
    }
}
