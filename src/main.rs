#![feature(binary_heap_retain)]

use std::io::Write;

use crate::{
    commands::{
        AddCommand, ChangeCommand, Command, CommandResult, IndicateCommand, RemoveCommand,
        SolveCommand,
    },
    game::SudokuBoard,
};

mod commands;
mod game;

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let mut board = if args.len() == 2 {
        let input = std::fs::read_to_string(&args[1]).unwrap();
        SudokuBoard::new(input.lines().collect())
    } else {
        SudokuBoard::new(Vec::<String>::new())
    };

    let commands: Vec<Box<dyn Command>> = vec![
        Box::new(AddCommand {}),
        Box::new(ChangeCommand {}),
        Box::new(RemoveCommand {}),
        Box::new(IndicateCommand {}),
        Box::new(SolveCommand {}),
    ];

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

        if input == "help" {
            for command in &commands {
                println!(
                    "`{}` - {}\n{}",
                    command.name(),
                    command.description(),
                    command.usage()
                );
            }
            continue;
        }

        for command in &commands {
            if command.name() == input_split[0] {
                if command.num_args() <= input_split.len() - 1 {
                    match command.execute(&mut board, input_split[1..].to_vec()) {
                        CommandResult::ParseError => println!("{}", command.usage()),

                        CommandResult::AddCommandSuccess(v, row, col) => {
                            println!("Added {v} to {:?}", (row, col));
                            println!("{board}");
                        }
                        CommandResult::AddCommandNoneValue => println!("Can't add a 0"),
                        CommandResult::AddCommandNotPossible => println!("Illegal move"),
                        CommandResult::AddCommandAlreadySet => {
                            println!("To change a value, use the change command")
                        }
                        CommandResult::AddCommandSolved => {
                            println!("The board is solved");
                            println!("{board}");
                            break;
                        }

                        CommandResult::RemoveCommandSuccess(v, row, col) => {
                            println!("Removed {v} from {:?}", (row, col));
                            println!("{board}");
                        }
                        CommandResult::RemoveCommandNoneValue => {
                            println!("Can't remove an empty cell")
                        }
                        CommandResult::RemoveCommandImmutable => {
                            println!("Can't remove an immutable cell")
                        }

                        CommandResult::ChangeCommandSuccess(rem_v, add_v, row, col) => {
                            println!("Changed {rem_v} to {add_v} at {:?}", (row, col));
                            println!("{board}");
                        }
                        CommandResult::ChangeCommandNoneValue => {
                            println!("To add a value, use the add command")
                        }
                        CommandResult::ChangeCommandNotPossible => {
                            println!("Illegal move")
                        }
                        CommandResult::ChangeCommandImmutable => {
                            println!("Can't change an immutable cell")
                        }
                        CommandResult::ChangeCommandSolved => {
                            println!("The board is solved");
                            println!("{board}");
                            break;
                        }

                        CommandResult::SolveCommandSuccess => {
                            println!("{board}");
                            break;
                        }

                        CommandResult::IndicateCommandSuccess(_) => {
                            println!("{board}");
                        }
                    }
                } else {
                    println!("{}", command.usage());
                }
            }
        }
    }

    Ok(())
}
