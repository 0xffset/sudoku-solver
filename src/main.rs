use std::io::Write;

use board::SudokuBoard;

mod board;
mod cell;

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
        if input_split[0] == "add" {
            if input_split.len() == 4 {
                let row = match input_split[1].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Usage: add <row> <col> <val>");
                        continue;
                    }
                };
                let col = match input_split[2].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Usage: add <row> <col> <val>");
                        continue;
                    }
                };
                let val = input_split[3];
                board.add(row, col, val);
            } else {
                println!("Usage: add <row> <col> <val>");
                continue;
            }
        } else if input_split[0] == "remove" {
            if input_split.len() == 3 {
                let row = match input_split[1].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Usage: remove <row> <col>");
                        continue;
                    }
                };
                let col = match input_split[2].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Usage: remove <row> <col>");
                        continue;
                    }
                };
                board.remove(row, col);
            } else {
                println!("Usage: remove <row> <col>");
                continue;
            }
        } else if input_split[0] == "change" {
            if input_split.len() == 4 {
                let row = match input_split[1].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Usage: change <row> <col> <val>");
                        continue;
                    }
                };
                let col = match input_split[2].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Usage: change <row> <col> <val>");
                        continue;
                    }
                };
                let val = input_split[3];
                board.change(row, col, val);
            } else {
                println!("Usage: change <row> <col> <val>");
                continue;
            }
        }

        println!("{board}");
    }

    Ok(())
}
