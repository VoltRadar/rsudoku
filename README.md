# rsudoku

## Purpose

This is a recreation of a project I did for university to solve sudoku's.
That was done in Python, and I've uploaded
[here](https://github.com/VoltRadar/python-sudoku-solver).
I made this project in part to get experience with Rust, and to compare to
Python, with attention paid to performance, development time, and ease of
use

## Solving strategy's

The methods I used to solve sudoku's in each program were very similar, and
I'll outline them here. Although the Rust version contains a few more
enhancements.

### Structure

Each Sudoku was assumed to have either zero, one, or many solutions. If a
Sudoku has multiple solutions, only one is returned

The Sudoku's were stored in an 2d array. Each value in the array is either
an integer, representing a known space, or another array, representing the
possible values that space could have.

### Initialization

When loading a new board, the possible spaces should be reduced based on
the known space. For each known space, you can go though all the spaces in
the same row, column or 3x3 box, and remove the known value from the
possible values of unknown spaces.

Sometimes, the number of possible values in an unknown space becomes 1,
which means it can turn into a known space and added to the list
(implemented in this Rust version as a queue) of the known spaces to go
though. Once the queue is empty, then the initialization has finished.
This process alone is enough for easy Sudoku's.

The event that an unknown space contains zero possible values, or that two
known spaces of the same are adjacent to each other, i.e. in the same row
column and box, the Sudoku is unsolvable

### Narrowing

Each row, column, and box needs to contain every value, 1 to 9. Consider
the following example:

```
X X X | X X X | X X X
X X X | X X X | X X X
X X X | X X X | 1 X X
---------------------
X X X | X X X | X X X
X X X | X X X | X X X
X X X | X X X | X 1 X 
---------------------
X 1 X | X X X | o o o
X X X | X 1 X | o o o 
X X X | X X X | o o o
```

Here, there is a 1 on rows 7 and 8, and in columns 7 and 8. This means
that the only space for a 1 to exist in row 9, column 9 and it can be
filled in

Using this idea, we can go though each row, column, and box, each
containing 9 spaces. Call each of these a 'set' of spaces. For each digit
1-9, go though all of these spaces in the set. If the digit is known in
the set, skip it. If it isn't known, count the unknown spaces where the
digit is a possible value. If the digit could be in two or more spaces,
we can't fill in anything. If it appears in zero spaces, we know the
Sudoku is unsolvable.

If the digit appears in exactly one space, then that space must contain
that digit, and it can be filled in as known. When filling in this known
space, you can also go though all spaces in it's row, column, and box, and
remove it's value from the possible values in the unknown spaces. Should
any of these spaces then have exactly one possible value, fill it in
recursively. If it contains zero values, or is a known value of the same
digit, then the Sudoku is unsolvable

This strategy, when combined with initialization above, can solve quite
hard Sudoku's. Due to the filling in of values, this may can be repeated a
number of times until the number of filled in spaces is zero. Or if it is
solved or shown to be unsolvable.

### Guessing

As a fall back, we can use guessing. We could find a space, and guess
values until we find the correct one. After filling in this value, it
would be easier to solve.

First we need to find the best possible guess. We make a guess that's most
likely to be correct, and one which will reveal lots of information. The
first heuristic for each guess is the number of possible values in the
space. This is the post important one. If a space has two possible values,
each guess has a 50% chance of being right. If it has 3, then it's 33%
chance per guess.

The second heuristic is the number of spaces that can be solved as a
direct result of the guess, and the third is the number of possible values
removed as result of the guess. These two mean we'll chose the guess with
the most impact, which means we will be able to quickly solve the Sudoku,
or find it was unsolvable. 

After we've found the best guess, we make a copy of the current Sudoku
board, and we fill in the guess, then we try to solve the board using
narrowing, and then random guessing on *that* board if narrowing solve the
board. If the new board is solved, return this solution. If it can't be
solved, however, that means the guess was wrong, and we can remove it from
the possible values for the original board.

From here, we can try narrowing the board again. While it's only one
removed value, so the result of the narrowing will often be no change, this
kind of recursive guessing has the highest complexity, with the total
number of guesses possible equal to the product of the number of possible
values in each unknown space. This makes any reduction of the possible
values of any space useful

## Performance

### Boards

I will use two boards to test performance. The first is the board `17`
from this project. It's found on Wikipedia for Sudoku.

The second is a blank board, with all spaces unknown. This should be
harder, and will require a lot of guessing.

### Python version


Results from timing my old Python solver for 5 minutes. All times in microseconds


17 spaces board:
- Trials: 32 886
- Medium: **9 317us**
- 10-quantiles: 7 673us, 7 957us, 8 511us, 9 190us, 9 317us, 9 436us,
9 588us, 9 814us, 10 224us

Blank board:
- Trials: 6 484
- Medium: **43 512us**
- 10-quantiles: 41 546us, 42 958us, 43 133us, 43 278us, 43 512us,
44 195us, 45 250us, 48 184us, 56 508us
