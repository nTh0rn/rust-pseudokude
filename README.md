# Rust Pseudokude
A sudoku solver made in rust. Capable of solving any size sudoku (so long as digits don't surpass a `u16`).

# How to use
Just modify the `let init = vec![]` to contain the board you want to solve.

# How does it work?
Pseudokude's main function is just a basic bruteforce back-tracker. It makes massive optimizations by, in between back-tracking, checking for rows/columns/houses containing cells of lone-possibilities. This means if only 1 cell in a row/column/house contains the possibility for a particular digit, it will immediately assign that cell to that digit. It also checks for cells that only have a single possibility, automatically assigning them too.

Pseudokude is able to solve easy to medium puzzles in a single pass through just by process of eliminating lone-possibilities. The more difficult the puzzle, the slower the first-half of solving, but the last-half is greatly sped up by process of eliminating lone-possibilities.

# Important note
Solving can be **GREATLY** sped-up by removing `self.show()` in `fn process_of_elimination()`. Printing text is extremely slow, so calculations can be ran much faster by removing this. It is included by default because it's very cool to see the board update while solving.
