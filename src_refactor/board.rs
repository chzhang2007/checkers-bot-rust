use std::fmt::Display;

#[derive(Debug)]
enum BoardError {
    InvalidMove,
    InvalidPiece,
    InvalidPosition,
}

impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardError::InvalidMove => write!(f, "Invalid move"),
            BoardError::InvalidPiece => write!(f, "Invalid piece"),
            BoardError::InvalidPosition => write!(f, "Invalid position"),
        }
    }
}

impl std::error::Error for BoardError {}


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Piece {
    Black(bool), // false == not crowned
    White(bool),
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Board {
    board: [[Option<Piece>; 8]; 8]
}

// Some useful functionality for a board
// More can be added as needed
impl Board {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Piece> {
        self.board[x][y]
    }

    pub fn set(&mut self, x: usize, y: usize, piece: Option<Piece>) {
        self.board[x][y] = piece;
    }

    pub fn remove(&mut self, x: usize, y: usize) -> Option<Piece> {
        let piece = self.board[x][y];
        self.board[x][y] = None;
        piece
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.board[x][y].is_none()
    }

    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        x < 8 && y < 8
    }

    pub fn is_crowned(&self, x: usize, y: usize) -> Result<bool, BoardError> {
        match self.board[x][y] {
            Some(Piece::Black(crowned)) | Some(Piece::White(crowned)) => Ok(crowned),
            _ => Err(BoardError::InvalidPiece),
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        /* Write how you would like the board to be displayed here */
        write!(f, "Board"); // Like this, DO NOT USE PRINT STATEMENTS
        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = [[None; 8]; 8];
        /* Fill the board with pieces */
        Board { board }
    }
}