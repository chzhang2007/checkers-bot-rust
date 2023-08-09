use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Black(bool), //crowned = true, uncrowned = false
    White(bool),
}
impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl From<Piece> for char {
    fn from(piece: Piece) -> char {
        match piece {
            Piece::Black(false) => 'b',
            Piece::Black(true) => 'B',
            Piece::White(false) => 'w',
            Piece::White(true) => 'W',
        }
    }
}
#[derive(Clone, Copy, Eq)]
pub struct State {
    pub board: [[Option<Piece>; 8]; 8],
    pub color: bool, //black = true, white = false
    pub player: bool, //computer = true, user = false
}
impl State {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_board(&self, row: usize, col: usize) -> Option<Piece> {
        self.board[row][col]
    }
    pub fn get_color(&self, row: usize, col: usize) -> Option<bool> {
        let piece = self.get_board(row, col);
        match piece {
            Some(Piece::Black(false)) => Some(true),
            Some(Piece::Black(true)) => Some(true),
            Some(Piece::White(false)) => Some(false),
            Some(Piece::White(true)) => Some(false),
            None => None,
        }
    }
    pub fn is_crowned(&self, row: usize, col: usize) -> Option<bool> {
        let piece = self.get_board(row, col);
        match piece {
            Some(Piece::Black(false)) => Some(false),
            Some(Piece::Black(true)) => Some(true),
            Some(Piece::White(false)) => Some(false),
            Some(Piece::White(true)) => Some(true),
            None => None,
        }
    }
    pub fn set_board(&mut self, row: usize, col: usize, val: Option<Piece>) {
        self.board[row][col] = val;
    }
    pub fn toggle_crown(&mut self, row: usize, col: usize) {
        let piece = self.get_board(row, col);
        if piece == Some(Piece::Black(false)) {
            self.set_board(row, col, Some(Piece::Black(true)));
        }
        else if piece == Some(Piece::Black(true)) {
            self.set_board(row, col, Some(Piece::Black(false)));
        }
        else if piece == Some(Piece::White(false)) {
            self.set_board(row, col, Some(Piece::White(true)));
        }
        else if piece == Some(Piece::White(true)) {
            self.set_board(row, col, Some(Piece::White(false)));
        }
    }
}
impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = vec![];
        board.push(' ');
        for col in 0..8 {
            board.push(' ');
            board.push(match col {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                _ => '7'
            });
        }
        board.push('\n');
        for row in 0..8 {
            board.push(match row {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                _ => '7'
            });
            board.push(' ');
            for col in 0..8 {
                board.push(match self.get_board(row, col) {
                    Some(piece) => piece.into(),
                    None => '_'
                });
                board.push(' ');
            }
            board.push('\n');
        }
        write!(f, "{}", board.iter().collect::<String>())
    }
}
impl Default for State {
    fn default() -> Self {
        let mut brd: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
        for row in 0..3 {
            for col in 0..8 {
                if row % 2 != col % 2 {
                    brd[row][col] = Some(Piece::White(bool::from(false)));
                }
            }
        }
        for row in 5..8 {
            for col in 0..8 {
                if row % 2 != col % 2 {
                    brd[row][col] = Some(Piece::Black(bool::from(false)));
                }
            }
        }
        State { board: brd,
                color: true,
                player: true }
    }
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..8 {
            for col in 0..8 {
                if self.get_board(row, col) != other.get_board(row, col) {
                    return false;
                }
            }
        }
        true
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}