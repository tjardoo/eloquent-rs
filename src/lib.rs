pub use eloquent_core::{add, subtract};

pub fn random_logic(start: usize, addition: usize, subtraction: usize) -> usize {
    let mut total = add(start, addition);

    total = subtract(total, subtraction);

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = random_logic(100, 50, 20);
        assert_eq!(result, 130);
    }
}
