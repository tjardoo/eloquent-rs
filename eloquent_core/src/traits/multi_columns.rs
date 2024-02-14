pub trait MultiColumns {
    fn to_columns(&self) -> Vec<String>;
}

impl MultiColumns for &str {
    fn to_columns(&self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl MultiColumns for Vec<&str> {
    fn to_columns(&self) -> Vec<String> {
        self.iter().map(|&s| s.to_string()).collect()
    }
}
