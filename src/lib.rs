pub use eloquent_core::CounterBuilder;

pub fn random_logic(add_value: i32, sub_value: i32) -> i32 {
    let total = CounterBuilder::new()
        .add(add_value)
        .subtract(sub_value)
        .count();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = random_logic(200, 150);

        assert_eq!(result, 50);

        let result = random_logic(-500, -200);

        assert_eq!(result, -300);
    }
}
