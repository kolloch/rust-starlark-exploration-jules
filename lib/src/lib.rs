//! This is a library crate that provides shared functionality.

/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = lib::add(2, 2);
/// assert_eq!(result, 4);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    proptest! {
        #[test]
        fn test_add_property(a in 0..1000usize, b in 0..1000usize) {
            let result = add(a, b);
            assert!(result >= a);
            assert!(result >= b);
        }
    }
}
