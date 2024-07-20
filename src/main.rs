use std::time::Duration;
use std::{fs, io, time};

use rsudoku;
use rsudoku::SudokuBoard;

/// Solve all the sudoku boards in the `boards` dir
fn time_all_boards() -> io::Result<()> {
    for board_entry in fs::read_dir("boards")? {
        if let io::Result::Ok(dir_entry) = board_entry {
            let start = time::Instant::now();
            let board = rsudoku::solve(&dir_entry.file_name().into_string().unwrap());

            if board.is_ok() {
                let solved = board.unwrap().is_solved();

                let time_taken = start.elapsed();

                let solved_string: String;
                if solved {
                    solved_string = String::from("Solved")
                } else {
                    solved_string = String::from("Unsolveable")
                }

                println!(
                    "{}\t{} in {}us",
                    dir_entry.file_name().into_string().unwrap(),
                    solved_string,
                    time_taken.as_micros()
                );

            } else {
                eprintln!(
                    "Couldn't read {} due to {:?}",
                    dir_entry.file_name().into_string().unwrap(),
                    board.err().unwrap()
                )
            }
        } else {
            eprintln!("Encounted error!");
        }
    }

    return io::Result::Ok(());
}

/// Load a board from the `boards` dir and print the solution and how long
/// it took to solve, or prove unsolvable
///
/// Returns io::Result::Err if the board couldn't be loaded
fn time_solve(board_name: &str) -> io::Result<()> {
    let start = time::Instant::now();

    let board: SudokuBoard = rsudoku::solve(board_name)?;
    let is_solved = board.is_solved();

    let time_taken = start.elapsed();

    println!("Board {}", board_name);
    println!("{}", board);

    if is_solved {
        println!("Solved in {}us", time_taken.as_micros());
    } else {
        println!(
            "Found that no solutions exist is {}us",
            time_taken.as_micros()
        );
    }

    return io::Result::Ok(());
}

fn main() -> io::Result<()> {
    return time_all_boards();
}
