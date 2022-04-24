const N: i32 = 6;
const M: i32 = 6;

fn print_board(board: &Vec<Vec<i32>>) {
    for i in 0..board.len() {
        print!("|");
        for j in 0..board.len() {
            print!("{0: <3}| ", board[i][j]);
        }
        println!("");
    }
}

fn is_possible(i: i32, j: i32) -> bool {
    return 0 <= i && i < N && 0 <= j && j < M;
}

fn generate_knight_moves(board: &mut Vec<Vec<i32>>, istart: i32, jstart: i32) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    
    result.push((istart - 2, jstart - 1));
    result.push((istart - 2, jstart + 1));
    result.push((istart - 1, jstart + 2));
    result.push((istart + 1, jstart + 2));
    result.push((istart + 2, jstart + 1));
    result.push((istart + 2, jstart - 1));
    result.push((istart + 1, jstart - 2));
    result.push((istart - 1, jstart - 2));

    result.retain(|&ij| is_possible(ij.0, ij.1) && board[ij.0 as usize][ij.1 as usize] == 0);

    return result;
}

fn solve_implementation(board_result: &mut Vec<Vec<i32>>, istart: i32, jstart: i32, move_count: i32) -> bool {
    let possibilities = generate_knight_moves(board_result, istart, jstart);
    for p in possibilities.iter() {
        board_result[p.0 as usize][p.1 as usize] = move_count;
        if solve_implementation(board_result, p.0, p.1, move_count + 1) {
            return true;
        }
        board_result[p.0 as usize][p.1 as usize] = 0;
    }
    return move_count - 1 == N * M;
}

fn solve(board_result: &mut Vec<Vec<i32>>) {
    board_result[0][0] = 1;
    solve_implementation(board_result, 0, 0, 2);
}

fn main() {
    let mut board = vec![vec![0i32; N as usize]; M as usize];
    solve(&mut board);
    print_board(&board);
}