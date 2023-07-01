use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Black(bool), //crowned = true, uncrowned = false
    White(bool),
}
impl Display for Option<Piece> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self);
        Ok(())
    }
}
impl From<Option<Piece>> for char {
    fn from(piece: Option<Piece>) -> char {
        match piece {
            Some(Piece::Black(false)) => 'B',
            Some(Piece::Black(true)) => 'C',
            Some(Piece::White(false)) => 'W',
            Some(Piece::White(true)) => 'X',
            None => '_'
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
    pub fn set_board(&self, row: usize, col: usize, val: Option<Piece>) {
        self.board[row][col] = val;
    }
}
impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..8 {
            for col in 0..8 {
                write!(f, "{}", self.get_board(row, col));
            }
            writeln!(f, "{}", "");
        }
        Ok(())
    }
}
impl Default for State {
    fn default() -> Self {
        let mut brd: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
        State { board: brd,
                color: true,
                player: true }
    }
}