// Sudoku parsing script
use std::io;
use std::fs;
// use std::path::Path;

const SUDOKU_DIR: &str = "./sudokus";
const SOLN_DIR: &str = "./solutions";

const NUM_ROWS: usize = 9;
const NUM_COLS: usize = 9;

// Hacky conversion from u8 to char
const CHAR_TO_U8_OFFSET: u8 = 48;

fn main() -> io::Result<()> {
    // Parse sudokus
    for file in fs::read_dir(SUDOKU_DIR)? {
        let file = file?;
        println!("{:?}", file.path());
        
        // Read numbers from file
        let contents = fs::read_to_string(file.path())?;
        let nums = contents.split(" ").collect::<Vec<&str>>();
        
        let mut board: [[u8; NUM_COLS]; NUM_ROWS] = [[0; NUM_COLS]; NUM_ROWS]; // sudoku board
        
        // Parse chars into board as u8's
        for i in 0..(NUM_ROWS*NUM_COLS) {
            let num_char: char = nums[i].chars().nth(nums[i].len() - 1).expect("error idk");
            board[i / 9][i % 9] = num_char as u8 - CHAR_TO_U8_OFFSET;
        }
        println!("{:?}", board);
    }
    Ok(())
}