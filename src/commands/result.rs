use crate::game::Value;

pub enum CommandResult {
    ParseError,

    AddCommandSuccess(Value, usize, usize),
    AddCommandNoneValue,
    AddCommandNotPossible,
    AddCommandAlreadySet,

    RemoveCommandSuccess(Value, usize, usize),
    RemoveCommandNoneValue,
    RemoveCommandImmutable,

    ChangeCommandSuccess(Value, Value, usize, usize),
    ChangeCommandNoneValue,
    ChangeCommandNotPossible,
    ChangeCommandImmutable,
}
