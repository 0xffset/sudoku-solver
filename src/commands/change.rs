use crate::game::{ChangeResult, SudokuBoard};

use super::{Command, CommandResult};

pub struct ChangeCommand;

impl Command for ChangeCommand {
    fn name(&self) -> &'static str {
        "c"
    }

    fn num_args(&self) -> usize {
        3
    }

    fn usage(&self) -> &'static str {
        "Usage: `c<row><col><val>`"
    }

    fn description(&self) -> &'static str {
        "Changes a value on the board"
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
        let val = args[2].clone();

        match board.change_str(row, col, val) {
            ChangeResult::Changed(rem_v, add_v) => {
                return CommandResult::ChangeCommandSuccess(rem_v, add_v, row, col);
            }
            ChangeResult::NoneValue => {
                return CommandResult::ChangeCommandNoneValue;
            }
            ChangeResult::NotPossible => {
                return CommandResult::ChangeCommandNotPossible;
            }
            ChangeResult::Immutable => {
                return CommandResult::ChangeCommandImmutable;
            }
            ChangeResult::Solved => {
                return CommandResult::ChangeCommandSolved;
            }
        }
    }
}
