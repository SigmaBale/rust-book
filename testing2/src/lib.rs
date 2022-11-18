pub mod solver {
    pub fn solve(board: [[i8;9];9]) -> [[i8;9];9] {
        let mut result = board;
        let empty_cell: [usize; 2] = find_empty_cell(board);
    
        if is_solved(board) {
            return result;
        }
    
        let row: usize = empty_cell[0];
        let column: usize = empty_cell[1];
    
        for guess in guesses(board, row, column) {
            result[row][column] = guess;
            result = solve(result);
            if is_solved(result) {
                return result;
            }
        }
        result = board;
        return result;
    }

    pub fn is_valid(board: [[i8;9];9], row: usize, collumn: usize, guess: i8) -> bool {

        for x in 0..9 {
            if guess == board[row][x] || guess == board[x][collumn] {
                return false
            }
        }
        let x_index = row / 3 * 3;
        let y_index = collumn / 3 * 3;

        for x in 0..3 {
            for y in 0..3 {
                if guess == board[x_index + x][y_index + y] {
                    return false
                } 
            }
        }
        true
    }
    pub fn find_empty_cell(board: [[i8;9];9]) -> [usize; 2] {
        for x in 0..9 {
            for y in 0..9 {
                if board[x][y] == 0 {
                    return [x, y];
                }
            }
        }
        [10,10]
    }

    pub fn is_solved(board: [[i8;9];9]) -> bool {
        find_empty_cell(board) == [10,10]
    }
    
    fn guesses(board: [[i8;9];9], row: usize, column: usize) -> Vec<i8> {
        let mut result = vec![];
        for guess in 1..10 {
            if is_valid(board, row, column, guess) {
                result.push(guess);
            }
        }
	    result
    }
    pub fn fibonnaci(input: i64) {
        let mut starting = 0;
        let mut previous = 1;
        println!("{starting}");
        println!("{previous}");
        for _ in 2..input {
            let next = starting + previous;
            println!("{next}");
            starting = previous;
            previous = next;
        }
    }

}



#[test]
fn difficulty_easy_sudoku() {
    use crate::solver::solve;
    let easy_sudoku_solved = [
        [9,8,4,2,7,6,5,3,1],
        [6,1,3,9,4,5,8,2,7],
        [2,5,7,1,3,8,6,4,9],
        [8,3,2,7,5,1,4,9,6],
        [7,4,5,6,9,3,2,1,8],
        [1,9,6,4,8,2,7,5,3],
        [3,7,8,5,1,4,9,6,2],
        [4,2,9,3,6,7,1,8,5],
        [5,6,1,8,2,9,3,7,4]
    ];
    let easy_sudoku = [
        [9,8,4,2,7,0,0,3,1],
        [6,1,3,9,4,5,0,2,0],
        [2,5,7,1,3,8,0,0,9],
        [8,3,2,7,5,0,4,9,0],
        [0,4,0,0,9,0,0,1,8],
        [0,0,6,0,8,2,0,0,3],
        [3,7,8,0,1,0,9,0,0],
        [4,0,0,0,0,7,0,0,0],
        [5,6,0,0,0,0,0,0,4]
    ];
    let board = solve(easy_sudoku);

    assert_eq!(easy_sudoku_solved, board)
}