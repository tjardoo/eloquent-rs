pub struct CounterBuilder {
    count: i32,
}

impl CounterBuilder {
    pub fn new() -> CounterBuilder {
        CounterBuilder {
            count: 0,
        }
    }

    pub fn add(&mut self, value: i32) -> &mut CounterBuilder {
        self.count = self.count + value;

        self
    }

    pub fn subtract(&mut self, value: i32) -> &mut CounterBuilder {
        self.count = self.count - value;

        self
    }

    pub fn count(&self) -> i32 {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_add() {
        let result = CounterBuilder::new()
            .add(100)
            .count();

        assert_eq!(result, 100);
    }

    #[test]
    fn it_can_subtract() {
        let result = CounterBuilder::new()
            .subtract(100)
            .count();

        assert_eq!(result, -100);
    }
}
