pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn subtract(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_add() {
        let result = subtract(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn it_can_subtract() {
        let result = subtract(5, 3);
        assert_eq!(result, 2);
    }
}
