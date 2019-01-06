fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod main_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_main_add() {
        assert_eq!((1 + 2), 3);
    }
}
