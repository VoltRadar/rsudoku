use std::{fs, path, io, fmt};

/**
 * A Sudoku space value
 * 
 * Known with a digit,
 * Unknown with a vector of possible tests
 */
#[derive(PartialEq, Debug)]
enum SudokuValue {
    Known(usize),
    Unknown(Vec<usize>)
}

impl SudokuValue {

    
    /// Create a new Sudoku value from a character.
    /// 
    /// If 'value' is a digit '1' to '9', then it will return a Known value
    /// 
    /// If 'value' is a letter a-z or A-Z or '0' then it'll return an
    /// Unknown value with possible values 1 to 9
    /// 
    /// All other values return None
    /// 
    /// ```
    /// assert_eq!(SudokuValue::new('3'), Some(Known(3)))
    /// assert_eq!(SudokuValue::new('0'), Some(Unknown(vec![1,2,3,4,5,6,7,8,9])))
    /// assert_eq!(SudokuValue::new('A'), Some(Unknown(vec![1,2,3,4,5,6,7,8,9])))
    /// assert_eq!(SudokuValue::new('-'), None)
    /// assert_eq!(SudokuValue::new('\t'), None)
    /// ```
    ///
    fn new(value: char) -> Option<SudokuValue> {
        
        match value {
            '1'..='9' => {
                return Some(SudokuValue::Known(value.to_digit(10).unwrap() as usize));
            },
            
            'a'..='z' | 'A'..='Z' | '0' => {
                return Some(SudokuValue::Unknown(vec![1,2,3,4,5,6,7,8,9]));
            }

            _ => {
                return None;
            }
        }
    }

    /// Return if an Sudoku value is Known
    ///
    ///
    fn is_known(&self) -> bool{
        match self {
            Self::Known(_) => true,
            Self::Unknown(_) => false,
        }
    }
}

/**
 * A Sudoku board. Contains the sudoku spaces in a 2D vector, and the
 * number of emtpy spaces. An empty_spaces option of 0 means the sudoku
 * is solved
 */
#[derive(Debug)]
struct SudokuBoard {
    spaces: Vec<Vec<SudokuValue>>,
    empty_spaces: usize
}

impl SudokuBoard {
    fn new(board_filename: &str) -> io::Result<Self> {
        let mut path = path::PathBuf::from("boards");
        path.push(board_filename);

        let board_string = fs::read_to_string(path)?;

        let mut spaces = Vec::with_capacity(9);

        let mut row_index = 0;
        let mut finished = false;

        let mut empty_spaces = 0;
        let mut known_spaces = 0;

        for charater in board_string.chars() {

            // Make a Sudoku value from a character
            match SudokuValue::new(charater) {
                
                Some(value) => {

                    if value.is_known() {
                        known_spaces += 1;
                    }
                    else {
                        empty_spaces += 1;
                    }

                    if spaces.len() == row_index {
                        spaces.push(Vec::with_capacity(9));
                    }

                    spaces[row_index].push(value);

                    if spaces.len() == 9 {
                        row_index += 1;
                        if row_index == 9 {
                            finished = true;
                            break;
                        }
                    }
                }

                None => {}
            }
            
            // Stop processing file if you've filled the board
            if finished {
                break;
            }
        }

        if known_spaces + empty_spaces != 81 {
            return io::Result::Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Failed to fill board"
            ))
        }
        
        return io::Result::Ok(SudokuBoard{
            spaces,
            empty_spaces
        });
    }


}



fn main() {
    let s = SudokuBoard::new("easy").unwrap();
    dbg!(s);
}
