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
                            // value didn't lead to a solution, remove it from the possible values and continue with the next value.
                            board.remove(row, col);
                        }
                    },
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
            // impossible because the board is always in a valid state => always solvable
            SolveResult::Failed => panic!("Impossible program state"),
        }
    }
}