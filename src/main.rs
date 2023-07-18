use std::io;
use rand::Rng;
use crate::board::{Piece, State};
mod board;

fn terminal(state: &State) -> Option<bool> { //not terminal = None, black wins = Some(true), white wins = Some(false)
    let mut b: bool = false;
    let mut w: bool = false;
    for row in 0..8 {
        for col in 0..8 {
            if state.get_color(row, col) == Some(true) {
                b = true;
            }
            else if state.get_color(row, col) == Some(false) {
                w = true;
            }
        }
    }
    if !b {
        return Some(false);
    }
    else if !w {
        return Some(true);
    }
    return None;
}
fn dfs_b(state1: &State, row1: &usize, col1: &usize) -> Vec<State> {
    let mut vec: Vec<State> = Vec::new();
    let mut state: State = *state1;
    let mut row: usize = *row1;
    let mut col: usize = *col1;
    let copy = state.clone();
    vec.push(copy);
    if row < 1 {
        return vec;
    }
    if col > 1 && state.get_color(row - 1, col - 1) == Some(false) && state.get_board(row - 2, col - 2) == None {
        let piece: Option<Piece> = state.get_board(row - 1, col - 1);
        state.set_board(row, col, None);
        state.set_board(row - 1, col - 1, None);
        state.set_board(row - 2, col - 2, Some(Piece::Black(false)));
        let v: Vec<State> = dfs_b(&state, &(row - 2), &(col - 2));
        for val in v {
            vec.push(val);
        }
        state.set_board(row, col, Some(Piece::Black(false)));
        state.set_board(row - 1, col - 1, piece);
        state.set_board(row - 2, col - 2, None);
        vec.retain(|value| *value != state);
    }
    if col < 6 && state.get_color(row - 1, col + 1) == Some(false) && state.get_board(row - 2, col + 2) == None {
        let piece: Option<Piece> = state.get_board(row - 1, col + 1);
        state.set_board(row, col, None);
        state.set_board(row - 1, col + 1, None);
        state.set_board(row - 2, col + 2, Some(Piece::Black(false)));
        let v: Vec<State> = dfs_b(&state, &(row - 2), &(col + 2));
        for val in v {
            vec.push(val);
        }
        state.set_board(row, col, Some(Piece::Black(false)));
        state.set_board(row - 1, col + 1, piece);
        state.set_board(row - 2, col + 2, None);
        vec.retain(|value| *value != state);
    }
    return vec;
}
fn dfs_w(state1: &State, row1: &usize, col1: &usize) -> Vec<State> {
    let mut vec: Vec<State> = Vec::new();
    let mut state: State = *state1;
    let mut row: usize = *row1;
    let mut col: usize = *col1;
    let copy = state.clone();
    vec.push(copy);
    if row > 6 {
        return vec;
    }
    if col > 1 && state.get_color(row + 1, col - 1) == Some(true) && state.get_board(row + 2, col - 2) == None {
        let piece: Option<Piece> = state.get_board(row + 1, col - 1);
        state.set_board(row, col, None);
        state.set_board(row + 1, col - 1, None);
        state.set_board(row + 2, col - 2, Some(Piece::White(false)));
        let v: Vec<State> = dfs_w(&state, &(row + 2), &(col - 2));
        for val in v {
            vec.push(val);
        }
        state.set_board(row, col, Some(Piece::White(false)));
        state.set_board(row + 1, col - 1, piece);
        state.set_board(row + 2, col - 2, None);
        vec.retain(|value| *value != state);
    }
    if col < 6 && state.get_color(row + 1, col + 1) == Some(true) && state.get_board(row + 2, col + 2) == None {
        let piece: Option<Piece> = state.get_board(row + 1, col + 1);
        state.set_board(row, col, None);
        state.set_board(row + 1, col + 1, None);
        state.set_board(row + 2, col + 2, Some(Piece::White(false)));
        let v: Vec<State> = dfs_w(&state, &(row + 2), &(col + 2));
        for val in v {
            vec.push(val);
        }
        state.set_board(row, col, Some(Piece::White(false)));
        state.set_board(row + 1, col + 1, piece);
        state.set_board(row + 2, col + 2, None);
        vec.retain(|value| *value != state);
    }
    return vec;
}
fn dfs_crowned(state1: &State, row1: &usize, col1: &usize) -> Vec<State> {
    let mut vec: Vec<State> = Vec::new();
    let mut state: State = *state1;
    let mut row: usize = *row1;
    let mut col: usize = *col1;
    let mut curpiece: Option<Piece> = state.get_board(row, col);
    let copy = state.clone();
    vec.push(copy);
    if col > 1 && state.get_color(row - 1, col - 1) != state.get_color(row, col) && state.get_board(row - 2, col - 2) == None {
        let piece: Option<Piece> = state.get_board(row - 1, col - 1);
        state.set_board(row, col, None);
        state.set_board(row - 1, col - 1, None);
        state.set_board(row - 2, col - 2, curpiece);
        let v: Vec<State> = dfs_crowned(&state, &(row - 2), &(col - 2));
        for val in v {
            vec.push(val);
        }
        state.set_board(row, col, curpiece);
        state.set_board(row - 1, col - 1, piece);
        state.set_board(row - 2, col - 2, None);
        vec.retain(|value| *value != state);
    }
    if col < 6 && state.get_color(row - 1, col + 1) != state.get_color(row, col) && state.get_board(row - 2, col + 2) == None {
        let piece: Option<Piece> = state.get_board(row - 1, col + 1);
        state.set_board(row, col, None);
        state.set_board(row - 1, col + 1, None);
        state.set_board(row - 2, col + 2, curpiece);
        let v: Vec<State> = dfs_crowned(&state, &(row - 2), &(col + 2));
        for val in v {
            vec.push(val);
        }
        state.set_board(row, col, curpiece);
        state.set_board(row - 1, col + 1, piece);
        state.set_board(row - 2, col + 2, None);
        vec.retain(|value| *value != state);
    }
    if col > 1 && state.get_color(row + 1, col - 1) != state.get_color(row, col) && state.get_board(row + 2, col - 2) == None {
        let piece: Option<Piece> = state.get_board(row + 1, col - 1);
        state.set_board(row, col, None);
        state.set_board(row + 1, col - 1, None);
        state.set_board(row + 2, col - 2, curpiece);
        let v: Vec<State> = dfs_crowned(&state, &(row + 2), &(col - 2));
        for val in v {
            vec.push(val);
        }
        state.set_board(row, col, curpiece);
        state.set_board(row + 1, col - 1, piece);
        state.set_board(row + 2, col - 2, None);
        vec.retain(|value| *value != state);
    }
    if col < 6 && state.get_color(row + 1, col + 1) != state.get_color(row, col) && state.get_board(row + 2, col + 2) == None {
        let piece: Option<Piece> = state.get_board(row + 1, col + 1);
        state.set_board(row, col, None);
        state.set_board(row + 1, col + 1, None);
        state.set_board(row + 2, col + 2, curpiece);
        let v: Vec<State> = dfs_crowned(&state, &(row + 2), &(col + 2));
        for val in v {
            vec.push(val);
        }
        state.set_board(row, col, curpiece);
        state.set_board(row + 1, col + 1, piece);
        state.set_board(row + 2, col + 2, None);
        vec.retain(|value| *value != state);
    }
    return vec;
}
fn children(state1: &State, row1: &usize, col1: &usize) -> Vec<State> {
    let mut vec: Vec<State> = Vec::new();
    let mut state: State = *state1;
    let mut row: usize = *row1;
    let mut col: usize = *col1;
    if state.color && state.get_board(row, col) == Some(Piece::Black(false)) {
        if row > 1 {
            let v: Vec<State> = dfs_b(&state, &row, &col);
            for val in v {
                vec.push(val);
            }
        }
    }
    else if state.color && state.get_board(row, col) == Some(Piece::Black(true)) {
        let v: Vec<State> = dfs_crowned(&state, &row, &col);
        for val in v {
            vec.push(val);
        }
    }
    else if !state.color && state.get_board(row, col) == Some(Piece::White(false)) {
        if row < 6 {
            let v: Vec<State> = dfs_w(&state, &row, &col);
            for val in v {
                vec.push(val);
            }
        }
    }
    else if !state.color && state.get_board(row, col) == Some(Piece::White(true)) {
        let v: Vec<State> = dfs_crowned(&state, &row, &col);
        for val in v {
            vec.push(val);
        }
    }
    if vec.is_empty() {
        if state.color && state.get_board(row, col) == Some(Piece::Black(false)) {
            if row > 0 {
                if col > 0 && state.get_board(row - 1, col - 1) == None {
                    state.set_board(row, col, None);
                    state.set_board(row - 1, col - 1, Some(Piece::Black(false)));
                    vec.push(state);
                    state.set_board(row, col, Some(Piece::Black(false)));
                    state.set_board(row - 1, col - 1, None);
                }
                if col < 7 && state.get_board(row - 1, col + 1) == None {
                    state.set_board(row, col, None);
                    state.set_board(row - 1, col + 1, Some(Piece::Black(false)));
                    vec.push(state);
                    state.set_board(row, col, Some(Piece::Black(false)));
                    state.set_board(row - 1, col + 1, None);
                }
            }
        }
        else if state.color && state.get_board(row, col) == Some(Piece::Black(true)) {
            if row > 0 {
                if col > 0 && state.get_board(row - 1, col - 1) == None {
                    state.set_board(row, col, None);
                    state.set_board(row - 1, col - 1, Some(Piece::Black(true)));
                    vec.push(state);
                    state.set_board(row, col, Some(Piece::Black(true)));
                    state.set_board(row - 1, col - 1, None);
                }
                if col < 7 && state.get_board(row - 1, col + 1) == None {
                    state.set_board(row, col, None);
                    state.set_board(row - 1, col + 1, Some(Piece::Black(true)));
                    vec.push(state);
                    state.set_board(row, col, Some(Piece::Black(true)));
                    state.set_board(row - 1, col + 1, None);
                }
            }
            if row < 7 {
                if col > 0 && state.get_board(row + 1, col - 1) == None {
                    state.set_board(row, col, None);
                    state.set_board(row + 1, col - 1, Some(Piece::Black(true)));
                    vec.push(state);
                    state.set_board(row, col, Some(Piece::Black(true)));
                    state.set_board(row + 1, col - 1, None);
                }
                if col < 7 && state.get_board(row + 1, col + 1) == None {
                    state.set_board(row, col, None);
                    state.set_board(row + 1, col + 1, Some(Piece::Black(true)));
                    vec.push(state);
                    state.set_board(row, col, Some(Piece::Black(true)));
                    state.set_board(row + 1, col + 1, None);
                }  
            }
        }
        else if !state.color && state.get_board(row, col) == Some(Piece::White(false)) {
            if row < 7 {
                if col > 0 && state.get_board(row + 1, col - 1) == None {
                    state.set_board(row, col, None);
                    state.set_board(row + 1, col - 1, Some(Piece::White(false)));
                    vec.push(state);
                    state.set_board(row, col, Some(Piece::White(false)));
                    state.set_board(row + 1, col - 1, None);
                }
                if col < 7 && state.get_board(row + 1, col + 1) == None {
                    state.set_board(row, col, None);
                    state.set_board(row + 1, col + 1, Some(Piece::White(false)));
                    vec.push(state);
                    state.set_board(row, col, Some(Piece::White(false)));
                    state.set_board(row + 1, col + 1, None);
                }
            }
        }
        else if !state.color && state.get_board(row, col) == Some(Piece::White(true)) {
            if row > 0 {
                if col > 0 && state.get_board(row - 1, col - 1) == None {
                    state.set_board(row, col, None);
                    state.set_board(row - 1, col - 1, Some(Piece::White(true)));
                    vec.push(state);
                    state.set_board(row, col, Some(Piece::White(true)));
                    state.set_board(row - 1, col - 1, None);
                }
                if col < 7 && state.get_board(row - 1, col + 1) == None {
                    state.set_board(row, col, None);
                    state.set_board(row - 1, col + 1, Some(Piece::White(true)));
                    vec.push(state);
                    state.set_board(row, col, Some(Piece::White(true)));
                    state.set_board(row - 1, col + 1, None);
                }
            }
            if row < 7 {
                if col > 0 && state.get_board(row + 1, col - 1) == None {
                    state.set_board(row, col, None);
                    state.set_board(row + 1, col - 1, Some(Piece::White(true)));
                    vec.push(state);
                    state.set_board(row, col, Some(Piece::White(true)));
                    state.set_board(row + 1, col - 1, None);
                }
                if col < 7 && state.get_board(row + 1, col + 1) == None {
                    state.set_board(row, col, None);
                    state.set_board(row + 1, col + 1, Some(Piece::White(true)));
                    vec.push(state);
                    state.set_board(row, col, Some(Piece::White(true)));
                    state.set_board(row + 1, col + 1, None);
                }  
            }
        }
    }
    return vec;
}
fn eval(state: &State) -> f64 {
    let mut score: u32 = 0;
    for row in 0..8 {
        for col in 0..8 {
            if state.get_color(row, col) == None {
                continue;
            }
            if state.color {
                if state.get_color(row, col) == Some(true) {
                    if state.is_crowned(row, col) == Some(true) {
                        score += 2 * (8 - (row as u32));
                    }
                    else {
                        score += 8 - (row as u32);
                    }
                }
                else {
                    if state.is_crowned(row, col) == Some(true) {
                        score -= 2 * (row as u32);
                    }
                    else {
                        score -= row as u32;
                    }                
                }
            }
            else if !state.color {
                if state.get_color(row, col) == Some(false) {
                    if state.is_crowned(row, col) == Some(true) {
                        score += 2 * (row as u32);
                    }
                    else {
                        score += row as u32;
                    }
                }
                else {
                    if state.is_crowned(row, col) == Some(true) {
                        score += 2 * (8 - (row as u32));
                    }
                    else {
                        score += 8 - (row as u32);
                    }      
                }
            }
        }
    }
    return 0.0;
}
fn minimax(state1: &State, min: f64, max: f64) -> State { //max player = black, min player = white
    let mut state: State = *state1;
    return state;
}
fn main() {
    loop {
        println!("Hello! Please enter 0 for a link to the rules, or enter any other number to start playing!");
        let mut state = State::new();
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Failed to read input.");
        let inp: u8 = inp.trim().parse().expect("Not a number!");
        if inp == 0 {
            println!("You can find the rules at https://www.wikihow.com/Play-Checkers.");
        }
        println!("{state}");
        println!("_ : empty space");
        println!("B : uncrowned black checker");
        println!("C: crowned black checker");
        println!("W : uncrowned white checker");
        println!("X : crowned white checker");
        let rnum: u8 = rand::thread_rng().gen_range(1..=2);
        if rnum == 1 {
            println!("You are playing black.");
            state.player = false;
        }
        else {
            println!("You are playing white.");
        }
        loop {
            if state.player {
                //computer's turn
            }
            else {
                let mut row = String::new();
                let mut col = String::new();
                loop {
                    println!("Enter the row number of the piece you would like to move: (0-7)");
                    let mut row = String::new();
                    io::stdin().read_line(&mut row).expect("Failed to read input.");
                    let row: u8 = row.trim().parse().expect("Not a number!");
                    println!("Enter the column number of the piece you would like to move: (0-7)");
                    let mut col = String::new();
                    io::stdin().read_line(&mut col).expect("Failed to read input.");
                    let col: u8 = col.trim().parse().expect("Not a number!");
                    if row < 0 || row > 7 || col < 0 || col > 7 {
                        println!("Position out of bounds! Please enter numbers from 0-7.");
                        continue;
                    }
                    if state.get_color(row as usize, col as usize) != Some(state.color) {
                        println!("The selected space does not contain one of your pieces.");
                        continue;
                    }
                    let vec = children(&state, &(row as usize), &(col as usize));
                    if vec.is_empty() {
                        println!("The selected space does not have any legal moves.");
                        continue;
                    }
                    println!("Enter the direction you would like to move this piece: (0 = up and left, 1 = up and right, 2 = down and left, 3 = down and right)");
                    let mut dir = String::new();
                    io::stdin().read_line(&mut dir).expect("Failed to read input.");
                    let dir: u8 = dir.trim().parse().expect("Not a number!");
                    if dir < 0 || dir > 3 {
                        println!("Please enter a number from 0-3.");
                        continue;
                    }
                    if (state.get_board(row as usize, col as usize) == Some(Piece::Black(false)) && dir > 1) || (state.get_board(row as usize, col as usize) == Some(Piece::White(false)) && dir < 2) {
                        println!("This piece can't legally move in this direction.");
                    }
                    break;
                }
            }
            break;
        }
        println!("Enter 0 to play again, or enter any other number to quit.");
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Failed to read input.");
        let inp: u8 = inp.trim().parse().expect("Not a number!");
        if inp != 0 {
            println!("You have quit the game.");
            break;
        }
        println!();
    }
}