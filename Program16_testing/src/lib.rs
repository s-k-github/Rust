pub mod equal_test;
mod greetings_test;
mod show_output;
pub mod test;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn failing_tests() {
        panic!("Oops! I panicked")
    }
}
