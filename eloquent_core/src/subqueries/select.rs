use crate::{interpolate_raw, Columnable, Function, Select, SubqueryBuilder, ToSql};

impl SubqueryBuilder {
    pub fn select<T>(mut self, columns: T) -> Self
    where
        T: Columnable,
    {
        let columns = columns.to_columns();

        for column in columns.iter() {
            self.selects.push(Select {
                function: None,
                column: column.to_string(),
                alias: None,
            });
        }

        self
    }

    pub fn select_as(mut self, column: &str, alias: &str) -> Self {
        self.selects.push(Select {
            function: None,
            column: column.to_string(),
            alias: Some(alias.to_string()),
        });

        self
    }

    pub fn select_raw(mut self, raw: &str, values: Vec<impl ToSql + 'static>) -> Self {
        match interpolate_raw(raw, values) {
            Ok(column) => self.selects.push(Select {
                function: None,
                column,
                alias: None,
            }),
            Err(error) => {
                self.error.get_or_insert(error);
            }
        }

        self
    }

    pub fn select_count(mut self, column: &str, alias: &str) -> Self {
        self.selects.push(Select {
            function: Some(Function::Count),
            column: column.to_string(),
            alias: Some(alias.to_string()),
        });

        self
    }

    pub fn select_min(mut self, column: &str, alias: &str) -> Self {
        self.selects.push(Select {
            function: Some(Function::Min),
            column: column.to_string(),
            alias: Some(alias.to_string()),
        });

        self
    }

    pub fn select_max(mut self, column: &str, alias: &str) -> Self {
        self.selects.push(Select {
            function: Some(Function::Max),
            column: column.to_string(),
            alias: Some(alias.to_string()),
        });

        self
    }

    pub fn select_avg(mut self, column: &str, alias: &str) -> Self {
        self.selects.push(Select {
            function: Some(Function::Avg),
            column: column.to_string(),
            alias: Some(alias.to_string()),
        });

        self
    }

    pub fn select_sum(mut self, column: &str, alias: &str) -> Self {
        self.selects.push(Select {
            function: Some(Function::Sum),
            column: column.to_string(),
            alias: Some(alias.to_string()),
        });

        self
    }

    pub fn select_distinct(mut self, column: &str) -> Self {
        self.selects.push(Select {
            function: None,
            column: column.to_string(),
            alias: None,
        });

        self.distinct()
    }

    pub fn distinct(mut self) -> Self {
        self.distinct = true;

        self
    }
}
