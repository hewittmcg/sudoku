// Test solving some basic sudokus

use sudoku::sudoku::Sudoku;

use sudoku::sudoku_examples::parse;

#[test]
fn test_solve_all() {
    // Solve the sudokus from http://lipas.uwasa.fi/~timan/sudoku
    let sudokus: Vec<Sudoku> = parse::parse();
    for mut sud in sudokus {
        println!("Before solving:");
        sud.display();
        sud.solve();
        assert!(sud.verify());
        println!("After solving:");
        sud.display();
    }
}