// Basic sudoku solver

pub const NUM_ROWS: usize = 9;
pub const NUM_COLS: usize = 9;

// Indices to print divider on (after 3rd, after 6th)
const PRINT_IDX: [usize; 2] = [2, 5];

// For output formatting
const ROW_DIVIDER: &str = "\n---------------------";
const COL_DIVIDER: &str = "| ";

// Cell values
const NUM_UNSET: u8 = 0;
const NUM_MIN: u8 = 1;
const NUM_MAX: u8 = 9;

pub struct Sudoku {
    // The board -- 0 = unsolved/unset
    board: [[u8; NUM_COLS]; NUM_ROWS],
}

impl Sudoku {
    // Create a new sudoku with the board passed in.
    pub fn new(board: [[u8; NUM_COLS]; NUM_ROWS]) -> Self {
        Sudoku {
            board: board,
        }
    }

    // Display the sudoku.
    pub fn display(&self) {
        for (i, row) in self.board.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if *val == NUM_UNSET {
                    print!("- ");
                } else {
                    print!("{} ", val);
                }

                if PRINT_IDX.contains(&j) {
                    print!("{}", COL_DIVIDER);
                }
            }
            if PRINT_IDX.contains(&i) {
                print!("{}", ROW_DIVIDER);
            }
            println!("");
        }
    }

    // Check whether a value would be valid for an individual cell.
    fn check_cell(&self, row:usize, col:usize, cell_val:u8) -> bool {
        // Check row/col
        for i in 0..usize::from(NUM_MAX) {
            // Row
            if i != col && self.board[row][i] == cell_val {
                return false;
            }

            // Col
            if i != row && self.board[i][col] == cell_val {
                return false;
            }
        }

        // Starting offsets of box
        let box_row_offset = (row / 3) * 3;
        let box_col_offset = (col / 3) * 3;

        // Check box
        for i in usize::from(box_row_offset)..usize::from(box_row_offset+3) {
            for j in usize::from(box_col_offset)..usize::from(box_col_offset+3) {
                if i != row && j != col && self.board[i][j] == cell_val {
                    return false;
                }
            }
        }
        return true;
    }

    // Verify whether an individual cell is valid.
    fn verify_cell(&self, row:usize, col:usize) -> bool {
        let cell_val = self.board[row][col];

        // Ignore if unset
        if cell_val == 0 {
            return true;
        }

        self.check_cell(row, col, cell_val)
    }

    // Verify whether the entire sudoku is valid.
    pub fn verify(&self) -> bool {
        for i in 0..usize::from(NUM_MAX) {
            for j in 0..usize::from(NUM_MAX) {
                if !self.verify_cell(i, j) {
                    return false;
                }
            }
        }
        return true;
    }

    // Get the indexes of the first set value.  Returns usize::MAX for both if not found.
    fn get_first_empty(&self) -> [usize; 2] {
        for i in 0..usize::from(NUM_MAX) {
            for j in 0..usize::from(NUM_MAX) {
                if self.board[i][j] == NUM_UNSET {
                    return [i, j];
                }
            }
        } 
        
        return [usize::MAX; 2];
    }

    // Solve the sudoku using backtracking.
    pub fn solve(&mut self) -> bool {
        let [row, col] = self.get_first_empty();
        if [row, col] == [usize::MAX; 2] {
            // None found, so the sudoku is solved
            return true;
        }

        for val in NUM_MIN..(NUM_MAX+1) {
            if self.check_cell(row, col, val) {
                // Cell value works, so try to continue solving with it
                self.board[row][col] = val;

                if self.solve() {
                    return true;
                }

                // Couldn't solve, so backtrack
                self.board[row][col] = NUM_UNSET;
            }
        }
        return false;
    }

    // Set the entire board to the values passed in.
    pub fn set_full(&mut self, board_set:[[u8; NUM_COLS]; NUM_ROWS]) {
        for (i, row) in board_set.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                self.board[i][j] = *val;
            }
        }
    }
}

impl Default for Sudoku {
    fn default() -> Sudoku {
        Sudoku {
            // Defaults to zeros (i.e. unsolved)
            board: [[NUM_UNSET; NUM_COLS]; NUM_ROWS],        
        }
    }
}

// Old internal tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test verify_cell() to make sure it generally works as expected
    fn test_verify_cell_basic() {
        let mut cur = Sudoku { ..Default::default() };
        // Populate first row 1-9, remaining 2-9, 1 so verify should work
        cur.board[0] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        for i in 1..NUM_MAX {
            cur.board[usize::from(i)] = [2, 3, 4, 5, 6, 7, 8, 9, 1];
        }

        // Should pass since no 1 in column, row, box
        assert_eq!(true, cur.verify_cell(0, 0));

        // Second row first col should fail
        assert_eq!(false, cur.verify_cell(1, 0));
    }

    
    #[test]
    // Check the full verify function with a couple examples from online
    fn test_verify_full() {
        let mut cur = Sudoku { ..Default::default() };

        // From https://leetcode.com/problems/valid-sudoku/
        cur.board = [[5,3,0,0,7,0,0,0,0]
        ,[6,0,0,1,9,5,0,0,0]
        ,[0,9,8,0,0,0,0,6,0]
        ,[8,0,0,0,6,0,0,0,3]
        ,[4,0,0,8,0,3,0,0,1]
        ,[7,0,0,0,2,0,0,0,6]
        ,[0,6,0,0,0,0,2,8,0]
        ,[0,0,0,4,1,9,0,0,5]
        ,[0,0,0,0,8,0,0,7,9]];
        assert_eq!(true, cur.verify());

        // From https://leetcode.com/problems/valid-sudoku/
        cur.board = [[8,3,0,0,7,0,0,0,0]
        ,[6,0,0,1,9,5,0,0,0]
        ,[0,9,8,0,0,0,0,6,0]
        ,[8,0,0,0,6,0,0,0,3]
        ,[4,0,0,8,0,3,0,0,1]
        ,[7,0,0,0,2,0,0,0,6]
        ,[0,6,0,0,0,0,2,8,0]
        ,[0,0,0,4,1,9,0,0,5]
        ,[0,0,0,0,8,0,0,7,9]];
        assert_eq!(false, cur.verify());

        // From https://www.geeksforgeeks.org/check-if-given-sudoku-board-configuration-is-valid-or-not/
        cur.board = [[ 5, 3, 0, 0, 7, 0, 0,0,0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9]];
        assert_eq!(true, cur.verify());
    }
}

