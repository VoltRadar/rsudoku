// Clippy configurations
#![allow(clippy::needless_return)]

use std::{env, fs, io, time};

use rsudoku::SudokuBoard;

/// Solve all the sudoku boards in the `boards` dir
fn time_all_boards() -> io::Result<()> {
    let boards_result = fs::read_dir("boards");
    if boards_result.is_err() {
        let error = boards_result.err().unwrap();
        if error.kind() == io::ErrorKind::NotFound {
            eprintln!("`boards` dir doesn't exist");
            eprintln!("Run with a path as a argument solve a sudoku");
            return io::Result::Ok(());
        };

        return io::Result::Err(error);
    }

    for board_entry in boards_result.unwrap() {
        if let io::Result::Ok(dir_entry) = board_entry {
            let start = time::Instant::now();
            let board = rsudoku::solve(dir_entry.path().to_str().unwrap());

            if board.is_ok() {
                let solved = board.unwrap().is_solved();

                let time_taken = start.elapsed();

                let solved_string: String = if solved {
                    String::from("Solved")
                } else {
                    String::from("Unsolvable")
                };

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
            eprintln!("Encountered error!");
        }
    }

    return io::Result::Ok(());
}

/// Load a board from the `boards` dir and print the solution and how long
/// it took to solve, or prove unsolvable
///
/// Returns io::Result::Err if the board couldn't be loaded
fn time_solve(board_path: &str) -> io::Result<()> {
    let start = time::Instant::now();

    let board_result: Result<SudokuBoard, io::Error> = rsudoku::solve(board_path);
    if board_result.is_err() {
        let error = board_result.err().unwrap();
        match error.kind() {
            io::ErrorKind::NotFound => {
                eprintln!("Can't find board at {board_path}");
                return io::Result::Ok(());
            }
            _ => {
                return io::Result::Err(error);
            }
        }
    }

    let board = board_result.ok().unwrap();

    let is_solved = board.is_solved();

    let time_taken = start.elapsed();

    println!("Board {}", board_path);
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

#[allow(dead_code)]
fn time_single_board(path: &str) {
    const DELTA: time::Duration = time::Duration::from_secs(300);

    let start = time::Instant::now();
    let end = start + DELTA;

    let mut times = Vec::new();
    let mut count = 0;

    while time::Instant::now() < end {
        let start_solve = time::Instant::now();
        let result = rsudoku::solve(path).unwrap();
        let dur = start_solve.elapsed();

        if result.is_solved() {
            count += 1;
        }

        times.push(dur.as_micros());

        if count % 3000 == 0 {
            println!("{:?}", end - time::Instant::now())
        }
    }

    println!("Trials: {}", times.len());

    times.sort_unstable();

    let mut ten_quantiles = Vec::new();

    for quantile in 1..=9 {
        let split = (quantile as f64) / 10_f64;
        let index = (split * times.len() as f64).round() as usize;
        ten_quantiles.push(times[index]);
    }

    println!("10-quantiles: {:?}", ten_quantiles);

    println!("Medium: {}", ten_quantiles[4]);
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return time_all_boards();
    } else {
        return time_solve(args.get(1).unwrap());
    }
}
