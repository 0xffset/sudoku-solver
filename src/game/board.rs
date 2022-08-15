use std::fmt::Display;

use super::{
    cell::Cell,
    results::{AddResult, ChangeResult, RemoveResult},
    value::Value,
};

/// SudokuBoard.0\[row]\[col]
pub struct SudokuBoard {
    pub board: [[Cell; 9]; 9],
    pub indicator: bool,
}

impl SudokuBoard {
    /// Creates a new sudoku board from a vector of strings, where each string is a row of the board. <br>
    /// If the vector is empty, a blank board is created.
    pub fn new<S: AsRef<str>>(lines: Vec<S>) -> Self {
        if lines.len() > 9 {
            panic!("Input file must have a maximum of 9 lines");
        }

        let mut sudoku_board = SudokuBoard {
            board: [[Cell::new(); 9]; 9],
            indicator: true,
        };

        for (row, line) in lines.iter().enumerate() {
            let line_split = line.as_ref().split("|").collect::<Vec<&str>>();
            if line_split.len() > 9 {
                panic!("Each line must have a maximum of 9 columns");
            }

            for (col, &c) in line_split.iter().enumerate() {
                if let AddResult::Added(v) = sudoku_board.add(row + 1, col + 1, c.trim()) {
                    if v != Value::None {
                        sudoku_board.board[row][col].set_immutable();
                    }
                }
            }
        }

        sudoku_board
    }

    /// Adds a value to the board. <br>
    /// `row` and `col` bounds are 1..=9.
    pub fn add<S: AsRef<str>>(&mut self, row: usize, col: usize, val: S) -> AddResult {
        let v = Value::from(val);
        let row = row - 1;
        let col = col - 1;

        // don't overwrite existing values and don't set to None
        if v != Value::None {
            if self.board[row][col].value == Value::None {
                // don't set value if not possible
                if self.board[row][col].possible_values.contains(&v) {
                    self.board[row][col].value = v;
                    self.board[row][col].remove_possible_value(v);

                    // update rows and cols
                    for i in 0..9 {
                        self.board[i][col].remove_possible_value(v);
                        self.board[row][i].remove_possible_value(v);
                    }

                    // update 3x3 square
                    let row_off = row / 3;
                    let col_off = col / 3;
                    for row_count in 0..3 {
                        for col_count in 0..3 {
                            self.board[row_off * 3 + row_count][col_off * 3 + col_count]
                                .remove_possible_value(v);
                        }
                    }

                    return AddResult::Added(v);
                }

                return AddResult::NotPossible;
            }

            return AddResult::AlreadySet;
        }

        AddResult::NoneValue
    }

    /// Checks if the value at the given row and column is possible.
    fn __value_is_possible(&self, row: usize, col: usize, val: Value) -> bool {
        // update rows and cols
        for i in 0..9 {
            if self.board[i][col].value == val {
                return false;
            }
            if self.board[row][i].value == val {
                return false;
            }
        }

        // update 3x3 square
        let row_off = row / 3;
        let col_off = col / 3;
        for row_count in 0..3 {
            for col_count in 0..3 {
                if self.board[row_off * 3 + row_count][col_off * 3 + col_count].value == val {
                    return false;
                }
            }
        }

        true
    }

    /// Removes a value from the board. <br>
    /// row and col bounds are 1..=9.
    pub fn remove(&mut self, row: usize, col: usize) -> RemoveResult {
        let row = row - 1;
        let col = col - 1;

        // don't remove if value is immutable (start value)
        if self.board[row][col].mutable {
            // don't remove if value is None
            if self.board[row][col].value != Value::None {
                // clear value and add it to possible values
                let v = self.board[row][col].value;
                self.board[row][col].value = Value::None;
                self.board[row][col].add_possible_value(v);

                // update rows and cols
                for i in 0..9 {
                    if self.__value_is_possible(i, col, v) {
                        self.board[i][col].add_possible_value(v);
                    }
                    if self.__value_is_possible(row, i, v) {
                        self.board[row][i].add_possible_value(v);
                    }
                }

                // update 3x3 square
                let row_off = row / 3;
                let col_off = col / 3;
                for row_count in 0..3 {
                    for col_count in 0..3 {
                        let row_offset = row_off * 3 + row_count;
                        let col_offset = col_off * 3 + col_count;
                        if self.__value_is_possible(row_offset, col_offset, v) {
                            self.board[row_offset][col_offset]
                                .add_possible_value(v);
                        }
                    }
                }

                return RemoveResult::Removed(v);
            }

            return RemoveResult::NoneValue;
        }

        RemoveResult::Immutable
    }

    /// Changes the value of a cell. <br>
    /// row and col bounds are 1..=9.
    pub fn change<S: AsRef<str>>(&mut self, row: usize, col: usize, val: S) -> ChangeResult {
        match self.remove(row, col) {
            RemoveResult::Removed(rem_v) => match self.add(row, col, val) {
                AddResult::Added(add_v) => ChangeResult::Changed(rem_v, add_v),
                AddResult::AlreadySet | AddResult::NoneValue => panic!("Impossible program state"),
                AddResult::NotPossible => {
                    // if adding failed because the new value is not legal, restore the old value
                    self.add(row, col, rem_v.to_string());
                    ChangeResult::NotPossible
                }
            },
            RemoveResult::NoneValue => ChangeResult::NoneValue,
            RemoveResult::Immutable => ChangeResult::Immutable,
        }
    }

    /// Returns a string representation of the possible values of a cell for the values val_off1 through val_off3. <br>
    fn __get_print_row_values(
        &self,
        cell: Cell,
        val_off1: usize,
        val_off2: usize,
        val_off3: usize,
    ) -> String {
        if cell.value == Value::None && self.indicator {
            format!(
                "{} {} {}",
                cell.possible_values[val_off1].sup_str(),
                cell.possible_values[val_off2].sup_str(),
                cell.possible_values[val_off3].sup_str()
            )
        } else if val_off1 == 3 {
            // print the value if on the middle row
            format!("  {}  ", cell.value)
        } else {
            "     ".to_string()
        }
    }

    /// prints a row. <br>
    fn __print_row(
        &self,
        f: &mut std::fmt::Formatter,
        i: usize,
        row: [Cell; 9],
    ) -> std::fmt::Result {
        for (val_off1, val_off2, val_off3) in [(0, 1, 2), (3, 4, 5), (6, 7, 8)] {
            let border = if val_off1 == 3 {
                format!("{}", i)
            } else {
                "║".to_string()
            };

            write!(f, "{border}")?;

            for (row1, row2, row3) in [(0, 1, 2), (3, 4, 5), (6, 7, 8)] {
                write!(
                    f,
                    " {} │ {} │ {} ║",
                    self.__get_print_row_values(row[row1], val_off1, val_off2, val_off3),
                    self.__get_print_row_values(row[row2], val_off1, val_off2, val_off3),
                    self.__get_print_row_values(row[row3], val_off1, val_off2, val_off3)
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

macro_rules! print_border {
    ($f:ident, head) => {
        writeln!(
            $f,
            "╔══ 1 ══╤══ 2 ══╤══ 3 ══╦══ 4 ══╤══ 5 ══╤══ 6 ══╦══ 7 ══╤══ 8 ══╤══ 9 ══╗"
        )?;
    };
    ($f:ident, thin) => {
        writeln!(
            $f,
            "╟───────┼───────┼───────╫───────┼───────┼───────╫───────┼───────┼───────╢"
        )?;
    };
    ($f:ident, thick) => {
        writeln!(
            $f,
            "╠═══════╪═══════╪═══════╬═══════╪═══════╪═══════╬═══════╪═══════╪═══════╣"
        )?;
    };
    ($f:ident, tail) => {
        write!(
            $f,
            "╚═══════╧═══════╧═══════╩═══════╧═══════╧═══════╩═══════╧═══════╧═══════╝"
        )?;
    };
}

impl Display for SudokuBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        print_border!(f, head);
        self.__print_row(f, 1, self.board[0])?;
        print_border!(f, thin);
        self.__print_row(f, 2, self.board[1])?;
        print_border!(f, thin);
        self.__print_row(f, 3, self.board[2])?;
        print_border!(f, thick);
        self.__print_row(f, 4, self.board[3])?;
        print_border!(f, thin);
        self.__print_row(f, 5, self.board[4])?;
        print_border!(f, thin);
        self.__print_row(f, 6, self.board[5])?;
        print_border!(f, thick);
        self.__print_row(f, 7, self.board[6])?;
        print_border!(f, thin);
        self.__print_row(f, 8, self.board[7])?;
        print_border!(f, thin);
        self.__print_row(f, 9, self.board[8])?;
        print_border!(f, tail);
        Ok(())
    }
}
