pub fn add(x: u32, y: u32) -> u32 {
    x + y
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_adds_two_positives() {
        assert_eq!(add(3, 4), 7)
    }
}
