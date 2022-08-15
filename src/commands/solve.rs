use core::panic;

use crate::game::{AddResult, PossibleCellValues, SolveResult, SudokuBoard};

use super::{Command, CommandResult};

pub struct SolveCommand;

impl SolveCommand {
    /// DFS algorithm to solve the board.
    fn solve(&self, board: &mut SudokuBoard) -> SolveResult {
        if let Some(PossibleCellValues { row, col, values }) = board.pop_possible_value() {
            for value in values {
                match board.add(row, col, value) {
                    AddResult::Solved => return SolveResult::Solved,
                    AddResult::Added(_) => match self.solve(board) {
                        SolveResult::Solved => return SolveResult::Solved,
                        SolveResult::Failed => {
                            // value didn't lead to a solution, remove it and continue with the next value.
                            board.remove(row, col);
                        }
                    },
                    // value was not possible, continue with the next value.
                    AddResult::NotPossible => continue,
                    _ => panic!("Impossible program state"),
                };
            }
        }

        if board.is_solved() {
            SolveResult::Solved
        } else {
            SolveResult::Failed
        }
    }
}

impl Command for SolveCommand {
    fn name(&self) -> &'static str {
        "solve"
    }

    fn num_args(&self) -> usize {
        0
    }

    fn usage(&self) -> &'static str {
        "Usage: `solve`"
    }

    fn description(&self) -> &'static str {
        "Solves the current board"
    }

    fn execute(&self, board: &mut SudokuBoard, _args: Vec<&str>) -> CommandResult {
        match self.solve(board) {
            SolveResult::Solved => CommandResult::SolveCommandSuccess,
            // only possible if the supplied config is invalid, boards made at
            // runtime are always in a valid state and thus solvable.
            SolveResult::Failed => CommandResult::SolveCommandFailure,
        }
    }
}
