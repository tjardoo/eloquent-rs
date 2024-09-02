use crate::SubqueryBuilder;

impl SubqueryBuilder {
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);

        self
    }
}
