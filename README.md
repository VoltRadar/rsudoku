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
removed value, so the result of the narrowing will often have no effect, this
kind of recursive guessing has a high complexity, with the total
number of guesses possible equal to the product of the number of possible
values in each unknown space. This makes any reduction of the possible
values of any space useful

## Performance results

### Boards

I will use two boards to test performance. The first is the board `17`
from this project. It's found on Wikipedia for Sudoku.

The second is a blank board, with all spaces unknown. This should be
harder, and will require a lot of guessing.

### Python version


Results from timing my old Python solver for 5 minutes. All times in
microseconds (µs)


17 spaces board:
- Trials: 32 886
- Medium: **9 317µs**
- 10-quantiles: 7 673µs, 7 957µs, 8 511µs, 9 190µs, 9 317µs, 9 436µs,
9 588µs, 9 814µs, 10 224µs

Blank board:
- Trials: 6 484
- Medium: **43 512µs**
- 10-quantiles: 41 546µs, 42 958µs, 43 133µs, 43 278µs, 43 512µs,
44 195µs, 45 250µs, 48 184µs, 56 508µs

### Rust version

Results from the same two boards in Rust:

17 Space board:
- Trials: 7490288
- Medium: **38 µs**
- 10-quantiles: 34µs, 35µs, 36µs, 37µs, 38µs, 39µs, 40µs, 40µs, 44µs

Blank board:
- Trials: 256082
- Medium: **1 106µs**
- 10-quantiles: 1 053µs, 1 070µs, 1 078µs, 1 088µs, 1 106µs, 1 121µs,
1 147µs, 1 186µs, 1 355µs

### Limitations

I have limited experience getting good information about the performance
of a program. The programs were slightly modified so that the main
function that solves a board was called multiple and the times taken
recorded. I used Pythons `time.monotonic()` function from the
[time module.](https://docs.python.org/3/library/time.html#time.monotonic)
It may not be the correct way to time a function. The Rust version is
similar.

Each board was solved multiple times for 5 minutes, and the times taken
recorded. I was also doing things in the background while I was doing
this. This could have affected the results. However, the 90% quantile time
(the time when 90% of times solved are less), is constantly 30ish% larger
then the 10% quantile time, which goes against the idea the way it was run
affected the times in a meaningful way, even if the way that the times
were captured was wrong

This isn't a fully fair comparison. In the Python version, after we guess
and it's found we can't solve it, we don't try narrowing again. I believe
this is the only difference, algorithmically, between the two programs.

I also wrote the Python sudoku solver 2 to 3 years ago. Now I've got more
programming experience, I might be able to go back and improve performance
of the Python version. After writing it, I wrote the Rust version, so I
would have had more time to think about the best algorithms for solving
sudoku's.

## Analysis

### Performance

The Rust version is clearly faster than the Python version. With solving
the board with 17 spaces, the Python solver did it on average in `9 317µs`,
but the Rust version takes only `38µs`. This makes the Rust version
**245 times** faster!

The blank board was also faster in Rust, but less pronounced. The Python
version solved it in `43 512µs`, and the Rust version solved it in
`1 106µs`. This makes it **39 times** faster.

I'd take these values of ratio's with a grain of salt. Read the
limitations section above.

That the ratio between the two boards being different is interesting.
Maybe it's because of the extra checks that the Rust version does for each
guess, so each guess takes longer. They'd be good for the 17 board, where
these extra checks may reveal extra information, but for the blank board
they wouldn't help as much.

These numbers are both different from the average ratio between Python and Rust found in the Paper
[Energy Efficiency across Programming Languages, Marco Couto et all, 2017.](https://www.researchgate.net/publication/320436353_Energy_efficiency_across_programming_languages_how_do_energy_time_and_memory_relate)
This paper found a ratio of 69 times on average between execution time in
Python and Rust. I believe that the performance difference for each
benchmark test that was run in the paper could change, so the ratio quoted
here is an average.

It's not surprising that Rust is faster then Python. Rust is compiled, and
the compilation time wasn't included in the times above. Python is
interpreted (or complied to bytecode then interpreted with the CPython
version I have installed, I think), and this interpretation *is* included
in the times recorded. 

### Ease of writing / development time

It has been a long time since writing the Sudoku solver in Python, so to
compare the two languages here would be difficult.

I found the Rust solver interesting to make. Rust is statically typed, and
enforces a strict set of rules with it's borrow checker. This makes it
more difficult to make something quickly that works. Although arguably it
makes it easier to make something that works quickly

Python meanwhile is a lot faster make make something in, and the
development time is lower. You aren't required to deal with every error
that could arise, and the dynamic type system is a lot more flexible. This
means that it's easier to write the program then in Rust.

You may look at the results for performance above and conclude that Rust
is faster. However, this clearly doesn't include development time, which
is often more important. Like, while the Rust version was 245 times better
then the Python version, it's also only 0.009 seconds better. And if the
Python version takes a few hours less to make, then we'd need to solve
1000s of puzzles to make it worth it.

If we're not making a service that
solves any Sudoku given to it as fast as possible, but simply writing a
script to solve it and then moving on to other projects, Python is
perfect to make a script that understandable and just works.

## Conclusion

I wanted to make this project mainly to get experience with Rust, but also
to have a little fun comparing Rust with Python.

It's a slightly academic exercise however. Rust and Python are designed
for completely different purposes. It's a bit like comparing a bicycle and
a passenger airliner.

The aircraft will be able to move a large amount of people at high speed
over a long distance. It's going to be a lot harder to do that on a bike
between London and Paris. On the other hand, if you've run out of some
food and need to quickly pop down to the local shop, a bicycle would be a
lot better for that short trip. I don't think most airliners would be very
good at riding down a country road or a narrow city street.

Yes, they're both methods of transportation, but they are not designed for
the same use cases, and will be a lot better then the other in a lot of
aspects. The same is true for programming languages.

Rust emphasizes performance and reliability. It's designed so that the
program you write runs as fast as it can, and deal with all the possible
errors that could be encountered. It's borrow checker means that it can be
memory safe and reliable without a garbage collector, improving
performance. It's strict type system also allows it to run faster.

These upsides come with downsides however. The type system and borrow
checker can be difficult to work with, and require often reference to
documentation to solve issues that are encountered. It's a awkward process
that slows down development, and can make the program difficult to
understand for people who don't have much experience in the language.

Python is almost the opposite. It's readable, and easy to learn and work
with. Types are extremely flexible. A list can contain numbers, strings,
list, custom objects, and even a copy of itself. Python makes writing
programs that just work easy. This, of course, comes with it's own
downsides. It's slow (comparatively) to run, and not knowing exactly what
type an object is can cause bugs.

Both languages have strengths and weaknesses. One isn't "better" then the
other, because they do different things. When selecting a programming
language for a project, many things need to be considered. How valuable is
the performance of the program? How costly is development time? Will this
program need to be maintainable by someone who didn't write it?

So, Rust is better then Python... and Python is better then Rust.