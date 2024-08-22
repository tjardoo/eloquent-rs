use crate::{Order, OrderColumn, QueryBuilder};

impl QueryBuilder {
    pub fn order_by_asc(mut self, column: &str) -> Self {
        self.order_by.push(OrderColumn {
            column: column.to_string(),
            order: Order::Asc,
        });

        self
    }

    pub fn order_by_desc(mut self, column: &str) -> Self {
        self.order_by.push(OrderColumn {
            column: column.to_string(),
            order: Order::Desc,
        });

        self
    }
}
