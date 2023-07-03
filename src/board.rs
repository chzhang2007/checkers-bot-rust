use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Black(bool), //crowned = true, uncrowned = false
    White(bool),
}
impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self);
        Ok(())
    }
}
impl From<Piece> for char {
    fn from(piece: Piece) -> char {
        match piece {
            Piece::Black(false) => 'B',
            Piece::Black(true) => 'C',
            Piece::White(false) => 'W',
            Piece::White(true) => 'X',
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
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
    pub fn set_board(&mut self, row: usize, col: usize, val: Option<Piece>) {
        self.board[row][col] = val;
    }
}
impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = vec![];
        for row in 0..8 {
            for col in 0..8 {
                board.push(match self.get_board(row, col) {
                    Some(piece) => piece.into(),
                    None => '_'
                });
            }
            board.push('\n');
        }
        write!(f, "{}", board.iter().collect::<String>());
        Ok(())
    }
}
impl Default for State {
    fn default() -> Self {
        let mut brd: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
        for row in 0..3 {
            for col in 0..8 {
                if col % 2 == 1 {
                    brd[row][col] = Some(Piece::White(bool::from(false)));
                }
            }
        }
        for row in 5..8 {
            for col in 0..8 {
                if col % 2 == 0 {
                    brd[row][col] = Some(Piece::Black(bool::from(false)));
                }
            }
        }
        State { board: brd,
                color: true,
                player: true }
    }
}