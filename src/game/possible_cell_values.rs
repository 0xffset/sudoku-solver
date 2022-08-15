use super::Value;

pub struct PossibleCellValues {
    pub row: usize,
    pub col: usize,
    pub values: Vec<Value>,
}

impl PartialEq for PossibleCellValues {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col && self.values == other.values
    }
}

impl Eq for PossibleCellValues {}

impl PartialOrd for PossibleCellValues {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PossibleCellValues {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.values.len().cmp(&self.values.len())
    }
}