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
                print!("{} ", val);
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
    test_disp();
    test_possible();
    println!("{}", MASK_FULL);
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

// Expected output:
// [1, 2, 3, 4, 5, 6, 7, 8, 9]
fn test_possible() {
    let cur = Sudoku { ..Default::default() };
    let possible = cur.get_possible(0, 0);
    println!("{:?}", possible);
}

