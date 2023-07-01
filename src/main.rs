use std::io;
use rand::Rng;
use crate::board::{Piece, State};
mod board;

fn terminal(state: State) -> (Option<bool>, State) { //not terminal = None, black wins = true, white wins = false
    let mut b: bool = false;
    let mut w: bool = false;
    for row in 0..8 {
        for col in 0..8 {
            if state.get_board(row, col) == Some(Piece::Black(false)) || state.get_board(row, col) == Some(Piece::Black(true)) {
                b = true;
            }
            else if state.get_board(row, col) == Some(Piece::White(false)) || state.get_board(row, col) == Some(Piece::White(true)) {
                w = true;
            }
        }
    }
    if !b {
        return (Some(false), state);
    }
    else if !w {
        return (Some(true), state);
    }
    return (None, state);
}
fn dfs_b(state: State, row: usize, col: usize) -> (Vec<State>, State, usize, usize) {
    let mut vec: Vec<State> = Vec::new();
    let mut state: State = state;
    let mut row: usize = row;
    let mut col: usize = col;
    let copy = state.clone();
    vec.push(copy);
    if row < 1 {
        return (vec, state, row, col);
    }
    if col > 1 && (state.get_board(row - 1, col - 1) == Some(Piece::White(bool::from(false))) || state.get_board(row - 1, col - 1) == Some(Piece::White(bool::from(true)))) && state.get_board(row - 2, col - 2) == None {
        let piece: Option<Piece> = state.get_board(row - 1, col - 1);
        state.set_board(row, col, None);
        state.set_board(row - 1, col - 1, None);
        state.set_board(row - 2, col - 2, Some(Piece::Black(bool::from(false))));
        let tup: (Vec<State>, State, usize, usize) = dfs_b(state, row - 2, col - 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 + 2;
        col = tup.3 + 2;
        state.set_board(row, col, Some(Piece::Black(bool::from(false))));
        state.set_board(row - 1, col - 1, piece);
        state.set_board(row - 2, col - 2, None);
    }
    if col < 6 && (state.get_board(row - 1, col + 1) == Some(Piece::White(bool::from(false))) || state.get_board(row - 1, col + 1) == Some(Piece::White(bool::from(true)))) && state.get_board(row - 2, col + 2) == None {
        let piece: Option<Piece> = state.get_board(row - 1, col + 1);
        state.set_board(row, col, None);
        state.set_board(row - 1, col + 1, None);
        state.set_board(row - 2, col + 2, Some(Piece::Black(bool::from(false))));
        let tup: (Vec<State>, State, usize, usize) = dfs_b(state, row - 2, col + 2);
        for val in tup.0 {
            vec.push(val);
        }
        state = tup.1;
        row = tup.2 + 2;
        col = tup.3 + 2;
        state.set_board(row, col, Some(Piece::Black(bool::from(false))));
        state.set_board(row - 1, col + 1, piece);
        state.set_board(row - 2, col + 2, None);
    }
    //todo!("Return only final moves");
    return (vec, state, row, col);
}
fn dfs_c(state: State, row: usize, col: usize) -> (Vec<State>, State, usize, usize) {
    let mut vec: Vec<State> = Vec::new();
    let mut state: State = state;
    let mut row: usize = row;
    let mut col: usize = col;
    let copy = state.clone();
    vec.push(copy);
    if col > 1 && (state.get_board(row - 1, col - 1) == Some(Piece::White(bool::from(false))) || state.get_board(row - 1, col - 1) == Some(Piece::White(bool::from(true)))) && state.get_board(row - 2, col - 2) == None {
        let piece: Option<Piece> = state.get_board(row - 1, col - 1);
        state.set_board(row, col, None);
        state.set_board(row - 1, col - 1, None);
        state.set_board(row - 2, col - 2, Some(Piece::Black(bool::from(true))));
        let tup: (Vec<State>, State, usize, usize) = dfs_c(state, row - 2, col - 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 + 2;
        col = tup.3 + 2;
        state.set_board(row, col, Some(Piece::Black(bool::from(true))));
        state.set_board(row - 1, col - 1, piece);
        state.set_board(row - 2, col - 2, None);
    }
    if col < 6 && (state.get_board(row - 1, col + 1) == Some(Piece::White(bool::from(false))) || state.get_board(row - 1, col + 1) == Some(Piece::White(bool::from(true)))) && state.get_board(row - 2, col + 2) == None {
        let piece: Option<Piece> = state.get_board(row - 1, col + 1);
        state.set_board(row, col, None);
        state.set_board(row - 1, col + 1, None);
        state.set_board(row - 2, col + 2, Some(Piece::Black(bool::from(true))));
        let tup: (Vec<State>, State, usize, usize) = dfs_c(state, row - 2, col + 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 + 2;
        col = tup.3 - 2;
        state.set_board(row, col, Some(Piece::Black(bool::from(true))));
        state.set_board(row - 1, col + 1, piece);
        state.set_board(row - 2, col + 2, None);
    }
    if col > 1 && (state.get_board(row + 1, col - 1) == Some(Piece::White(bool::from(false))) || state.get_board(row + 1, col - 1) == Some(Piece::White(bool::from(true)))) && state.get_board(row + 2, col - 2) == None {
        let piece: Option<Piece> = state.get_board(row + 1, col - 1);
        state.set_board(row, col, None);
        state.set_board(row + 1, col - 1, None);
        state.set_board(row + 2, col - 2, Some(Piece::Black(bool::from(true))));
        let tup: (Vec<State>, State, usize, usize) = dfs_c(state, row + 2, col - 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 - 2;
        col = tup.3 + 2;
        state.set_board(row, col, Some(Piece::Black(bool::from(true))));
        state.set_board(row + 1, col - 1, piece);
        state.set_board(row + 2, col - 2, None);
    }
    if col < 6 && (state.get_board(row + 1, col + 1) == Some(Piece::White(bool::from(false))) || state.get_board(row + 1, col + 1) == Some(Piece::White(bool::from(true)))) && state.get_board(row + 2, col + 2) == None {
        let piece: Option<Piece> = state.get_board(row + 1, col + 1);
        state.set_board(row, col, None);
        state.set_board(row + 1, col + 1, None);
        state.set_board(row + 2, col + 2, Some(Piece::Black(bool::from(true))));
        let tup: (Vec<State>, State, usize, usize) = dfs_c(state, row + 2, col + 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 - 2;
        col = tup.3 - 2;
        state.set_board(row, col, Some(Piece::Black(bool::from(true))));
        state.set_board(row + 1, col + 1, piece);
        state.set_board(row + 2, col + 2, None);
    }
    //todo!("Return only final moves");
    return (vec, state, row, col);
}
fn dfs_w(state: State, row: usize, col: usize) -> (Vec<State>, State, usize, usize) {
    let mut vec: Vec<State> = Vec::new();
    let mut state: State = state;
    let mut row: usize = row;
    let mut col: usize = col;
    let copy = state.clone();
    vec.push(copy);
    if row > 6 {
        return (vec, state, row, col);
    }
    if col > 1 && (state.get_board(row + 1, col - 1) == Some(Piece::Black(bool::from(false))) || state.get_board(row + 1, col - 1) == Some(Piece::Black(bool::from(true)))) && state.get_board(row + 2, col - 2) == None {
        let piece: Option<Piece> = state.get_board(row + 1, col - 1);
        state.set_board(row, col, None);
        state.set_board(row + 1, col - 1, None);
        state.set_board(row + 2, col - 2, Some(Piece::White(bool::from(false))));
        let tup: (Vec<State>, State, usize, usize) = dfs_w(state, row + 2, col - 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 - 2;
        col = tup.3 + 2;
        state.set_board(row, col, Some(Piece::White(bool::from(false))));
        state.set_board(row + 1, col - 1, piece);
        state.set_board(row + 2, col - 2, None);
    }
    if col < 6 && (state.get_board(row + 1, col + 1) == Some(Piece::Black(bool::from(false))) || state.get_board(row + 1, col + 1) == Some(Piece::Black(bool::from(true)))) && state.get_board(row + 2, col + 2) == None {
        let piece: Option<Piece> = state.get_board(row + 1, col + 1);
        state.set_board(row, col, None);
        state.set_board(row + 1, col + 1, None);
        state.set_board(row + 2, col + 2, Some(Piece::White(bool::from(false))));
        let tup: (Vec<State>, State, usize, usize) = dfs_w(state, row + 2, col + 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 - 2;
        col = tup.3 - 2;
        state.set_board(row, col, Some(Piece::White(bool::from(false))));
        state.set_board(row + 1, col + 1, piece);
        state.set_board(row + 2, col + 2, None);
    }
    //todo!("Return only final moves");
    return (vec, state, row, col);
}
fn dfs_x(state: State, row: usize, col: usize) -> (Vec<State>, State, usize, usize) {
    let mut vec: Vec<State> = Vec::new();
    let mut state: State = state;
    let mut row: usize = row;
    let mut col: usize = col;
    let copy = state.clone();
    vec.push(copy);
    if col > 1 && (state.get_board(row - 1, col - 1) == Some(Piece::Black(bool::from(false))) || state.get_board(row - 1, col - 1) == Some(Piece::Black(bool::from(true)))) && state.get_board(row - 2, col - 2) == None {
        let piece: Option<Piece> = state.get_board(row - 1, col - 1);
        state.set_board(row, col, None);
        state.set_board(row - 1, col - 1, None);
        state.set_board(row - 2, col - 2, Some(Piece::White(bool::from(true))));
        let tup: (Vec<State>, State, usize, usize) = dfs_x(state, row - 2, col - 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 + 2;
        col = tup.3 + 2;
        state.set_board(row, col, Some(Piece::White(true)));
        state.set_board(row - 1, col - 1, piece);
        state.set_board(row - 2, col - 2, None);
    }
    if col < 6 && (state.get_board(row - 1, col + 1) == Some(Piece::Black(bool::from(false))) || state.get_board(row - 1, col + 1) == Some(Piece::Black(bool::from(true)))) && state.get_board(row - 2, col + 2) == None {
        let piece: Option<Piece> = state.get_board(row - 1, col + 1);
        state.set_board(row, col, None);
        state.set_board(row - 1, col + 1, None);
        state.set_board(row - 2, col + 2, Some(Piece::White(bool::from(true))));
        let tup: (Vec<State>, State, usize, usize) = dfs_x(state, row - 2, col + 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 + 2;
        col = tup.3 - 2;
        state.set_board(row, col, Some(Piece::White(bool::from(true))));
        state.set_board(row - 1, col + 1, piece);
        state.set_board(row - 2, col + 2, None);
    }
    if col > 1 && (state.get_board(row + 1, col - 1) == Some(Piece::Black(bool::from(false))) || state.get_board(row + 1, col - 1) == Some(Piece::Black(bool::from(true)))) && state.get_board(row + 2, col - 2) == None {
        let piece: Option<Piece> = state.get_board(row + 1, col - 1);
        state.set_board(row, col, None);
        state.set_board(row + 1, col - 1, None);
        state.set_board(row + 2, col - 2, Some(Piece::White(bool::from(true))));
        let tup: (Vec<State>, State, usize, usize) = dfs_x(state, row + 2, col - 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 - 2;
        col = tup.3 + 2;
        state.set_board(row, col, Some(Piece::White(bool::from(true))));
        state.set_board(row + 1, col - 1, piece);
        state.set_board(row + 2, col - 2, None);
    }
    if col < 6 && (state.get_board(row + 1, col + 1) == Some(Piece::Black(bool::from(false))) || state.get_board(row + 1, col + 1) == Some(Piece::Black(bool::from(true)))) && state.get_board(row + 2, col + 2) == None {
        let piece: Option<Piece> = state.get_board(row + 1, col + 1);
        state.set_board(row, col, None);
        state.set_board(row + 1, col + 1, None);
        state.set_board(row + 2, col + 2, Some(Piece::White(bool::from(true))));
        let tup: (Vec<State>, State, usize, usize) = dfs_x(state, row + 2, col + 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 - 2;
        col = tup.3 - 2;
        state.set_board(row, col, Some(Piece::White(bool::from(true))));
        state.set_board(row + 1, col + 1, piece);
        state.set_board(row + 2, col + 2, None);
    }
    //todo!("Return only final moves");
    return (vec, state, row, col);
}
fn children(state: State, row: usize, col: usize) -> (Vec<State>, State) {
    let mut vec: Vec<State> = Vec::new();
    let mut state: State = state;
    let mut row: usize = row;
    let mut col: usize = col;
    if state.color {
        for row in 0..8 {
            for col in 0..8 {
                if state.get_board(row, col) == Some(Piece::Black(false)) {
                    if row > 1 {
                        let tup: (Vec<State>, State, usize, usize) = dfs_b(state, row, col);
                        for val in tup.0 {
                            vec.push(val);
                        }
                        state = tup.1;
                    }
                }
                else if state.get_board(row, col) == Some(Piece::Black(true)) {
                    let tup: (Vec<State>, State, usize, usize) = dfs_c(state, row, col);
                    for val in tup.0 {
                        vec.push(val);
                    }
                    state = tup.1;
                }
            }
        }
    }
    else {

    }
    //todo!();
    return (vec, state);
}
fn eval(state: State) -> f64 {
    //todo!();
    return 0.0;
}
fn minimax(state: State, min: f64, max: f64) -> State {
    let s = State {
        board: [[None as Option<Piece>; 8] ; 8],
        color: true,
        player: true,
    };
    //todo!();
    return s;
}
fn main() {
    loop {
        println!("Hello! Please enter 0 for a link to the rules, or enter anything else to start playing!.");
        let mut state = State {
            board: [[None as Option<Piece>; 8] ; 8],
            color: true,
            player: true,
        };
        loop {
            let mut inp = String::new();
            io::stdin().read_line(&mut inp).expect("Failed to read input.");
            let inp: u8 = inp.trim().parse().expect("Not a number!");
            if inp == 0 {
                println!("You can find the rules at https://www.wikihow.com/Play-Checkers.");
            }
            else {
                let rnum: u8 = rand::thread_rng().gen_range(1..=2);
                if rnum == 1 {
                    println!("You are playing black.");
                    state.player = false;
                    break;
                }
                println!("You are playing white.");
                break;
            }
        }
        for row in 0..3 {
            for col in 0..8 {
                if col % 2 == 1 {
                    state.set_board(row, col, Some(Piece::White(bool::from(false))));
                }
            }
        }
        for row in 5..8 {
            for col in 0..8 {
                if row % 2 == 0 {
                    state.set_board(row, col, Some(Piece::Black(bool::from(false))));
                }
            }
        }
        state.Display();
        println!("_ : empty space");
        println!("B : uncrowned black checker");
        println!("C: crowned black checker");
        println!("W : uncrowned white checker");
        println!("X : crowned white checker");
        loop {
            let tup: (Option<bool>, State) = terminal(state);
            let end: Option<bool> = tup.0;
            state = tup.1;
            if end == Some(true) {
                println!("Black won!");
                break;
            }
            else if end == Some(false) {
                println!("White won!");
                break;
            }
            break;
        }
        println!("Enter 0 to play again or 1 to quit.");
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Failed to read input.");
        let inp: u8 = inp.trim().parse().expect("Not a number!");
        if inp == 1 {
            println!("You have quit the game.");
            break;
        }
        println!();
    }
}