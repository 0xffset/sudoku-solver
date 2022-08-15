# Sudoku game and solver
## Usage
```bash
$ cargo run <optional board file>
```
e.g. `cargo run setup.txt` or `cargo run`

## The setup file is a text file with the following format:
 - Each line is a row of the board
 - Each row a `|` separated list of the values in the row (1 through 9)
 - The file must not have more than 9 lines
 - A row must not have more than 9 by `|` separated values
 - Consecutive `||` are interpreted as empty cells
 - Lines can be empty
 - Rows must not have 9 values