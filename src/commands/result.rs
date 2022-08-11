use crate::game::Value;

pub enum CommandResult {
    ParseError,

    AddCommandSuccess(Value, usize, usize),
    AddCommandNoneValue,
    AddCommandNotPossible,
    AddCommandAlreadySet,

    RemoveCommandSuccess(Value, usize, usize),
    RemoveCommandNoneValue,

    ChangeCommandSuccess(Value, Value, usize, usize),
    ChangeCommandNoneValue,
    ChangeCommandNotPossible,
}
