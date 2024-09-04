use crate::SubqueryBuilder;

impl SubqueryBuilder {
    /// Create a new instance of the subquery builder.
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

    /// Set the table name for the subquery.
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
