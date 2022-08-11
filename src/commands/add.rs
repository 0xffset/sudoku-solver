use crate::game::{AddResult, SudokuBoard};

use super::{Command, CommandResult};

pub struct AddCommand;

impl Command for AddCommand {
    fn name(&self) -> &'static str {
        "add"
    }

    fn num_args(&self) -> usize {
        3
    }

    fn usage(&self) -> &'static str {
        "Usage: `add <row> <col> <val>`"
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

        match board.add(row, col, val) {
            AddResult::Added(v) => {
                return CommandResult::AddCommandSuccess(v, row, col);
            }
            AddResult::NoneValue => {
                return CommandResult::AddCommandNoneValue;
            }
            AddResult::NotPossible => {
                return CommandResult::AddCommandNotPossible;
            }
            AddResult::AlreadySet => {
                return CommandResult::AddCommandAlreadySet;
            }
        }
    }
}
