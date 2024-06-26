use std::fs;

/**
 * A Sudoku space value
 * 
 * Known with a digit,
 * Unknown with a vector of possible tests
 */
#[derive(PartialEq)]
enum SudokuValue {
    Known(usize),
    Unknown(Vec<usize>)
}

/**
 * A Sudoku board. Contains the sudoku spaces in a 2D vector, and the
 * number of emtpy spaces. An empty_spaces option of None means it's not
 * initalized. A value of Some(0) means the Sudoku is complete
 */
struct SudokuBoard {
    spaces: Vec<Vec<SudokuValue>>,
    empty_spaces: Option<usize>
}

impl SudokuBoard {
    fn new() -> SudokuBoard {
        // Make it the sudoku file without crashing
    }
}

fn main() {
    let a = SudokuValue::Known(0);
    let b = SudokuValue::Known(1);
    let c = SudokuValue::Known(0);

    println!("{}", a == b);
    println!("{}", b == a);
    println!("{}", a == c);
    println!("{}", c == a);
    println!("{}", b == c);
    println!("{}", c == b);
}
