use super::value::Value;

pub enum AddResult {
    Added(Value),
    NoneValue,
    NotPossible,
    AlreadySet,
}

pub enum RemoveResult {
    Removed(Value),
    NoneValue,
    Immutable,
}

pub enum ChangeResult {
    Changed(Value, Value),
    NoneValue,
    NotPossible,
    Immutable,
}
