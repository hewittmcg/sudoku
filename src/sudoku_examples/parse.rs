// Sudoku parsing script for sudokus from http://lipas.uwasa.fi/~timan/sudoku
use std::fs;

use crate::sudoku::{Sudoku, NUM_ROWS, NUM_COLS};

const SUDOKU_DIR: &str = "./src/sudoku_examples/sudokus";

// Currently unused
// const SOLN_DIR: &str = "./src/sudoku_examples/solutions";

// Hacky conversion from u8 to char
const CHAR_TO_U8_OFFSET: u8 = b'0';

// Parse sudokus from ./sudokus and return.
pub fn parse() -> Vec<Sudoku> {
    let mut sudokus: Vec<Sudoku> = Vec::new();

    let files = fs::read_dir(SUDOKU_DIR)
    .expect("Error reading dir");

    for file in files {
        let file = file.unwrap();

        // Read numbers from file
        let contents = fs::read_to_string(file.path()).unwrap();
        let nums = contents.split(' ').collect::<Vec<&str>>();
        
        let mut board: [[u8; NUM_COLS]; NUM_ROWS] = [[0; NUM_COLS]; NUM_ROWS]; // sudoku board
        
        // Parse chars into board as u8's
        for i in 0..(NUM_ROWS*NUM_COLS) {
            let num_char: char = nums[i].chars().nth(nums[i].len() - 1)
                .expect("Error converting char");
            board[i / 9][i % 9] = num_char as u8 - CHAR_TO_U8_OFFSET;
        }
        sudokus.push(Sudoku::new(board));
    }
    sudokus
}
