use crate::SudokuBoard;

use super::CommandResult;

pub trait Command {
    fn name(&self) -> &'static str;
    fn num_args(&self) -> usize;
    fn usage(&self) -> &'static str;
    fn execute(&self, board: &mut SudokuBoard, args: Vec<&str>) -> CommandResult;
}
