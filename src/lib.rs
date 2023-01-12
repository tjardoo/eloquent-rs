pub use eloquent_core::CounterBuilder;

pub fn random_logic(init_value: i32, add_value: i32, sub_value: i32) -> i32 {
    let total = CounterBuilder::new()
        .init_value(init_value)
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
        let result = random_logic(0, 200, 150);

        assert_eq!(result, 50);

        let result = random_logic(100, -500, -200);

        assert_eq!(result, -200);
    }
}
