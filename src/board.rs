use std::fmt::Display;

use crate::cell::{Cell, Value};

/// SudokuBoard.0\[row]\[col]
pub struct SudokuBoard([[Cell; 9]; 9]);

impl SudokuBoard {
    pub fn new<S: AsRef<str>>(lines: Vec<S>) -> Self {
        if lines.len() > 9 {
            panic!("Input file must have a maximum of 9 lines");
        }

        let mut board = [[Cell::new(); 9]; 9];
        for (i, line) in lines.iter().enumerate() {
            let line_split = line.as_ref().split("|").collect::<Vec<&str>>();
            if line_split.len() > 9 {
                panic!("Each line must have a maximum of 9 columns");
            }

            for (j, &c) in line_split.iter().enumerate() {
                let v = match c {
                    "1" => Value::One,
                    "2" => Value::Two,
                    "3" => Value::Three,
                    "4" => Value::Four,
                    "5" => Value::Five,
                    "6" => Value::Six,
                    "7" => Value::Seven,
                    "8" => Value::Eight,
                    "9" => Value::Nine,
                    _ => continue,
                };
                board[i][j] = Cell::from(v);
            }
        }

        // TODO: validate and calculate possible values for each cell
        SudokuBoard(board)
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

macro_rules! border {
    ($f:ident, head) => {
        writeln!(
            $f,
            "╔═══a═══╤═══b═══╤═══c═══╦═══d═══╤═══e═══╤═══f═══╦═══g═══╤═══h═══╤═══i═══╗"
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
        border!(f, head);
        print_row(f, 1, self.0[0])?;
        border!(f, thin);
        print_row(f, 2, self.0[1])?;
        border!(f, thin);
        print_row(f, 3, self.0[2])?;
        border!(f, thick);
        print_row(f, 4, self.0[3])?;
        border!(f, thin);
        print_row(f, 5, self.0[4])?;
        border!(f, thin);
        print_row(f, 6, self.0[5])?;
        border!(f, thick);
        print_row(f, 7, self.0[6])?;
        border!(f, thin);
        print_row(f, 8, self.0[7])?;
        border!(f, thin);
        print_row(f, 9, self.0[8])?;
        border!(f, tail);
        Ok(())
    }
}
