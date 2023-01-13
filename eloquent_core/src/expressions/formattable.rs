use crate::error::EloquentError;

pub trait Formattable {
    fn to_query_format(&self) -> Result<String, EloquentError>;
}
