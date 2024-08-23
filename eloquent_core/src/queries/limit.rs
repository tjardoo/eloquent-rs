use crate::QueryBuilder;

impl QueryBuilder {
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);

        self
    }
}
