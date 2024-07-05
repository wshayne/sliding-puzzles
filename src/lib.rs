use std::collections::VecDeque;
use std::fmt::Display;

use array2d::Array2D;

mod soccer_puzzle;
pub use soccer_puzzle::Game as SoccerPuzzle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone)]
pub struct Board {
    board: Array2D<u32>,
    blank_pos: [usize; 2],
    moves: Vec<Move>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in self.board.rows_iter() {
            for i in row {
                s += &format!("{:3}", i);
            }
            s += "\n";
        }
        f.write_str(&s)
    }
}

impl Board {
    pub fn new(board: Array2D<u32>) -> Self {
        let mut blank_pos = [0, 0];
        for (i, v) in board.elements_row_major_iter().enumerate() {
            if *v == 0 {
                blank_pos = [i % board.row_len(), i / board.row_len()];
                break;
            }
        }
        Self {
            board,
            blank_pos,
            moves: Vec::new(),
        }
    }

    fn make_move(&mut self, m: Move) {
        match m {
            Move::Left => {
                if self.blank_pos[0] > 0 {
                    self.board
                        .set(
                            self.blank_pos[1],
                            self.blank_pos[0],
                            *self
                                .board
                                .get(self.blank_pos[1], self.blank_pos[0] - 1)
                                .unwrap(),
                        )
                        .unwrap();
                    self.board
                        .set(self.blank_pos[1], self.blank_pos[0] - 1, 0)
                        .unwrap();
                    self.blank_pos[0] -= 1;
                }
            }
            Move::Right => {
                if self.blank_pos[0] < self.board.row_len() - 1 {
                    self.board
                        .set(
                            self.blank_pos[1],
                            self.blank_pos[0],
                            *self
                                .board
                                .get(self.blank_pos[1], self.blank_pos[0] + 1)
                                .unwrap(),
                        )
                        .unwrap();
                    self.board
                        .set(self.blank_pos[1], self.blank_pos[0] + 1, 0)
                        .unwrap();
                    self.blank_pos[0] += 1;
                }
            }
            Move::Up => {
                if self.blank_pos[1] > 0 {
                    self.board
                        .set(
                            self.blank_pos[1],
                            self.blank_pos[0],
                            *self
                                .board
                                .get(self.blank_pos[1] - 1, self.blank_pos[0])
                                .unwrap(),
                        )
                        .unwrap();
                    self.board
                        .set(self.blank_pos[1] - 1, self.blank_pos[0], 0)
                        .unwrap();
                    self.blank_pos[1] -= 1;
                }
            }
            Move::Down => {
                if self.blank_pos[1] < self.board.column_len() - 1 {
                    self.board
                        .set(
                            self.blank_pos[1],
                            self.blank_pos[0],
                            *self
                                .board
                                .get(self.blank_pos[1] + 1, self.blank_pos[0])
                                .unwrap(),
                        )
                        .unwrap();
                    self.board
                        .set(self.blank_pos[1] + 1, self.blank_pos[0], 0)
                        .unwrap();
                    self.blank_pos[1] += 1;
                }
            }
        }
        self.moves.push(m);
    }

    fn find(&self, val: u32) -> Result<[usize; 2], u32> {
        for (j, row) in self.board.rows_iter().enumerate() {
            for (i, v) in row.enumerate() {
                if val == *v {
                    return Ok([i, j]);
                }
            }
        }
        return Err(val);
    }

    fn move_piece(&mut self, piece: u32, mut dest: [usize; 2]) {
        let mut piece_start = self.find(piece).unwrap();

        // If our destination is the right edge of the board, we will instead move it to the second column from the right
        // and shift left as our final move
        let mut final_left = false;
        if dest[0] == self.board.row_len() - 1 {
            dest[0] -= 1;
            final_left = true;
        }

        // Standardize the position we are starting from: place the blank tile to the right of the tile being moved
        // This isn't necessarily the most efficient technique, if we are moving a piece to the far right column
        // of the board, then we could have unnecessary swaps

        // If the piece and the blank are in the same column
        if piece_start[0] == self.blank_pos[0] {
            // If the blank isn't on the right edge, move it to the column to the right of the piece
            if piece_start[0] != self.board.row_len() - 1 {
                self.make_move(Move::Right);
                if piece_start[1] > self.blank_pos[1] {
                    for _ in 0..piece_start[1] - self.blank_pos[1] {
                        self.make_move(Move::Down);
                    }
                } else {
                    for _ in 0..self.blank_pos[1] - piece_start[1] {
                        self.make_move(Move::Up);
                    }
                }
            }
            // If the blank is on the right edge, want to get it to the left of the piece then move right
            else {
                // If the piece is above the blank, then simply move left, then up, then right
                if piece_start[1] < self.blank_pos[1] {
                    self.make_move(Move::Left);
                    for _ in 0..self.blank_pos[1] - piece_start[1] {
                        self.make_move(Move::Up);
                    }
                    self.make_move(Move::Right);
                    piece_start[0] -= 1;
                }
                // If the piece is below the blank, we want to go down 1 if we can
                else if piece_start[1] > self.blank_pos[1] + 1 {
                    self.make_move(Move::Down);
                    self.make_move(Move::Left);
                    for _ in 0..piece_start[1] - self.blank_pos[1] - 1 {
                        self.make_move(Move::Down);
                    }
                    self.make_move(Move::Right);
                    piece_start[0] -= 1;
                }
                // Otherwise assume an initial left is safe
                else {
                    self.make_move(Move::Left);
                    for _ in 0..piece_start[1] - self.blank_pos[1] {
                        self.make_move(Move::Down);
                    }
                    self.make_move(Move::Right);
                    piece_start[0] -= 1;
                }
            }
        } else {
            // If the blank is above the piece, move it down into the same row
            if piece_start[1] > self.blank_pos[1] {
                for _ in 0..piece_start[1] - self.blank_pos[1] {
                    self.make_move(Move::Down);
                }
            }
            // If the blank is to the right of the piece, move to the square to the right
            if self.blank_pos[0] > piece_start[0] {
                for _ in 0..self.blank_pos[0] - piece_start[0] - 1 {
                    self.make_move(Move::Left);
                }
                // If the blank is below the piece, move it up to the same row
                for _ in 0..self.blank_pos[1] - piece_start[1] {
                    self.make_move(Move::Up);
                }
            } else {
                // If the blank is to the left of the piece, move it so it is to the left of the piece
                for _ in 0..piece_start[0] - self.blank_pos[0] - 1 {
                    self.make_move(Move::Right);
                }
                // If the blank is below the piece, move it up to the same row
                for _ in 0..self.blank_pos[1] - piece_start[1] {
                    self.make_move(Move::Up);
                }
                // Swap the blank and the piece
                self.make_move(Move::Right);
                piece_start[0] -= 1;
            }
        }

        // Figure out what directions the piece needs to be moved in
        let mut r = 0;
        let mut l = 0;
        let mut u = 0;
        let mut d = 0;

        if piece_start[0] > dest[0] {
            l = piece_start[0] - dest[0];
        } else {
            r = dest[0] - piece_start[0];
        }

        if piece_start[1] > dest[1] {
            u = piece_start[1] - dest[1];
        } else {
            d = dest[1] - piece_start[1];
        }

        // Ignoring diagonal moves for now
        // let ur = cmp::min(u, r);
        // u -= ur;
        // r -= ur;

        // let ul = cmp::min(u, l);
        // u -= ul;
        // l -= ul;

        // // This should always be 0
        // let dr = cmp::min(d, r);
        // d -= dr;
        // r -= dr;

        // let dl = cmp::min(d, l);
        // d -= dl;
        // l -= dl;

        let on_bottom = piece_start[1] == self.board.column_len() - 1;

        // Move order: right, up, down, left

        for _ in 0..r {
            // Move right
            self.make_move(Move::Left);
            if on_bottom {
                self.make_move(Move::Up);
            } else {
                self.make_move(Move::Down);
            }
            self.make_move(Move::Right);
            self.make_move(Move::Right);
            if on_bottom {
                self.make_move(Move::Down);
            } else {
                self.make_move(Move::Up);
            }
        }

        for _ in 0..u {
            // Move up
            self.make_move(Move::Up);
            self.make_move(Move::Left);
            self.make_move(Move::Down);
            self.make_move(Move::Right);
            self.make_move(Move::Up);
        }

        for _ in 0..d {
            // Move down
            self.make_move(Move::Down);
            self.make_move(Move::Left);
            self.make_move(Move::Up);
            self.make_move(Move::Right);
            self.make_move(Move::Down);
        }

        let on_bottom = dest[1] == self.board.column_len() - 1;
        // If moving left, we do it from the left side of the piece
        // Change to move from bottom: left is interfering
        if l > 0 {
            self.make_move(Move::Left);
            for _ in 0..l {
                // Move left
                self.make_move(Move::Right);
                if on_bottom {
                    self.make_move(Move::Up);
                } else {
                    self.make_move(Move::Down);
                }
                self.make_move(Move::Left);
                self.make_move(Move::Left);
                if on_bottom {
                    self.make_move(Move::Down);
                } else {
                    self.make_move(Move::Up);
                }
            }
            self.make_move(Move::Right);
        }

        if final_left {
            self.make_move(Move::Left);
        }
    }

    fn solve_row(&mut self, row: usize) {
        for piece in 1 + row * self.board.row_len() + row..(row + 1) * self.board.row_len() - 1 {
            self.move_piece(piece as u32, [(piece - 1) % self.board.row_len(), row]);
        }
        self.move_piece(
            ((row + 1) * self.board.row_len() - 1) as u32,
            [self.board.row_len() - 1, row],
        );
        self.move_piece(
            ((row + 1) * self.board.row_len()) as u32,
            [self.board.row_len() - 1, row + 1],
        );
        self.make_move(Move::Up);
        self.make_move(Move::Right);
        self.make_move(Move::Down);
    }

    fn solve_col(&mut self, col: usize) {
        for piece in (self.board.row_len() * (col + 1) + col + 1
            ..self.board.row_len() * (self.board.column_len() - 2) + col + 1)
            .step_by(self.board.row_len())
        {
            self.move_piece(piece as u32, [col, piece / self.board.row_len()]);
        }
        let last = self.board.row_len() * (self.board.column_len() - 1) + 1 + col;
        let second_last = last - self.board.row_len();
        self.move_piece(second_last as u32, [col, self.board.column_len() - 1]);
        self.move_piece(last as u32, [col + 1, self.board.column_len() - 1]);
        self.make_move(Move::Up);
        self.make_move(Move::Left);
        self.make_move(Move::Left);
        self.make_move(Move::Down);
        self.make_move(Move::Right);
    }

    fn brute_force_corner(&mut self) {
        let mut a_star = VecDeque::new();
        a_star.push_back((self.moves.last().unwrap().clone(), self.clone()));
        loop {
            let (prev, start) = a_star.pop_front().unwrap();
            // println!("{}", start);
            if prev != Move::Down && start.blank_pos[1] >= self.board.column_len() - 2 {
                let mut up = start.clone();
                up.make_move(Move::Up);
                if up.check_solved() {
                    *self = up;
                    return;
                } else {
                    a_star.push_back((Move::Up, up));
                }
            }
            if prev != Move::Up && start.blank_pos[1] <= self.board.column_len() - 2 {
                let mut down = start.clone();
                down.make_move(Move::Down);
                if down.check_solved() {
                    *self = down;
                    return;
                } else {
                    a_star.push_back((Move::Down, down));
                }
            }
            if prev != Move::Left && start.blank_pos[0] <= self.board.row_len() - 2 {
                let mut right = start.clone();
                right.make_move(Move::Right);
                if right.check_solved() {
                    *self = right;
                    return;
                } else {
                    a_star.push_back((Move::Right, right));
                }
            }
            if prev != Move::Right && start.blank_pos[0] >= self.board.row_len() - 2 {
                let mut left = start.clone();
                left.make_move(Move::Left);
                if left.check_solved() {
                    *self = left;
                    return;
                } else {
                    a_star.push_back((Move::Left, left));
                }
            }
        }
    }

    fn check_solved(&self) -> bool {
        self.board
            .as_row_major()
            .into_iter()
            .filter(|i| *i != 0)
            .collect::<Vec<u32>>()
            .as_slice()
            .windows(2)
            .all(|w| w[0] < w[1])
    }

    pub fn print_moves(&self) -> String {
        let mut s = String::new();
        for m in self.moves.iter() {
            match m {
                Move::Left => s += "L,",
                Move::Right => s += "R,",
                Move::Up => s += "U,",
                Move::Down => s += "D,",
            }
        }
        s
    }

    pub fn solve(&mut self) {
        for i in 0..self.board.column_len() - 3 {
            self.solve_row(i);
            self.solve_col(i);
        }
        self.brute_force_corner();
    }
}
