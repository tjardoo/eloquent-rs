use crate::error::EloquentError;

pub trait Formattable {
    fn is_used(&self) -> bool;
    fn to_query_format(&self) -> Result<String, EloquentError>;
}
