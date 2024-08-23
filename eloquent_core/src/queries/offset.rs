use crate::QueryBuilder;

impl QueryBuilder {
    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);

        self
    }
}
