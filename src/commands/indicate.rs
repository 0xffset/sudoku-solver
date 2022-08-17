use super::{Command, CommandResult};

pub struct IndicateCommand;

impl Command for IndicateCommand {
    fn name(&self) -> &'static str {
        "i"
    }

    fn num_args(&self) -> usize {
        0
    }

    fn usage(&self) -> &'static str {
        "Usage: `i`"
    }

    fn description(&self) -> &'static str {
        "Toggles the indicators of available values"
    }

    fn execute(&self, board: &mut crate::game::SudokuBoard, _: Vec<&str>) -> CommandResult {
        let setting = !board.indicator;
        board.indicator = setting;
        CommandResult::IndicateCommandSuccess(setting)
    }
}
