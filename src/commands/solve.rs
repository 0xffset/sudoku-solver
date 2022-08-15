use super::Command;

pub struct SolveCommand;

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

    fn execute(&self, board: &mut crate::game::SudokuBoard, args: Vec<&str>) -> super::CommandResult {
        todo!()
    }
}