use crate::board::{Board, Piece};
use rand::{self, Rng};

// Hash is here just incase you want to use lookup tables down the line
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct State {
    pub board: Board,
    pub color: bool,
    pub player: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            board: Board::new(), // Default board
            color: rand::thread_rng().gen_bool(0.5), // Randomly choose color
            player: false, // Player starts
        }
    }

    pub fn eval(&self) -> i32 {
        /* Some sort of heuristic evaluation function */
        todo!()
    }

    pub fn is_terminal(&self) -> Option<bool> {
        let mut b: bool = false;
        let mut w: bool = false;
        
        for x in 0..8 {
            for y in 0..8 {
                match self.board.get(x, y) {
                    Some(Piece::White(_)) => w = true,
                    Some(Piece::Black(_)) => b = true,
                    None => {},
                }
            }
        }
        if !b { Some(false) } 
        else if !w { Some(true) } 
        else { None }
    }

    pub fn children(&self, x: usize, y: usize) -> Vec<State> {
        /* Generate children */
        todo!()
    }
}