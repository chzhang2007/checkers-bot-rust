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
    if row < 2 {
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
    if row > 5 {
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
    if state.color && state.get_board(row, col) == Some(Piece::Black(false)) {
        if row > 0 {
            if col > 0 && state.get_board(row - 1, col - 1) == None {
                state.set_board(row, col, None);
                state.set_board(row - 1, col - 1, Some(Piece::Black(false)));
                vec.push(state.clone());
                state.set_board(row, col, Some(Piece::Black(false)));
                state.set_board(row - 1, col - 1, None);
            }
            if col < 7 && state.get_board(row - 1, col + 1) == None {
                state.set_board(row, col, None);
                state.set_board(row - 1, col + 1, Some(Piece::Black(false)));
                vec.push(state.clone());
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
                vec.push(state.clone());
                state.set_board(row, col, Some(Piece::Black(true)));
                state.set_board(row - 1, col - 1, None);
            }
            if col < 7 && state.get_board(row - 1, col + 1) == None {
                state.set_board(row, col, None);
                state.set_board(row - 1, col + 1, Some(Piece::Black(true)));
                vec.push(state.clone());
                state.set_board(row, col, Some(Piece::Black(true)));
                state.set_board(row - 1, col + 1, None);
            }
        }
        if row < 7 {
            if col > 0 && state.get_board(row + 1, col - 1) == None {
                state.set_board(row, col, None);
                state.set_board(row + 1, col - 1, Some(Piece::Black(true)));
                vec.push(state.clone());
                state.set_board(row, col, Some(Piece::Black(true)));
                state.set_board(row + 1, col - 1, None);
            }
            if col < 7 && state.get_board(row + 1, col + 1) == None {
                state.set_board(row, col, None);
                state.set_board(row + 1, col + 1, Some(Piece::Black(true)));
                vec.push(state.clone());
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
                vec.push(state.clone());
                state.set_board(row, col, Some(Piece::White(false)));
                state.set_board(row + 1, col - 1, None);
            }
            if col < 7 && state.get_board(row + 1, col + 1) == None {
                state.set_board(row, col, None);
                state.set_board(row + 1, col + 1, Some(Piece::White(false)));
                vec.push(state.clone());
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
                vec.push(state.clone());
                state.set_board(row, col, Some(Piece::White(true)));
                state.set_board(row - 1, col - 1, None);
            }
            if col < 7 && state.get_board(row - 1, col + 1) == None {
                state.set_board(row, col, None);
                state.set_board(row - 1, col + 1, Some(Piece::White(true)));
                vec.push(state.clone());
                state.set_board(row, col, Some(Piece::White(true)));
                state.set_board(row - 1, col + 1, None);
            }
        }
        if row < 7 {
            if col > 0 && state.get_board(row + 1, col - 1) == None {
                state.set_board(row, col, None);
                state.set_board(row + 1, col - 1, Some(Piece::White(true)));
                vec.push(state.clone());
                state.set_board(row, col, Some(Piece::White(true)));
                state.set_board(row + 1, col - 1, None);
            }
            if col < 7 && state.get_board(row + 1, col + 1) == None {
                state.set_board(row, col, None);
                state.set_board(row + 1, col + 1, Some(Piece::White(true)));
                vec.push(state.clone());
                state.set_board(row, col, Some(Piece::White(true)));
                state.set_board(row + 1, col + 1, None);
            }  
        }
    }
    return vec;
}
fn eval(state: &State) -> i64 {
    let mut score: i64 = 0;
    for row in 0..8 {
        for col in 0..8 {
            if state.get_color(row, col) == None {
                continue;
            }
            if state.color {
                if state.get_color(row, col) == Some(true) {
                    if state.is_crowned(row, col) == Some(true) {
                        score += 2 * (8 - (row as i64));
                    }
                    else {
                        score += 8 - (row as i64);
                    }
                }
                else {
                    if state.is_crowned(row, col) == Some(true) {
                        score -= 2 * (row as i64);
                    }
                    else {
                        score -= row as i64;
                    }                
                }
            }
            else if !state.color {
                if state.get_color(row, col) == Some(false) {
                    if state.is_crowned(row, col) == Some(true) {
                        score += 2 * (row as i64);
                    }
                    else {
                        score += row as i64;
                    }
                }
                else {
                    if state.is_crowned(row, col) == Some(true) {
                        score += 2 * (8 - (row as i64));
                    }
                    else {
                        score += 8 - (row as i64);
                    }      
                }
            }
        }
    }
    score
}
fn minimax(state1: &State, depth: &u8) -> (State, i64) { //max player = black, min player = white
    let state: State = *state1;
    let mut new_state = state;
    let mut val: i64 = 0;
    if *depth == 3 || terminal(&state) != None {
        return (state, eval(&state));
    }
    if state.color == true { //max player
        val = -1000000000;
        for row in 0..8 {
            for col in 0..8 {
                let vec = children(&state, &row, &col);
                for mut i in vec {
                    i.color = !i.color;
                    i.player = !i.player;
                    let tup = minimax(&i, &(depth + 1));
                    if tup.1 > val {
                        new_state = i;
                        val = tup.1;
                    }
                }
            }
        }
        return (new_state, val);
    }
    else { //min player
        val = 1000000000;
        for row in 0..8 {
            for col in 0..8 {
                let vec = children(&state, &row, &col);
                for mut i in vec {
                    i.color = !i.color;
                    i.player = !i.player;
                    let tup = minimax(&i, &(depth + 1));
                    if tup.1 < val {
                        new_state = i;
                        val = tup.1;
                    }
                }
            }
        }
        return (new_state, val);
    }
}
fn main() {
    let mut test = State::new();
    test.set_board(0, 0, Some(Piece::Black(true)));
    println!("{test}");
    let vec = children(&test, &0, &0);
    println!("{}", vec.len());
    /*loop {
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
            let end = terminal(&state);
            if end == Some(true) {
                println!("Black won!");
                break;
            }
            else if end == Some(false) {
                println!("White won!");
                break;
            }
            if state.player {
                let tup = minimax(&state, &0);
                state = tup.0;
                println!("The computer has made its move.");
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
                        println!("The selected space does not contain one of your pieces. Please select a different space.");
                        continue;
                    }
                    let vec = children(&state, &(row as usize), &(col as usize));
                    if vec.is_empty() {
                        println!("The selected space does not have any legal moves. Please select a different space.");
                        continue;
                    }
                    let dr: [i8; 4] = [-1, -1, 1, 1];
                    let dc: [i8; 4] = [-1, 1, -1, 1];
                    let mut legal = false;
                    for dir in 0..4 {
                        let new_r = row as i8 + dr[dir as usize];
                        let new_c = col as i8 + dc[dir as usize];
                        if new_r >= 0 && new_r < 8 && new_c >= 0 && new_c < 8 {
                            let new_r = new_r as u8;
                            let new_c = new_c as u8;
                            if state.get_board(new_r as usize, new_c as usize) == None {
                                legal = true;
                                break;
                            }
                        }
                        let dest_r = row as i8 + 2 * dr[dir as usize];
                        let dest_c = col as i8 + 2 * dc[dir as usize];
                        if dest_r >= 0 && dest_r < 8 && dest_c >= 0 && dest_c < 8 {
                            let dest_r = dest_r as u8;
                            let dest_c = dest_c as u8;
                            if state.is_crowned(row as usize, col as usize) == Some(true) || (state.get_color(row as usize, col as usize) == Some(true) && dir < 2) || (state.get_color(row as usize, col as usize) == Some(false) && dir > 1) {
                                if ((state.get_color(row as usize, col as usize) == Some(true) && state.get_color(new_r as usize, new_c as usize) == Some(false)) || (state.get_color(row as usize, col as usize) == Some(false) && state.get_color(new_r as usize, new_c as usize) == Some(true))) && state.get_board(dest_r as usize, dest_c as usize) == None {
                                    legal = true;
                                    break;
                                }
                            }
                        }
                    }
                    if !legal {
                        println!("This piece has no legal moves. Please select a different move.");
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
                    let new_r = row as i8 + dr[dir as usize];
                    let new_c = col as i8 + dc[dir as usize];
                    if new_r < 0 || new_r > 7 || new_c < 0 || new_c > 7 {
                        println!("The space you are trying to move to is out of range. Please select a different move.");
                        continue;
                    }
                    if (state.get_board(row as usize, col as usize) == Some(Piece::Black(false)) && dir > 1) || (state.get_board(row as usize, col as usize) == Some(Piece::White(false)) && dir < 2) {
                        println!("This piece can't legally move in this direction. Please select a different move.");
                        continue;
                    }
                    if state.get_color(row as usize, col as usize) == Some(true) {
                        if state.get_board(new_r as usize, new_c as usize) == None {
                            state.set_board(new_r as usize, new_c as usize, state.get_board(row as usize, col as usize));
                            state.set_board(row as usize, col as usize, None);
                            state.color = false;
                            state.player = true;
                            break;
                        }
                        let dest_r = row as i8 + 2 * dr[dir as usize];
                        let dest_c = col as i8 + 2 * dc[dir as usize];
                        if dest_r >= 0 && dest_r < 8 && dest_c >= 0 && dest_c < 8 {
                            let dest_r = dest_r as u8;
                            let dest_c = dest_c as u8;
                            if state.get_color(new_r as usize, new_c as usize) == Some(false) && state.get_board(dest_r as usize, dest_c as usize) == None {
                                state.set_board(dest_r as usize, dest_c as usize, state.get_board(row as usize, col as usize));
                                state.set_board(new_r as usize, new_c as usize, None);
                                state.set_board(row as usize, col as usize, None);
                                state.color = false;
                                state.player = true;
                                let row = dest_r;
                                let col = dest_c;
                                loop {
                                    let mut legal = false;
                                    for ind in 0..4 {
                                        let new_r = row as i8 + dr[ind];
                                        let new_c = col as i8 + dc[ind];
                                        let dest_r = row as i8 + 2 * dr[ind];
                                        let dest_c = col as i8 + 2 * dc[ind];
                                        if dest_r >= 0 && dest_r < 8 && dest_c >= 0 && dest_c < 8 {
                                            let new_r = new_r as u8;
                                            let new_c = new_c as u8;
                                            let dest_r = dest_r as u8;
                                            let dest_c = dest_c as u8;
                                            if state.is_crowned(row as usize, col as usize) == Some(true) || dir < 2 {
                                                if state.get_color(new_r as usize, new_c as usize) == Some(false) && state.get_board(dest_r as usize, dest_c as usize) == None {
                                                    legal = true;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    if !legal {
                                        break;
                                    }
                                    println!("Enter the direction you would like to move this piece: (0 = up and left, 1 = up and right, 2 = down and left, 3 = down and right)");
                                    let mut dir = String::new();
                                    io::stdin().read_line(&mut dir).expect("Failed to read input.");
                                    let dir: u8 = dir.trim().parse().expect("Not a number!");
                                    let new_r = row as i8 + dr[dir as usize];
                                    let new_c = col as i8 + dc[dir as usize];
                                    let dest_r = row as i8 + 2 * dr[dir as usize];
                                    let dest_c = col as i8 + 2 * dc[dir as usize];
                                    if dest_r >= 0 && dest_r < 8 && dest_c >= 0 && dest_c < 8 {
                                        let new_r = new_r as u8;
                                        let new_c = new_c as u8;
                                        let dest_r = dest_r as u8;
                                        let dest_c = dest_c as u8;
                                        if state.is_crowned(row as usize, col as usize) == Some(true) || dir < 2 {
                                            if state.get_color(new_r as usize, new_c as usize) == Some(false) && state.get_board(dest_r as usize, dest_c as usize) == None {
                                                state.set_board(dest_r as usize, dest_c as usize, state.get_board(row as usize, col as usize));
                                                state.set_board(new_r as usize, new_c as usize, None);
                                                state.set_board(row as usize, col as usize, None);
                                                continue;
                                            }
                                        }
                                    }
                                    println!("This piece can't legally move in this direction. Please select another move.");
                                }
                                break;
                            }
                            else {
                                println!("This move is illegal. Please select a different move.");
                                continue;
                            }
                        }
                        else {
                            println!("This piece has no legal moves in this direction. Please select a different move.");
                        }
                    }
                    else {
                        if state.get_board(new_r as usize, new_c as usize) == None {
                            state.set_board(new_r as usize, new_c as usize, state.get_board(row as usize, col as usize));
                            state.set_board(row as usize, col as usize, None);
                            state.color = true;
                            state.player = true;
                            break;
                        }
                        let dest_r = row as i8 + 2 * dr[dir as usize];
                        let dest_c = col as i8 + 2 * dc[dir as usize];
                        if dest_r >= 0 && dest_r < 8 && dest_c >= 0 && dest_c < 8 {
                            let dest_r = dest_r as u8;
                            let dest_c = dest_c as u8;
                            if state.get_color(new_r as usize, new_c as usize) == Some(true) && state.get_board(dest_r as usize, dest_c as usize) == None {
                                state.set_board(dest_r as usize, dest_c as usize, state.get_board(row as usize, col as usize));
                                state.set_board(new_r as usize, new_c as usize, None);
                                state.set_board(row as usize, col as usize, None);
                                state.color = true;
                                state.player = true;
                                let row = dest_r;
                                let col = dest_c;
                                loop {
                                    let mut legal = false;
                                    for ind in 0..4 {
                                        let new_r = row as i8 + dr[ind];
                                        let new_c = col as i8 + dc[ind];
                                        let dest_r = row as i8 + 2 * dr[ind];
                                        let dest_c = col as i8 + 2 * dc[ind];
                                        if dest_r >= 0 && dest_r < 8 && dest_c >= 0 && dest_c < 8 {
                                            let new_r = new_r as u8;
                                            let new_c = new_c as u8;
                                            let dest_r = dest_r as u8;
                                            let dest_c = dest_c as u8;
                                            if state.is_crowned(row as usize, col as usize) == Some(true) || dir > 1 {
                                                if state.get_color(new_r as usize, new_c as usize) == Some(true) && state.get_board(dest_r as usize, dest_c as usize) == None {
                                                    legal = true;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    if !legal {
                                        break;
                                    }
                                    println!("Enter the direction you would like to move this piece: (0 = up and left, 1 = up and right, 2 = down and left, 3 = down and right)");
                                    let mut dir = String::new();
                                    io::stdin().read_line(&mut dir).expect("Failed to read input.");
                                    let dir: u8 = dir.trim().parse().expect("Not a number!");
                                    let new_r = row as i8 + dr[dir as usize];
                                    let new_c = col as i8 + dc[dir as usize];
                                    let dest_r = row as i8 + 2 * dr[dir as usize];
                                    let dest_c = col as i8 + 2 * dc[dir as usize];
                                    if dest_r >= 0 && dest_r < 8 && dest_c >= 0 && dest_c < 8 {
                                        let new_r = new_r as u8;
                                        let new_c = new_c as u8;
                                        let dest_r = dest_r as u8;
                                        let dest_c = dest_c as u8;
                                        if state.is_crowned(row as usize, col as usize) == Some(true) || dir > 1 {
                                            if state.get_color(new_r as usize, new_c as usize) == Some(true) && state.get_board(dest_r as usize, dest_c as usize) == None {
                                                state.set_board(dest_r as usize, dest_c as usize, state.get_board(row as usize, col as usize));
                                                state.set_board(new_r as usize, new_c as usize, None);
                                                state.set_board(row as usize, col as usize, None);
                                                continue;
                                            }
                                        }
                                    }
                                    println!("This piece can't legally move in this direction. Please select another move.");
                                }
                                break;
                            }
                            else {
                                println!("This move is illegal. Please select a different move.");
                                continue;
                            }
                        }
                        else {
                            println!("This piece has no legal moves in this direction. Please select a different move.");
                        }
                    }
                }
            }
            println!("{state}");
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
    }*/
}