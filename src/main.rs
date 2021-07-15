// Basic sudoku solver

const NUM_ROWS: usize = 9;
const NUM_COLS: usize = 9;

// Indices to print divider on (after 3rd, after 6th)
const PRINT_IDX: [usize; 2] = [2, 5];

// For output formatting
const ROW_DIVIDER: &str = "\n---------------------";
const COL_DIVIDER: &str = "| ";

// Indices 1-9 filled, 0 left empty for simplicity
const MASK_FULL: u16 = 0x1FF << 1;

// Cell values
const NUM_UNSET: u8 = 0;
const NUM_MIN: u8 = 1;
const NUM_MAX: u8 = 9;

pub struct Sudoku {
    // The board -- 0 = unsolved/unset
    board: [[u8; NUM_COLS]; NUM_ROWS],
    
    // Options available at each location (bitmask)
    possibles: [[u16; NUM_COLS]; NUM_ROWS],
}

impl Sudoku {
    // Display the sudoku
    fn display(&self) {
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

    // Return a vector of the options at a location
    fn get_possible(&self, row:usize, col:usize) -> Vec<u8> {
        let val = self.possibles[row][col];
        let mut ret = Vec::<u8>::new();
        for i in NUM_MIN..NUM_MAX+1 {
            if val & (1 << i) != 0 {
                ret.push(i);
            }
        }
        ret
    }

    // Verify whether an individual cell is valid.
    fn verify_cell(&self, row:usize, col:usize) -> bool {
        let cell_val = self.board[row][col];

        // Ignore if unset
        if cell_val == 0 {
            return true;
        }

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

    // Verify whether the entire sudoku is valid.
    fn verify(&self) -> bool {
        for i in 0..usize::from(NUM_MAX) {
            for j in 0..usize::from(NUM_MAX) {
                if !self.verify_cell(i, j) {
                    return false;
                }
            }
        }
        return true;
    }
}

impl Default for Sudoku {
    fn default() -> Sudoku {
        Sudoku {
            // Defaults to zeros (i.e. unsolved)
            board: [[NUM_UNSET; NUM_COLS]; NUM_ROWS],

            // All options possible
            possibles: [[MASK_FULL; NUM_COLS]; NUM_ROWS],
        }
    }
}


fn main() {
    // let mut cur = Sudoku { .. Default::default() };
    // for row in &mut cur.board {
    //     *row = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    // }

    // cur.solve_brute_force();
    // cur.display();
    test_normal_disp();
}

// Expected output:
// 1 2 3 | 4 5 6 | 7 8 9
// 1 2 3 | 4 5 6 | 7 8 9
// 1 2 3 | 4 5 6 | 7 8 9
// ---------------------
// 1 2 3 | 4 5 6 | 7 8 9
// 1 2 3 | 4 5 6 | 7 8 9
// 1 2 3 | 4 5 6 | 7 8 9
// ---------------------
// 1 2 3 | 4 5 6 | 7 8 9
// 1 2 3 | 4 5 6 | 7 8 9
// 1 2 3 | 4 5 6 | 7 8 9
fn test_disp() {
    let mut cur = Sudoku { ..Default::default() };
    // Populate all rows with 1-9
    for row in &mut cur.board {
        *row = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    }
    cur.display();
}

// Test outputting a general unsolved sudoku
fn test_normal_disp() {
    let cur = Sudoku {
        board: [[5,3,0,0,7,0,0,0,0]
        ,[6,0,0,1,9,5,0,0,0]
        ,[0,9,8,0,0,0,0,6,0]
        ,[8,0,0,0,6,0,0,0,3]
        ,[4,0,0,8,0,3,0,0,1]
        ,[7,0,0,0,2,0,0,0,6]
        ,[0,6,0,0,0,0,2,8,0]
        ,[0,0,0,4,1,9,0,0,5]
        ,[0,0,0,0,8,0,0,7,9]],

        ..Default::default()
    };
    cur.display();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Make sure get_possible returns all values by default
    fn test_possible() {
        let cur = Sudoku { ..Default::default() };
        let possible = cur.get_possible(0, 0);
        let expected:Vec<u8> = [1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec();
        assert_eq!(expected, possible);
    }

    #[test]
    // Test verify_cell() to make sure it generally work sas expected
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

