pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_overflow() {
        add(2147483647, 1);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(12, 1), 11);
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_subtract_overflow() {
        subtract(-2147483648, 1);
    }


    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 2), 4);
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_multiply_overflow() {
        multiply(2147483647, 2);
    }
}