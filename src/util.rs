/// Take the square root of the given usize.
///
/// # Panics
///
/// Panics if the given number isn't a factor of 2.
#[inline(always)]
pub fn usize_sqrt(num: usize) -> usize {
    let sqrt = (num as f64).sqrt() as usize;
    assert_eq!(num, sqrt * sqrt, "given number isn't a multiple of 2");
    sqrt as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usize_sqrt_squared() {
        assert_eq!(usize_sqrt(0), 0);
        assert_eq!(usize_sqrt(1), 1);
        assert_eq!(usize_sqrt(4), 2);
        assert_eq!(usize_sqrt(25), 5);
    }

    /// Taking the integer square root of a number not being a multiple of 2 fails.
    #[test]
    #[should_panic]
    fn usize_sqrt_not_squared() {
        usize_sqrt(3);
    }
}
