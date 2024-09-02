use crate::{Columnable, Function, QueryBuilder, Select, Selectable, ToSql};

impl QueryBuilder {
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

    pub fn select_as<T>(mut self, column: T, alias: &str) -> Self
    where
        T: Selectable,
    {
        self.selects.push(Select {
            function: None,
            column: column.to_select_column(),
            alias: Some(alias.to_string()),
        });

        self
    }

    pub fn select_raw(mut self, raw: &str, values: Vec<impl ToSql + 'static>) -> Self {
        let mut formatted_raw = raw.to_string();
        for value in values {
            formatted_raw = formatted_raw.replacen('?', &value.to_sql().unwrap(), 1);
        }

        self.selects.push(Select {
            function: None,
            column: formatted_raw.to_string(),
            alias: None,
        });

        self
    }

    pub fn select_count<T>(mut self, column: T, alias: &str) -> Self
    where
        T: Selectable,
    {
        self.selects.push(Select {
            function: Some(Function::Count),
            column: column.to_select_column(),
            alias: Some(alias.to_string()),
        });

        self
    }

    pub fn select_min<T>(mut self, column: T, alias: &str) -> Self
    where
        T: Selectable,
    {
        self.selects.push(Select {
            function: Some(Function::Min),
            column: column.to_select_column(),
            alias: Some(alias.to_string()),
        });

        self
    }

    pub fn select_max<T>(mut self, column: T, alias: &str) -> Self
    where
        T: Selectable,
    {
        self.selects.push(Select {
            function: Some(Function::Max),
            column: column.to_select_column(),
            alias: Some(alias.to_string()),
        });

        self
    }

    pub fn select_avg<T>(mut self, column: T, alias: &str) -> Self
    where
        T: Selectable,
    {
        self.selects.push(Select {
            function: Some(Function::Avg),
            column: column.to_select_column(),
            alias: Some(alias.to_string()),
        });

        self
    }

    pub fn select_sum<T>(mut self, column: T, alias: &str) -> Self
    where
        T: Selectable,
    {
        self.selects.push(Select {
            function: Some(Function::Sum),
            column: column.to_select_column(),
            alias: Some(alias.to_string()),
        });

        self
    }

    pub fn select_distinct<T>(mut self, column: T) -> Self
    where
        T: Selectable,
    {
        self.selects.push(Select {
            function: Some(Function::Distinct),
            column: column.to_select_column(),
            alias: None,
        });

        self
    }
}
