use crate::game::{RemoveResult, SudokuBoard};

use super::{Command, CommandResult};

pub struct RemoveCommand;

impl Command for RemoveCommand {
    fn name(&self) -> &'static str {
        "remove"
    }

    fn num_args(&self) -> usize {
        2
    }

    fn usage(&self) -> &'static str {
        "Usage: `remove <row> <col>`"
    }

    fn execute(&self, board: &mut SudokuBoard, args: Vec<&str>) -> CommandResult {
        let row = match args[0].parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                return CommandResult::ParseError;
            }
        };
        let col = match args[1].parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                return CommandResult::ParseError;
            }
        };

        match board.remove(row, col) {
            RemoveResult::Removed(v) => {
                return CommandResult::RemoveCommandSuccess(v, row, col);
            }
            RemoveResult::NoneValue => {
                return CommandResult::RemoveCommandNoneValue;
            }
            RemoveResult::Immutable => {
                return CommandResult::RemoveCommandImmutable;
            }
        }
    }
}
