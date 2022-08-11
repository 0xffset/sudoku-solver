use std::io::Write;

use board::SudokuBoard;

mod board;
mod cell;
mod results;

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let mut board = if args.len() == 2 {
        let input = std::fs::read_to_string(&args[1]).unwrap();
        SudokuBoard::new(input.lines().collect())
    } else {
        SudokuBoard::new(Vec::<String>::new())
    };

    println!("{board}");

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    loop {
        print!(">");
        stdout.flush()?;
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" || input == "q" {
            break;
        }

        let input_split = input.split(" ").collect::<Vec<&str>>();
        if input_split.is_empty() {
            continue;
        }

        if input_split[0] == "add" {
            if input_split.len() == 4 {
                let row = match input_split[1].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Usage: `add <row> <col> <val>`");
                        continue;
                    }
                };
                let col = match input_split[2].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Usage: `add <row> <col> <val>`");
                        continue;
                    }
                };
                let val = input_split[3];

                match board.add(row, col, val) {
                    results::AddResult::Added(v) => {
                        println!("Added {v} to {:?}", (row, col));
                    }
                    results::AddResult::NoneValue => {
                        println!("Can't add a 0");
                        continue;
                    }
                    results::AddResult::NotPossible => {
                        println!("Illegal move");
                        continue;
                    }
                    results::AddResult::AlreadySet => {
                        println!("To change a value, use `change <row> <col> <val>`");
                        continue;
                    }
                }
            } else {
                println!("Usage: `add <row> <col> <val>`");
                continue;
            }
        } else if input_split[0] == "remove" {
            if input_split.len() == 3 {
                let row = match input_split[1].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Usage: `remove <row> <col>`");
                        continue;
                    }
                };
                let col = match input_split[2].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Usage: `remove <row> <col>`");
                        continue;
                    }
                };

                match board.remove(row, col) {
                    results::RemoveResult::Removed(v) => {
                        println!("Removed {v} from {:?}", (row, col));
                    }
                    results::RemoveResult::NoneValue => {
                        println!("Can't remove an empty cell");
                        continue;
                    }
                }
            } else {
                println!("Usage: `remove <row> <col>`");
                continue;
            }
        } else if input_split[0] == "change" {
            if input_split.len() == 4 {
                let row = match input_split[1].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Usage: `change <row> <col> <val>`");
                        continue;
                    }
                };
                let col = match input_split[2].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Usage: `change <row> <col> <val>`");
                        continue;
                    }
                };
                let val = input_split[3];

                match board.change(row, col, val) {
                    results::ChangeResult::Changed(rem_v, add_v) => {
                        println!("Changed {rem_v} to {add_v} at {:?}", (row, col));
                    }
                    results::ChangeResult::NoneValue => {
                        println!("To add a value, use `add <row> <col> <val>`");
                        continue;
                    }
                    results::ChangeResult::NotPossible => {
                        println!("Illegal move");
                        continue;
                    }
                }
            } else {
                println!("Usage: `change <row> <col> <val>`");
                continue;
            }
        }

        println!("{board}");
    }

    Ok(())
}
