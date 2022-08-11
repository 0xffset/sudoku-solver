use std::fmt::Display;

use crate::cell::{Cell, Value};

/// SudokuBoard.0\[row]\[col]
pub struct SudokuBoard([[Cell; 9]; 9]);

impl SudokuBoard {
    pub fn new<S: AsRef<str>>(lines: Vec<S>) -> Self {
        if lines.len() > 9 {
            panic!("Input file must have a maximum of 9 lines");
        }

        let mut sudoku_board = SudokuBoard([[Cell::new(); 9]; 9]);
        for (row, line) in lines.iter().enumerate() {
            let line_split = line.as_ref().split("|").collect::<Vec<&str>>();
            if line_split.len() > 9 {
                panic!("Each line must have a maximum of 9 columns");
            }

            for (col, &c) in line_split.iter().enumerate() {
                sudoku_board.add(row + 1, col + 1, c.trim());
            }
        }

        sudoku_board
    }

    /// row and col bounds are 1..=9
    pub fn add<S: AsRef<str>>(&mut self, row: usize, col: usize, val: S) {
        let v = Value::from(val);
        let row = row - 1;
        let col = col - 1;

        if v != Value::None && self.0[row][col].value == Value::None {
            if self.0[row][col].possible_values.contains(&v) {
                self.0[row][col].value = v;
                self.0[row][col].remove_possible_value(v);

                for i in 0..9 {
                    self.0[i][col].remove_possible_value(v);
                    self.0[row][i].remove_possible_value(v);
                }
                let row_off = row / 3;
                let col_off = col / 3;
                for row_count in 0..3 {
                    for col_count in 0..3 {
                        self.0[row_off * 3 + row_count][col_off * 3 + col_count]
                            .remove_possible_value(v);
                        self.0[row_off * 3 + row_count][col_off * 3 + col_count]
                            .remove_possible_value(v);
                    }
                }
            }
        }
    }

    /// row and col bounds are 1..=9
    pub fn remove(&mut self, row: usize, col: usize) -> Value {
        let row = row - 1;
        let col = col - 1;

        if self.0[row][col].value != Value::None {
            let v = self.0[row][col].value;
            self.0[row][col].value = Value::None;
            self.0[row][col].add_possible_value(v);

            for i in 0..9 {
                self.0[i][col].add_possible_value(v);
                self.0[row][i].add_possible_value(v);
            }
            let row_off = row / 3;
            let col_off = col / 3;
            for row_count in 0..3 {
                for col_count in 0..3 {
                    self.0[row_off * 3 + row_count][col_off * 3 + col_count].add_possible_value(v);
                    self.0[row_off * 3 + row_count][col_off * 3 + col_count].add_possible_value(v);
                }
            }

            return v;
        }

        Value::None
    }

    /// row and col bounds are 1..=9
    pub fn change<S: AsRef<str>>(&mut self, row: usize, col: usize, val: S) {
        let v = self.remove(row, col);
        if v != Value::None {
            self.add(row, col, v.to_string());
        } else {
            self.add(row, col, val);
        }
    }
}

fn print_row(f: &mut std::fmt::Formatter, i: usize, row: [Cell; 9]) -> std::fmt::Result {
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
                row[row1].get_print_row_values(val_off1, val_off2, val_off3),
                row[row2].get_print_row_values(val_off1, val_off2, val_off3),
                row[row3].get_print_row_values(val_off1, val_off2, val_off3)
            )?;
        }
        writeln!(f)?;
    }

    Ok(())
}

macro_rules! print_border {
    ($f:ident, head) => {
        writeln!(
            $f,
            "╔═══1═══╤═══2═══╤═══3═══╦═══4═══╤═══5═══╤═══6═══╦═══7═══╤═══8═══╤═══9═══╗"
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
        writeln!(
            $f,
            "╚═══════╧═══════╧═══════╩═══════╧═══════╧═══════╩═══════╧═══════╧═══════╝"
        )?;
    };
}

impl Display for SudokuBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        print_border!(f, head);
        print_row(f, 1, self.0[0])?;
        print_border!(f, thin);
        print_row(f, 2, self.0[1])?;
        print_border!(f, thin);
        print_row(f, 3, self.0[2])?;
        print_border!(f, thick);
        print_row(f, 4, self.0[3])?;
        print_border!(f, thin);
        print_row(f, 5, self.0[4])?;
        print_border!(f, thin);
        print_row(f, 6, self.0[5])?;
        print_border!(f, thick);
        print_row(f, 7, self.0[6])?;
        print_border!(f, thin);
        print_row(f, 8, self.0[7])?;
        print_border!(f, thin);
        print_row(f, 9, self.0[8])?;
        print_border!(f, tail);
        Ok(())
    }
}
