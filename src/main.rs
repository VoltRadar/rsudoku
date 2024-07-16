use std::{fmt, fs, io, path};


/// All of the results of removing a possible value from a space. 
/// 
/// Possible value was in the space,
/// The possible value wasn't in the space,
/// The value is now known. The SudokuValue is now a Known type
enum SudokuValueResult {
    PossibleValueRemoved,
    PossibleValueAlreadyRemoved,
    ValueNowKnown
}

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
    fn from(value: char) -> Option<SudokuValue> {
        
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
    fn is_known(&self) -> bool{
        match self {
            Self::Known(_) => true,
            Self::Unknown(_) => false,
        }
    }

    /// Removes a possible value from a sudoku space. Has multiple 
    /// outputs:
    /// 
    /// For an Unknown space:
    /// - If possible_value_to_remove isn't in the space, does nothing.
    /// - If it is, value if removed, and SudokuValue is turned into Known
    ///   if only one unknown value exists
    /// 
    /// For a Known space:
    /// - If possible_value_to_remove not the known value, returns
    ///   `PossibleValueAlreadyRemoved`
    /// - Else, return `Result::Err(())`. Sudoku is unsolvable
    /// 
    /// 
    /// TODO: Make this work so you can reassign self
    ///     
    fn remove(&mut self, possible_value_to_remove: usize) -> Result<SudokuValueResult, ()> {
        
        match self {
            Self::Known(value) => {
                
                if *value == possible_value_to_remove {
                    // Tried to remove a known value space. Sudoku is unsolvable
                    return Result::Err(());
                } else {
                    return Result::Ok(SudokuValueResult::PossibleValueAlreadyRemoved);
                }
            }

            Self::Unknown(possible_values) => {
                let possible_index = possible_values
                    .iter().
                    position(|&x| x == possible_value_to_remove);

                match possible_index {
                    None => {return Result::Ok(SudokuValueResult::PossibleValueAlreadyRemoved)}
                    
                    Some(index) => {
                
                        // Remove the possible value from the vector
                        possible_values.swap_remove(index);
                        
                        // Check the length, to see if the value can be known
                        if possible_values.len() == 1 {
                            *self = SudokuValue::Known(possible_values[0]);
                            return Result::Ok(SudokuValueResult::ValueNowKnown);
                        } else {
                            return Result::Ok(SudokuValueResult::PossibleValueRemoved);
                        }
                    
                    }
                }
            }
        }
    }
}

impl fmt::Display for SudokuValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Known(x) => {write!(f, "{}", x)}
            Self::Unknown(_) => {write!(f, "{}", 'X')}
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

    /// Create a new sudoku board from a file
    /// 
    /// `board_filename` should exclude the `boards` directory
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
            match SudokuValue::from(charater) {
                
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

                    if spaces[row_index].len() == 9 {
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

    /// Returns if the number of empty spaces is zero
    fn is_solved(&self) -> bool {
        return self.empty_spaces == 0;
    }

    /// Returns the cordients of the spaces adjecent to the input space
    /// 
    /// An Adjectent space is a space that is in the same row, column, or
    /// box. All entries are unique
    /// 
    /// (0, 0) is top left, (8, 0) is top right
    fn get_adjecent_spaces(point: (usize, usize)) -> Vec<(usize, usize)> {
        let mut adjecent_spaces = Vec::with_capacity(20);

        // Add the rows and columns
        for i in 0..9 {
            if i != point.0 {
                adjecent_spaces.push((i, point.1));
            }

            if i != point.1 {
                adjecent_spaces.push((point.0, i));
            }
        }

        // Add box spaces not in rows or columns
        let mut box_x = Vec::with_capacity(2);
        let mut box_y = Vec::with_capacity(2);

        for j in 0..3 {
            if point.0 % 3 != j {
                box_x.push(j + (point.0 / 3) * 3)
            }

            if point.1 % 3 != j {
                box_y.push(j + (point.1 / 3) * 3)
            }
        }

        for x in &box_x {
            for y in &box_y {
                adjecent_spaces.push((*x, *y));
            }
        }

        return adjecent_spaces;
    }

}

impl fmt::Display for SudokuBoard {

    /// Print the Sudoku board
    /// 
    /// The board consistes of the Sudoku Values, sperated by spaces, and
    /// horizontoal lines
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines: Vec<String> = Vec::with_capacity(11);

        let mut line_index = 0;
        for line in &self.spaces {
            
            // Create a line containing sudoku values
            lines.push(String::with_capacity(21));

            for (space_index, space) in line.iter().enumerate() {
                
                if space_index == 8 {
                    lines[line_index].push_str(&space.to_string());
                }

                else {
                    lines[line_index].push_str(&format!("{} ", space));

                    // Add a horizontal line character to make boxes
                    if (space_index + 1) % 3 == 0 {
                        lines[line_index].push_str("| ")
                    }
                }
            }

            // Insert two horizontal lines
            if line_index == 2 || line_index == 6 {
                lines.push(String::from("---------------------"));
                line_index += 1
            }

            line_index += 1;
        }

        for (line_index, line) in lines.iter().enumerate() {
            if line_index < lines.len() - 1 {
                writeln!(f, "{}", line)?;
            } else {
                write!(f, "{}", line)?;
            }
        }
        
        fmt::Result::Ok(())
    }
}

fn main() {
    let s = SudokuBoard::new("easy").unwrap();

    println!("{}", s);

    println!("{:?}", SudokuBoard::get_adjecent_spaces((6, 7)));
}
