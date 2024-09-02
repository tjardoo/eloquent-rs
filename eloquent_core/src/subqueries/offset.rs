use crate::SubqueryBuilder;

impl SubqueryBuilder {
    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);

        self
    }
}
