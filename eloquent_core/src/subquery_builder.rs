use crate::SubqueryBuilder;

impl SubqueryBuilder {
    pub fn new() -> Self {
        Self {
            table: None,
            selects: Vec::new(),
            conditions: Vec::new(),
            joins: Vec::new(),
            havings: Vec::new(),
            group_by: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    pub fn table(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());

        self
    }
}

impl Default for SubqueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
