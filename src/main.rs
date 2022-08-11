use board::SudokuBoard;

mod board;
mod cell;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <input file path>", args[0]);
        std::process::exit(1);
    }

    let input = std::fs::read_to_string(&args[1]).unwrap();
    let board = SudokuBoard::new(input.lines().collect());
    println!("{board}");
}
