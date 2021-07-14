// Basic sudoku solver

const NUM_ROWS: usize = 9;
const NUM_COLS: usize = 9;

// Indices to print divider on (after 3rd, after 6th)
const PRINT_IDX: [usize; 2] = [2, 5];

// For output formatting
const ROW_DIVIDER: &str = "\n---------------------";
const COL_DIVIDER: &str = "| ";

pub struct Sudoku {
    board: [[u8; NUM_COLS]; NUM_ROWS],
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
}

impl Default for Sudoku {
    fn default() -> Sudoku {
        Sudoku {
            // Defaults to zeros (i.e. unset)
            board: [[0; NUM_COLS]; NUM_ROWS],
        }
    }
}


fn main() {
    test_disp();
}

fn test_disp() {
    let mut cur = Sudoku { ..Default::default() };
    for row in &mut cur.board {
        *row = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    }
    cur.display();
}

