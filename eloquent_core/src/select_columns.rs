pub trait SelectColumns {
    fn to_columns(&self) -> Vec<String>;
}

impl SelectColumns for &str {
    fn to_columns(&self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl SelectColumns for Vec<&str> {
    fn to_columns(&self) -> Vec<String> {
        self.iter().map(|&s| s.to_string()).collect()
    }
}
