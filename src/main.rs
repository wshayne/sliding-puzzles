use array2d::Array2D;
use sliding_puzzle_solver::*;

fn main() {
    // // some move is not working for this board, need to check it out later
    // let rows = vec![vec![16, 12, 7, 0, 5], vec![6, 10, 4, 1, 9], vec![21, 11, 24, 19, 3], vec![17, 14, 2, 18, 15], vec![13, 23, 22, 8, 20]];
    // // let rows = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20], vec![21, 22, 23, 24, 0]];
    // let board_array = Array2D::from_rows(&rows).unwrap();
    // let mut board = Board::new(board_array);

    // println!("{}", board);
    // board.solve();
    // println!("{}", board);
    // println!("{}", board.print_moves());

    let mut g = SoccerPuzzle::new();
    println!("{}", g);
    let start = std::time::Instant::now();
    g.solve();
    println!("{:?}", start.elapsed());
    // let mut demo = SoccerPuzzle::new();
    // for m in g.moves {
    //     demo.make_move(m);
    //     println!("{}", demo);
    // }
}
