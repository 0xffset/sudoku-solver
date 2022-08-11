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

macro_rules! print_board {
    ($f:ident, $rows:ident) => {
        writeln!($f, "╔═══a═══╤═══b═══╤═══c═══╦═══d═══╤═══e═══╤═══f═══╦═══g═══╤═══h═══╤═══i═══╗")?;
        print_board!($f, 1, $rows[0], (0, 1, 2), (3, 4, 5), (6, 7, 8));
        writeln!($f, "╟───────┼───────┼───────╫───────┼───────┼───────╫───────┼───────┼───────╢")?;
        print_board!($f, 2, $rows[1], (0, 1, 2), (3, 4, 5), (6, 7, 8));
        writeln!($f, "╟───────┼───────┼───────╫───────┼───────┼───────╫───────┼───────┼───────╢")?;
        print_board!($f, 3, $rows[2], (0, 1, 2), (3, 4, 5), (6, 7, 8));
        writeln!($f, "╠═══════╪═══════╪═══════╬═══════╪═══════╪═══════╬═══════╪═══════╪═══════╣")?;
        print_board!($f, 4, $rows[3], (0, 1, 2), (3, 4, 5), (6, 7, 8));
        writeln!($f, "╟───────┼───────┼───────╫───────┼───────┼───────╫───────┼───────┼───────╢")?;
        print_board!($f, 5, $rows[4], (0, 1, 2), (3, 4, 5), (6, 7, 8));
        writeln!($f, "╟───────┼───────┼───────╫───────┼───────┼───────╫───────┼───────┼───────╢")?;
        print_board!($f, 6, $rows[5], (0, 1, 2), (3, 4, 5), (6, 7, 8));
        writeln!($f, "╠═══════╪═══════╪═══════╬═══════╪═══════╪═══════╬═══════╪═══════╪═══════╣")?;
        print_board!($f, 7, $rows[6], (0, 1, 2), (3, 4, 5), (6, 7, 8));
        writeln!($f, "╟───────┼───────┼───────╫───────┼───────┼───────╫───────┼───────┼───────╢")?;
        print_board!($f, 8, $rows[7], (0, 1, 2), (3, 4, 5), (6, 7, 8));
        writeln!($f, "╟───────┼───────┼───────╫───────┼───────┼───────╫───────┼───────┼───────╢")?;
        print_board!($f, 9, $rows[8], (0, 1, 2), (3, 4, 5), (6, 7, 8));
        writeln!($f, "╚═══════╧═══════╧═══════╩═══════╧═══════╧═══════╩═══════╧═══════╧═══════╝")?;
    };
    ($f:ident, $i:literal, $row:expr, $(($x:literal, $y:literal, $z:literal)),*) => {
        $(
            writeln!(
                $f,
                "{} {} │ {} │ {} ║ {} │ {} │ {} ║ {} │ {} │ {} ║",
                if $x == 3 { format!("{}", $i) } else { "║".to_string() },
                print_board!($row, $x, $y, $z, 0),
                print_board!($row, $x, $y, $z, 1),
                print_board!($row, $x, $y, $z, 2),
                print_board!($row, $x, $y, $z, 3),
                print_board!($row, $x, $y, $z, 4),
                print_board!($row, $x, $y, $z, 5),
                print_board!($row, $x, $y, $z, 6),
                print_board!($row, $x, $y, $z, 7),
                print_board!($row, $x, $y, $z, 8),
            )?;
        )*
    };

    ($row:expr, $x:literal, $y:literal, $z:literal, $i:literal) => {
        if $row[$i].value == Value::None {
            format!(
                "{} {} {}",
                $row[$i].possible_values[$x].sup_str(),
                $row[$i].possible_values[$y].sup_str(),
                $row[$i].possible_values[$z].sup_str()
            )
        } else if $x == 3 {
            format!("  {}  ", $row[$i].value)
        } else {
            "     ".to_string()
        }
    };
}

impl Display for SudokuBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let rows = self.0;
        print_board!(f, rows);
        Ok(())
    }
}
