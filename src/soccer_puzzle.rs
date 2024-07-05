use std::collections::{HashMap, VecDeque};

use array2d::Array2D;

#[derive(Copy, Clone, Debug, Eq, derivative::Derivative)]
#[derivative(PartialEq, Hash)]
enum Piece {
    Tall(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        usize,
    ),
    Small,
    Blank,
    Wide,
    Large,
}

#[derive(Copy, Clone, Debug)]
pub struct Move {
    start_1: [usize; 2],
    end_1: [usize; 2],
    start_2: Option<[usize; 2]>,
    end_2: Option<[usize; 2]>,
}

#[derive(Clone, Debug)]
pub struct Game {
    /// 2D array of Piece objects representing the board
    board: Array2D<Piece>,
    /// Positions of the blank squares in the board, stored as [row, column] order
    blank_pos: ([usize; 2], [usize; 2]),
    /// Vector of moves made to get to this position
    pub moves: Vec<Move>
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in self.board.rows_iter() {
            s += "|";
            for piece in row {
                match piece {
                    Piece::Tall(i) => s += &format!("T{}|", i),
                    Piece::Small => s += "SS|",
                    Piece::Blank => s += "  |",
                    Piece::Wide => s += "WW|",
                    Piece::Large => s += "LL|",
                }
            }
            s += "\n";
        }

        f.write_str(&s)
    }
}

impl Game {
    /// Returns a new Game with the default block positions
    pub fn new() -> Self {
        let board = Array2D::from_rows(&vec![vec![Piece::Tall(0), Piece::Large, Piece::Large, Piece::Tall(1)], vec![Piece::Tall(0), Piece::Large, Piece::Large, Piece::Tall(1)], vec![Piece::Blank, Piece::Wide, Piece::Wide, Piece::Blank], vec![Piece::Tall(2), Piece::Small, Piece::Small, Piece::Tall(3)], vec![Piece::Tall(2), Piece::Small, Piece::Small, Piece::Tall(3)]]).unwrap();
        let blank_pos = ([2, 0], [2, 3]);
        Self {board, blank_pos, moves: vec![]}
    }

    /// Returns a list of all possible moves from the given position
    pub fn list_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        let (start_1, start_2) = self.blank_pos;
        let ([b1_row, b1_col], [b2_row, b2_col]) = self.blank_pos;
        let right = self.board.get(b1_row, b1_col + 1);
        match right {
            Some(Piece::Small) => moves.push(Move {start_1, end_1: [b1_row, b1_col + 1], start_2: None, end_2: None}),
            Some(Piece::Wide) => moves.push(Move {start_1, end_1: [b1_row, b1_col + 2], start_2: None, end_2: None}),
            Some(Piece::Large) => {
                if self.board.get(b2_row, b2_col + 1) == Some(&Piece::Large) {
                    moves.push(Move {start_1, end_1: [b1_row, b1_col + 2], start_2: Some(start_2), end_2: Some([b2_row, b2_col + 2])});
                }
            },
            Some(Piece::Tall(i)) => {
                if let Some(Piece::Tall(i_2)) =  self.board.get(b2_row, b2_col + 1) {
                    if *i == *i_2 {
                        moves.push(Move {start_1, end_1: [b1_row, b1_col + 1], start_2: Some(start_2), end_2: Some([b2_row, b2_col + 1])});
                    }
                }
            }
            _ => {}
        }
        let left = self.board.get(b1_row, b1_col.wrapping_sub(1));
        match left {
            Some(Piece::Small) => moves.push(Move {start_1, end_1: [b1_row, b1_col - 1], start_2: None, end_2: None}),
            Some(Piece::Wide) => moves.push(Move {start_1, end_1: [b1_row, b1_col - 2], start_2: None, end_2: None}),
            Some(Piece::Large) => {
                if self.board.get(b2_row, b2_col.wrapping_sub(1)) == Some(&Piece::Large) {
                    moves.push(Move {start_1, end_1: [b1_row, b1_col - 2], start_2: Some(start_2), end_2: Some([b2_row, b2_col - 2])});
                }
            },
            Some(Piece::Tall(i)) => {
                if let Some(Piece::Tall(i_2)) = self.board.get(b2_row, b2_col.wrapping_sub(1)) {
                    if *i == *i_2 {
                        moves.push(Move {start_1, end_1: [b1_row, b1_col.wrapping_sub(1)], start_2: Some(start_2), end_2: Some([b2_row, b2_col - 1])});
                    }
                }
            }
            _ => {}
        }
        let up = self.board.get(b1_row.wrapping_sub(1), b1_col);
        match up {
            Some(Piece::Small) => moves.push(Move {start_1, end_1: [b1_row - 1, b1_col], start_2: None, end_2: None}),
            Some(Piece::Tall(_)) => moves.push(Move {start_1, end_1: [b1_row - 2, b1_col], start_2: None, end_2: None}),
            Some(Piece::Large) => {
                if self.board.get(b2_row.wrapping_sub(1), b2_col) == Some(&Piece::Large) {
                    moves.push(Move {start_1, end_1: [b1_row - 2, b1_col], start_2: Some(start_2), end_2: Some([b2_row - 2, b2_col])});
                }
            },
            Some(Piece::Wide) => {
                if self.board.get(b2_row.wrapping_sub(1), b2_col) == Some(&Piece::Wide) {
                    moves.push(Move {start_1, end_1: [b1_row - 1, b1_col], start_2: Some(start_2), end_2: Some([b2_row - 1, b2_col])});
                }
            },
            _ => {}
        }
        let down = self.board.get(b1_row + 1, b1_col);
        match down {
            Some(Piece::Small) => moves.push(Move {start_1, end_1: [b1_row + 1, b1_col], start_2: None, end_2: None}),
            Some(Piece::Tall(_)) => moves.push(Move {start_1, end_1: [b1_row + 2, b1_col], start_2: None, end_2: None}),
            Some(Piece::Large) => {
                if self.board.get(b2_row + 1, b2_col) == Some(&Piece::Large) {
                    moves.push(Move {start_1, end_1: [b1_row + 2, b1_col], start_2: Some(start_2), end_2: Some([b2_row + 2, b2_col])});
                }
            },
            Some(Piece::Wide) => {
                if self.board.get(b2_row + 1, b2_col) == Some(&Piece::Wide) {
                    moves.push(Move {start_1, end_1: [b1_row + 1, b1_col], start_2: Some(start_2), end_2: Some([b2_row + 1, b2_col])});
                }
            },
            _ => {}
        }

        let b2_right = self.board.get(b2_row, b2_col + 1);
        match b2_right {
            Some(Piece::Small) => moves.push(Move {start_1: start_2, end_1: [b2_row, b2_col + 1], start_2: None, end_2: None}),
            Some(Piece::Wide) => moves.push(Move {start_1: start_2, end_1: [b2_row, b2_col + 2], start_2: None, end_2: None}),
            _ => {}
        }

        let b2_left = self.board.get(b2_row, b2_col.wrapping_sub(1));
        match b2_left {
            Some(Piece::Small) => moves.push(Move {start_1: start_2, end_1: [b2_row, b2_col - 1], start_2: None, end_2: None}),
            Some(Piece::Wide) => moves.push(Move {start_1: start_2, end_1: [b2_row, b2_col - 2], start_2: None, end_2: None}),
            _ => {}
        }

        let b2_up = self.board.get(b2_row.wrapping_sub(1), b2_col);
        match b2_up {
            Some(Piece::Small) => moves.push(Move {start_1: start_2, end_1: [b2_row - 1, b2_col], start_2: None, end_2: None}),
            Some(Piece::Tall(_)) => moves.push(Move {start_1: start_2, end_1: [b2_row - 2, b2_col], start_2: None, end_2: None}),
            _ => {}
        }

        let b2_down = self.board.get(b2_row + 1, b2_col);
        match b2_down {
            Some(Piece::Small) => moves.push(Move {start_1: start_2, end_1: [b2_row + 1, b2_col], start_2: None, end_2: None}),
            Some(Piece::Tall(_)) => moves.push(Move {start_1: start_2, end_1: [b2_row + 2, b2_col], start_2: None, end_2: None}),
            _ => {}
        }
        moves
    }

    /// Executes a Move by swapping the Pieces at the start and end positions
    pub fn make_move(&mut self, m: Move) {
        let start = m.start_1;
        let end = m.end_1;
        let _  = self.board.set(start[0], start[1], *self.board.get(end[0], end[1]).unwrap());
        let _ = self.board.set(end[0], end[1], Piece::Blank);
        if self.blank_pos.0 == start {
            self.blank_pos.0 = end;
        } else if self.blank_pos.1 == start {
            self.blank_pos.1 = end;
        }
        if let (Some(start), Some(end)) = (m.start_2, m.end_2) {
            let _ = self.board.set(start[0], start[1], *self.board.get(end[0], end[1]).unwrap());
            let _ = self.board.set(end[0], end[1], Piece::Blank);
            self.blank_pos.1 = end;
        }
        self.moves.push(m);
    }

    /// Checks if the large piece is in the solved position
    fn check_solved(&self) -> bool {
        if let (Some(a), Some(b), Some(c), Some(d)) = (self.board.get(3, 1), self.board.get(3, 2), self.board.get(4, 1), self.board.get(4, 2)) {
            return *a == Piece::Large && *b == Piece::Large && *c == Piece::Large && *d == Piece::Large;
        }
        false
    }

    /// Performs a brute-force a* solve with pruning using a hash of the board's layout
    pub fn solve(&mut self) {
        let mut a_star = VecDeque::new();
        let mut found_states = HashMap::<Array2D<Piece>, bool>::new();
        found_states.insert(self.board.clone(), true);
        a_star.push_back(self.clone());
        loop {
            let start = a_star.pop_front().unwrap();
            for m in start.list_moves() {
                let mut moved = start.clone();
                moved.make_move(m);
                if let Some(_) = found_states.insert(moved.board.clone(), true) {
                } else if moved.check_solved() {
                    *self = moved;
                    return;
                } else {
                    a_star.push_back(moved);
                }
            }
        }
    }
}