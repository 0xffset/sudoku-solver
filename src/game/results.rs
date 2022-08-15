use super::value::Value;

#[derive(PartialEq)]
pub enum AddResult {
    Added(Value),
    NoneValue,
    NotPossible,
    AlreadySet,
    Solved,
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
    Solved,
}

pub enum SolveResult {
    Solved,
    Failed,
}
