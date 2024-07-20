use std::{cmp::Ordering, collections::VecDeque, fmt, fs, io, path, time};

/// All of the results of removing a possible value from a space.
///
/// Possible value was in the space,
/// The possible value wasn't in the space,
/// The value is now known. The SudokuValue is now a Known type
enum SudokuValueResult {
    PossibleValueRemoved,
    PossibleValueAlreadyRemoved,
    ValueNowKnown,
}

/**
 * A Sudoku space value
 *
 * Known with a digit,
 * Unknown with a vector of possible tests
 */
#[derive(PartialEq, Debug, Clone)]
enum SudokuValue {
    Known(usize),
    Unknown(Vec<usize>),
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
            }

            'a'..='z' | 'A'..='Z' | '0' => {
                return Some(SudokuValue::Unknown(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));
            }

            _ => {
                return None;
            }
        }
    }

    /// Return if an Sudoku value is Known
    fn is_known(&self) -> bool {
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
                    .iter()
                    .position(|&x| x == possible_value_to_remove);

                match possible_index {
                    None => return Result::Ok(SudokuValueResult::PossibleValueAlreadyRemoved),

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
            Self::Known(x) => {
                write!(f, "{}", x)
            }
            Self::Unknown(_) => {
                write!(f, "{}", 'X')
            }
        }
    }
}

/**
 * A Sudoku board. Contains the sudoku spaces in a 2D vector, and the
 * number of emtpy spaces. An empty_spaces option of 0 means the sudoku
 * is solved
 */
#[derive(Debug, Clone)]
struct SudokuBoard {
    spaces: Vec<Vec<SudokuValue>>,
    empty_spaces: usize,
    initalised: bool
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
        let mut empty_spaces = 0;
        let mut known_spaces = 0;

        for charater in board_string.chars() {
            // Make a Sudoku value from a character
            match SudokuValue::from(charater) {
                Some(value) => {
                    if value.is_known() {
                        known_spaces += 1;
                    } else {
                        empty_spaces += 1;
                    }

                    if spaces.len() == row_index {
                        spaces.push(Vec::with_capacity(9));
                    }

                    spaces[row_index].push(value);

                    if spaces[row_index].len() == 9 {
                        row_index += 1;
                        if row_index == 9 {
                            break;
                        }
                    }
                }

                None => {}
            }
        }

        if known_spaces + empty_spaces != 81 {
            return io::Result::Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Failed to fill board",
            ));
        }

        return io::Result::Ok(SudokuBoard {
            spaces,
            empty_spaces,
            initalised: false
        });
    }

    /// Returns if the number of empty spaces is zero
    fn is_solved(&self) -> bool {
        return self.empty_spaces == 0;
    }

    /// Static method
    ///
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

    /// Function that checks all known spaces and removes their value from
    /// adjectent unknown spaces.
    ///
    /// May also change unknown spaces to known spaces, and also removes
    /// these values from adjectent spaces
    fn inital_check(&mut self) -> Result<(), String> {

        // Set self be initalised. This method only needs to be run once on each board
        self.initalised = true;

        // Queue of known points. The 3 value tuple is it's row index,
        // column index, and space value
        let mut known_points_to_check: VecDeque<(usize, usize, usize)> = VecDeque::new();

        // Add all the currently known spaces to the queue
        for (i, row) in self.spaces.iter().enumerate() {
            for (j, space) in row.iter().enumerate() {
                if let SudokuValue::Known(space_value) = space {
                    known_points_to_check.push_back((i, j, *space_value));
                }
            }
        }

        while !known_points_to_check.is_empty() {
            let (x, y, space_value) = known_points_to_check.pop_front().expect("Queue is empty!");

            for adjecent_space in SudokuBoard::get_adjecent_spaces((x, y)) {
                // Remove it, and get the result of removeing it
                let remove_result =
                    self.spaces[adjecent_space.0][adjecent_space.1].remove(space_value);

                if remove_result.is_err() {
                    return Result::Err(String::from("Sudoku board is unsolvable"));
                }

                if let SudokuValueResult::ValueNowKnown = remove_result.unwrap() {
                    self.empty_spaces -= 1;

                    match self.spaces[adjecent_space.0][adjecent_space.1] {
                        SudokuValue::Known(adjecent_space_value) => {
                            known_points_to_check.push_back((
                                adjecent_space.0,
                                adjecent_space.1,
                                adjecent_space_value,
                            ));
                        }

                        _ => panic!("Space should be Known"),
                    }
                }
            }
        }

        return Result::Ok(());
    }

    /// Returns a 2d vector of points.
    ///
    /// Each vector of points are the postion of values in a single row,
    /// column, or 3x3 box
    fn get_all_full_corrdinates() -> Vec<Vec<(usize, usize)>> {
        let mut output: Vec<Vec<(usize, usize)>> = Vec::with_capacity(27);

        // Add rows and columns
        for i in 0..9 {
            let mut row: Vec<(usize, usize)> = Vec::with_capacity(9);
            let mut column: Vec<(usize, usize)> = Vec::with_capacity(9);
            for j in 0..9 {
                row.push((i, j));
                column.push((j, i));
            }

            output.push(row);
            output.push(column);
        }

        // Add 3 by 3 boxes
        for box_x_index in 0..3 {
            for box_y_index in 0..3 {
                let mut sudoku_box: Vec<(usize, usize)> = Vec::with_capacity(9);

                for i in 0..9 {
                    sudoku_box.push((box_x_index * 3 + i % 3, box_y_index * 3 + i / 3))
                }

                output.push(sudoku_box);
            }
        }

        return output;
    }

    /// Fill in a sudoku space with a value
    ///
    /// Also remove this value from adject sudoku spaces, and if the value
    /// is now known, recesivlly removes and checks new known values
    ///
    /// If space is already known, checks to see if the value is correct,
    /// then proform the checks as above
    ///
    /// Returns the number of new known spaces
    ///
    /// If filling in any space is invalid, then return Result::Err
    fn fill_space(&mut self, point: (usize, usize), value: usize) -> Result<usize, String> {
        let mut new_known = 0;

        // Fill in the space
        match &self.spaces[point.0][point.1] {
            SudokuValue::Known(x) => {
                if *x != value {
                    return Result::Err(String::from("Tried to overright in a known value"));
                } else {
                    // The function should still check adjectent values and remove these.
                }
            }

            SudokuValue::Unknown(possible) => {
                if possible.contains(&value) {
                    // Repalce the value
                    self.spaces[point.0][point.1] = SudokuValue::Known(value);
                    new_known += 1;
                    self.empty_spaces -= 1;
                } else {
                    return Result::Err(String::from("Not a possible value"));
                }
            }
        }

        for point_to_check in SudokuBoard::get_adjecent_spaces(point) {
            let space_to_check = &mut self.spaces[point_to_check.0][point_to_check.1];

            // Remove known value from possible values, and check the result
            let removed_result = space_to_check.remove(value);

            if removed_result.is_err() {
                return Result::Err(String::from("Sudoku is unsolvable"));
            }

            // If the value can now be known, add it to a vector to be checked later
            if let SudokuValueResult::ValueNowKnown = removed_result.unwrap() {
                if let SudokuValue::Known(checked_known_value) = space_to_check {
                    let owned_checked_known_value = checked_known_value.clone();

                    self.fill_space(point_to_check, owned_checked_known_value)?;
                }

                new_known += 1;
                self.empty_spaces -= 1;
            }
        }

        return Result::Ok(new_known);
    }

    /// Narrow down the possible values of empty spaces, filling in any
    /// ones it can
    ///
    /// Does the following tests:
    ///
    /// - Check each row, column, and box. If there's exactly one space
    /// that can have a digit, fill in that digit. If zero places can have
    /// that digit, return an error
    ///
    /// Returns the number of new known spaces. Further narrowing may be
    /// done if the result is higher then zero
    ///
    /// If the sudoku is unsolvable, return an errors
    fn narrow(&mut self) -> Result<usize, String> {
        let mut new_spaces_known = 0;

        // For each row, column, and box
        for space_set_corrdinates in SudokuBoard::get_all_full_corrdinates() {
            'values: for value in 1..=9 {
                // Possition of unknown value to fill in
                let mut unknown_value_to_fill_in: Option<(usize, usize)> = None;

                for point in &space_set_corrdinates {
                    match &self.spaces[point.0][point.1] {
                        // Check the next value if known value already in the set
                        SudokuValue::Known(known_value) => {
                            if *known_value == value {
                                continue 'values;
                            }
                        }

                        SudokuValue::Unknown(possible_values) => {
                            if possible_values.contains(&value) {
                                match unknown_value_to_fill_in {
                                    // More than one unknown space with
                                    // the same possible value, check
                                    // next value
                                    Some(_) => {
                                        continue 'values;
                                    }
                                    None => unknown_value_to_fill_in = Some(*point),
                                }
                            }
                        }
                    }
                }

                match unknown_value_to_fill_in {
                    // There exists a possible space in a row, column, or
                    // box that couldn't contain a value not already known
                    // in that row, column, or box. This is invalid, so
                    // the sudoku is unsolvable
                    None => return Result::Err(String::from("Sudoku is unsolvable")),

                    // This value can be filled in. Return any error it
                    // might raise. Update the number of new spaces known
                    Some(point) => {
                        new_spaces_known += self.fill_space(point, value)?;
                    }
                }
            }
        }

        return Result::Ok(new_spaces_known);
    }

    /// Repeats narrowing until no new spaces are found, or until solved.
    ///
    /// Narrows at least once
    ///
    /// Returns if sudoku is now solved
    ///
    /// Returns Err if sudoku is unsolvable
    fn narrow_full(&mut self) -> Result<bool, String> {
        while self.narrow()? > 0 && !self.is_solved() {}

        return Result::Ok(self.is_solved());
    }

    /// Returns a reference to a space
    fn get_space(&self, point: (usize, usize)) -> &SudokuValue {
        return &self.spaces[point.0][point.1];
    }

    /// Returns the guess with the most impact, which is the guess that
    /// results in the most adject spaces being solved.
    ///
    /// The one with the most impact will always have the smallest number
    /// of possible values
    ///
    /// If there is a tie, pick the guess with the most possible value
    /// removed from adject spaces as a result of the guess
    fn most_impactful_guess(&self) -> ((usize, usize), usize) {
        
        #[derive(Debug)]
        struct Impact {
            point: (usize, usize),
            guess: usize,
            number_of_possible_values: usize,
            spaces_solved: usize,
            possible_values_removed: usize,
        }


        let mut best_guess = Impact {
            point: (0, 0),
            guess: 0,
            number_of_possible_values: usize::MAX,
            spaces_solved: 0,
            possible_values_removed: 0,
        };

        for i in 0..9 {
            for j in 0..9 {
                if let SudokuValue::Unknown(possible_values) = &self.get_space((i, j)) {

                    if possible_values.len() > best_guess.number_of_possible_values {
                        continue;
                    }

                    for possible_value in possible_values {
                        // Check adject values
                        let mut other_spaces_solved_by_guess = 0;
                        let mut possible_values_removed_by_guess = 0;

                        // Look at all spaces adject to the guess
                        for point in SudokuBoard::get_adjecent_spaces((i, j)) {
                            if let SudokuValue::Unknown(adject_possible_values) =
                                self.get_space(point)
                            {
                                
                                // Is our guess one of the adject spaces possible values?
                                if adject_possible_values.contains(possible_value) {
                                    
                                    possible_values_removed_by_guess += 1;
                                    if adject_possible_values.len() == 2 {
                                        other_spaces_solved_by_guess += 1;
                                    }
                                }
                            }
                        }

                        let new_guess = Impact {
                            point: (i, j),
                            guess: *possible_value,
                            number_of_possible_values: possible_values.len(),
                            spaces_solved: other_spaces_solved_by_guess,
                            possible_values_removed: possible_values_removed_by_guess
                        };

                        if new_guess.number_of_possible_values < best_guess.number_of_possible_values {
                            // Number of possible values for this point is
                            // less then best guess, so better, because
                            // the guess is more likey to be correct
                            best_guess = new_guess;
                            continue;
                        }

                        match new_guess.spaces_solved.cmp(&best_guess.spaces_solved) {
                            Ordering::Greater => {
                                // New guess solves more spaces then best case, so is better

                                best_guess = new_guess;

                                continue;
                            },
                            Ordering::Less => continue, // Best guess solves more spaces then new guess
                            _ => {}
                        }

                        match new_guess.possible_values_removed.cmp(&best_guess.possible_values_removed) {
                            Ordering::Greater => {
                                // New guess removes more possible values then old guess
                                best_guess = new_guess;

                                continue;
                            }

                            _ => {} // New guess is no better then best guess
                        }
                    }
                }
            }
        }

        return (best_guess.point, best_guess.guess);
    }

    /// Tries to solve the sudoku
    /// 
    /// This is done with a comination of algormiths norrowing down the
    /// possible values each space could be, and random guessing and
    /// recersving trying to solve that board
    /// 
    /// Returns if the sudoku was solved
    fn solve(&mut self) -> bool {

        // Proform some quick checks to fill in easy values and remove
        // possible values for each space

        if !self.initalised {
            if let Result::Err(_) = self.inital_check() {
                return false;
            }
        }
        if self.is_solved() {
            return true;
        }

        if let Result::Err(_) = self.narrow_full() {
            return false;
        }
        if self.is_solved() {
            return true;
        }

        loop {
            // Get most impactful guess
            let (point, guess_value) = self.most_impactful_guess();

            let mut guess_board = self.clone();

            if guess_board.fill_space(point, guess_value).is_ok() {
                // Try to solve the board with a guess
                if guess_board.solve() {
                    // Set this board to the guess board if guess board was solved
                    *self = guess_board;
                    return true;
                }
            }

            // The guess was wrong remove it
            let result_of_removeal =  self.spaces[point.0][point.1].remove(guess_value);

            if result_of_removeal.is_err() {
                return false;
            }

            if let SudokuValueResult::ValueNowKnown = result_of_removeal.expect("Checked for error above") {
                // The guess was wrong, but now the guessed space is known
                self.empty_spaces -= 1;

                if let SudokuValue::Known(point_new_value) = self.get_space(point) {
                    if self.fill_space(point, *point_new_value).is_err() {
                        // Filling in the space resulted in an unsolvable sudoku
                        return false;
                    }

                    if self.is_solved() {
                        return true;
                    }

                    // Do more algormic removing
                    if self.narrow_full().is_err() {
                        return false;
                    }
                    
                    if self.is_solved() {
                        return true;
                    }
                }

            };
        }
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
                } else {
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

fn main() -> Result<(), String> {
    let mut s = SudokuBoard::new("veryhard").unwrap();

    println!("{}", s);
    println!("");

    let start = time::Instant::now();
    s.solve();
    let duration = start.elapsed();

    println!("{}", s);
    println!("{}us", duration.as_micros());

    if s.is_solved() {
        println!("Solved!");
        return Result::Ok(());
    } else {
        return Result::Err(String::from("Not solved!"));
    }
}
