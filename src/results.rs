use crate::value::Value;

pub enum AddResult {
    Added(Value),
    NoneValue,
    NotPossible,
    AlreadySet,
}

pub enum RemoveResult {
    Removed(Value),
    NoneValue,
}

pub enum ChangeResult {
    Changed(Value, Value),
    NoneValue,
    NotPossible,
}
