fn main() {
    println!("Hello Universe!");
}

/// Adds the given numbers.
///
/// # Examples
///
/// ```
/// use crate2;
///
/// let x = 5;
/// let y = 7;
///
/// assert_eq!(12, crate2::add(x, y));
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod fozzie_tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
