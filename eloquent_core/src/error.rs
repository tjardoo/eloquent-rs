#[derive(Debug, PartialEq)]
pub enum EloquentError {
    MissingTableNameError,
    SelectBindingWithoutTableNameError,
    CombinationSelectAndInsertBindingError,
    MissingSelectAndInsertBindingError,
}
