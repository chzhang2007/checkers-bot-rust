use std::io;
use rand::Rng;
use crate::board::{Piece, State};
mod board;

fn dfs(state1: &State, row1: &usize, col1: &usize) -> Vec<State> {
    let mut state = *state1;
    let row = *row1;
    let col = *col1;
    let dr: [i8; 4] = [-1, -1, 1, 1];
    let dc: [i8; 4] = [-1, 1, -1, 1];
    let mut vec = Vec::new();
    let mut cont = false;
    for dir in 0..4 {
        let new_r = row as i8 + dr[dir];
        let new_c = col as i8 + dc[dir];
        let dest_r = row as i8 + 2 * dr[dir];
        let dest_c = col as i8 + 2 * dc[dir];
        if dest_r >= 0 && dest_r < 8 && dest_c >= 0 && dest_c < 8 {
            if state.is_crowned(row, col) == Some(true) || (state.get_color(row, col) == Some(true) && dir < 2) || (state.get_color(row, col) == Some(false) && dir > 1) {
                if ((state.get_color(row, col) == Some(true) && state.get_color(new_r as usize, new_c as usize) == Some(false)) || (state.get_color(row, col) == Some(false) && state.get_color(new_r as usize, new_c as usize) == Some(true))) && state.get_board(dest_r as usize, dest_c as usize) == None {
                    cont = true;
                    let mut crown = false;
                    let piece = state.get_board(new_r as usize, new_c as usize);
                    state.set_board(dest_r as usize, dest_c as usize, state.get_board(row, col));
                    state.set_board(new_r as usize, new_c as usize, None);
                    state.set_board(row, col, None);
                    if (dest_r == 0 && state.get_board(dest_r as usize, dest_c as usize) == Some(Piece::Black(false))) || (dest_r == 7 && state.get_board(dest_r as usize, dest_c as usize) == Some(Piece::White(false))) {
                        state.toggle_crown(dest_r as usize, dest_c as usize);
                        crown = true;
                    }
                    let subvec = dfs(&state, &(dest_r as usize), &(dest_c as usize));
                    for val in subvec {
                        vec.push(val);
                    }
                    state.set_board(row, col, state.get_board(dest_r as usize, dest_c as usize));
                    state.set_board(new_r as usize, new_c as usize, piece);
                    state.set_board(dest_r as usize, dest_c as usize, None);
                    if crown {
                        state.toggle_crown(row, col);
                    }
                }
            }
        }
    }
    if !cont {
        let mut statevec = Vec::new();
        statevec.push(state);
        return statevec;
    }
    return vec;
}
fn children(state1: &State, row1: &usize, col1: &usize, dfs_only: bool) -> Vec<State> {
    let mut vec: Vec<State> = Vec::new();
    let mut state: State = *state1;
    let row: usize = *row1;
    let col: usize = *col1;
    let dr: [i8; 4] = [-1, -1, 1, 1];
    let dc: [i8; 4] = [-1, 1, -1, 1];
    let v: Vec<State> = dfs(&state, &row, &col);
    for val in &v {
        if !state.board_eq(*val) {
            vec.push(*val);
        }
    }
    if !dfs_only {
        for dir in 0..4 {
            let new_r = row as i8 + dr[dir];
            let new_c = col as i8 + dc[dir];
            let mut crown = false;
            if new_r >= 0 && new_r < 8 && new_c >= 0 && new_c < 8 {
                if state.is_crowned(row, col) == Some(true) || (state.get_color(row, col) == Some(true) && dir < 2) || (state.get_color(row, col) == Some(false) && dir > 1) {
                    if state.get_board(new_r as usize, new_c as usize) == None {
                        state.set_board(new_r as usize, new_c as usize, state.get_board(row, col));
                        state.set_board(row, col, None);
                        if (new_r == 0 && state.get_board(new_r as usize, new_c as usize) == Some(Piece::Black(false))) || (new_r == 7 && state.get_board(new_r as usize, new_c as usize) == Some(Piece::White(false))) {
                            state.toggle_crown(new_r as usize, new_c as usize);
                            crown = true;
                        }
                        vec.push(state.clone());
                        state.set_board(row, col, state.get_board(new_r as usize, new_c as usize));
                        state.set_board(new_r as usize, new_c as usize, None);
                        if crown {
                            state.toggle_crown(row, col);
                        }
                    }
                }
            }
        }
    }
    return vec;
}
fn terminal(state: &State) -> Option<bool> { //not terminal = None, black wins = Some(true), white wins = Some(false)
    let mut b: bool = false;
    let mut w: bool = false;
    for row in 0..8 {
        for col in 0..8 {
            if state.get_color(row, col) == Some(true) {
                let vec = children(&state, &row, &col, false);
                if !vec.is_empty() {
                    b = true;
                }
            }
            else if state.get_color(row, col) == Some(false) {
                let vec = children(&state, &row, &col, false);
                if !vec.is_empty() {
                    w = true;
                }
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
fn eval(state: &State) -> i64 {
    let mut score: i64 = 0;
    if terminal(state) == Some(true) {
        return 10000;
    }
    else if terminal(state) == Some(false) {
        return -10000;
    }
    for row in 0..8 {
        for col in 0..8 {
            if state.get_color(row, col) == None {
                continue;
            }
            if state.get_color(row, col) == Some(true) {
                if state.is_crowned(row, col) == Some(true) {
                    score += 30;
                }
                else {
                    score += 20 + (7 - row as i64);
                }
            }
            else if state.get_color(row, col) == Some(false) {
                if state.is_crowned(row, col) == Some(true) {
                    score -= 30;
                }
                else {
                    score -= 20 + row as i64;
                }
            }
        }
    }
    score
}
fn minimax(state1: &State, depth: &u8, same: &bool) -> (State, i64) { //max player = black, min player = white
    let state: State = *state1;
    let mut new_state = state;
    if *depth == 5 || terminal(&state) != None {
        return (state, eval(&state));
    }
    let mut jump = false;
    for row in 0..8 {
        for col in 0..8 {
            if state.get_color(row, col) != Some(state.color) {
                continue;
            }
            let vec = children(&state, &row, &col, true);
            if !vec.is_empty() {
                jump = true;
                break;
            }
        }
        if jump {
            break;
        }
    }
    println!("{}", jump);
    if state.color == true { //max player
        let mut val = -1000000000;
        for row in 0..8 {
            for col in 0..8 {
                if state.get_color(row, col) == Some(true) {
                    let vec = children(&state, &row, &col, jump);
                    for i in &vec {
                        if state.board_eq(*i) {
                            continue;
                        }
                        let mut temp = *i;
                        temp.player = !state.player;
                        if *same {
                            temp.color = i.player;
                        }
                        else {
                            temp.color = !i.player;
                        }
                        let tup = minimax(&temp, &(depth + 1), &same);
                        if tup.1 > val {
                            new_state = temp;
                            val = tup.1;
                        }
                    }
                }
            }
        }
        return (new_state, val);
    }
    else { //min player
        let mut val = 1000000000;
        for row in 0..8 {
            for col in 0..8 {
                if state.get_color(row, col) == Some(false) {
                    let vec = children(&state, &row, &col, jump);
                    for i in &vec {
                        if state.board_eq(*i) {
                            continue;
                        }
                        let mut temp = *i;
                        temp.player = !state.player;
                        if *same {
                            temp.color = i.player;
                        }
                        else {
                            temp.color = !i.player;
                        }
                        let tup = minimax(&temp, &(depth + 1), &same);
                        if tup.1 < val {
                            new_state = temp;
                            val = tup.1;
                        }
                    }
                }
            }
        }
        return (new_state, val);
    }
}
fn main() {
    loop {
        println!("Hello! Please enter 0 for a link to the rules, or enter any other number to start playing!");
        let mut state = State::new();
        let mut inp = String::new();
        let mut same = false;
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
        if state.color == state.player {
            same = true;
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
            let mut moves = false;
            let mut jump = false;
            for r in 0..8 {
                for c in 0..8 {
                    if state.get_color(r, c) == Some(state.color) {
                        let vec = children(&state, &r, &c, false);
                        if !vec.is_empty() {
                            moves = true;
                        }
                        let vec = children(&state, &r, &c, true);
                        if !vec.is_empty() {
                            jump = true;
                            break;
                        }
                    }
                }
                if jump {
                    break;
                }
            }
            if !moves && state.color == true {
                println!("White won!");
                break;
            }
            else if !moves {
                println!("Black won!");
            }
            if state.player {
                let tup = minimax(&state, &0, &same);
                state = tup.0;
                state.player = false;
                if same {
                    state.color = state.player;
                }
                else {
                    state.color = !state.player;
                }
                println!("The computer has made its move.");
                println!("{}", tup.1);
            }
            else {
                loop {
                    println!("Enter the row number of the piece you would like to move: (0-7)");
                    let mut row = String::new();
                    io::stdin().read_line(&mut row).expect("Failed to read input.");
                    let row: u16 = row.trim().parse().expect("Not a number!");
                    println!("Enter the column number of the piece you would like to move: (0-7)");
                    let mut col = String::new();
                    io::stdin().read_line(&mut col).expect("Failed to read input.");
                    let col: u16 = col.trim().parse().expect("Not a number!");
                    if row > 7 || col > 7 {
                        println!("Position out of bounds! Please enter numbers from 0-7.");
                        continue;
                    }
                    if state.get_color(row as usize, col as usize) != Some(state.color) {
                        println!("The selected space does not contain one of your pieces. Please select a different space.");
                        continue;
                    }
                    let vec = children(&state, &(row as usize), &(col as usize), false);
                    if vec.is_empty() {
                        println!("The selected space does not have any legal moves. Please select a different space.");
                        continue;
                    }
                    let dr: [i8; 4] = [-1, -1, 1, 1];
                    let dc: [i8; 4] = [-1, 1, -1, 1];
                    println!("Enter the direction you would like to move this piece: (0 = up and left, 1 = up and right, 2 = down and left, 3 = down and right)");
                    let mut dir = String::new();
                    io::stdin().read_line(&mut dir).expect("Failed to read input.");
                    let dir: u8 = dir.trim().parse().expect("Not a number!");
                    if dir > 3 {
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
                            if jump {
                                println!("You have the opportunity to jump a checker, so you must do so. Please select a different move.");
                                continue;
                            }
                            state.set_board(new_r as usize, new_c as usize, state.get_board(row as usize, col as usize));
                            state.set_board(row as usize, col as usize, None);
                            state.color = false;
                            state.player = true;
                            if new_r == 0 && state.is_crowned(new_r as usize, new_c as usize) == Some(false) {
                                state.toggle_crown(new_r as usize, new_c as usize);
                            }
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
                                if dest_r == 0 && state.is_crowned(dest_r as usize, dest_c as usize) == Some(false) {
                                    state.toggle_crown(dest_r as usize, dest_c as usize);
                                }
                                let row = dest_r;
                                let col = dest_c;
                                loop {
                                    let vec = children(&state, &(row as usize), &(col as usize), true);
                                    if vec.is_empty() {
                                        break;
                                    }
                                    println!("{state}");
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
                                                state.color = false;
                                                state.player = true;
                                                if dest_r == 0 && state.is_crowned(dest_r as usize, dest_c as usize) == Some(false) {
                                                    state.toggle_crown(dest_r as usize, dest_c as usize);
                                                }
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
                            if jump {
                                println!("You have the opportunity to jump a checker, so you must do so. Please select a different move.");
                                continue;
                            }
                            state.set_board(new_r as usize, new_c as usize, state.get_board(row as usize, col as usize));
                            state.set_board(row as usize, col as usize, None);
                            state.color = true;
                            state.player = true;
                            if new_r == 7 && state.is_crowned(new_r as usize, new_c as usize) == Some(false) {
                                state.toggle_crown(new_r as usize, new_c as usize);
                            }
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
                                if dest_r == 7 && state.is_crowned(dest_r as usize, dest_c as usize) == Some(false) {
                                    state.toggle_crown(dest_r as usize, dest_c as usize);
                                }
                                let row = dest_r;
                                let col = dest_c;
                                loop {
                                    let vec = children(&state, &(row as usize), &(col as usize), true);
                                    if vec.is_empty() {
                                        break;
                                    }
                                    println!("{state}");
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
                                                if dest_r == 7 && state.is_crowned(dest_r as usize, dest_c as usize) == Some(false) {
                                                    state.toggle_crown(dest_r as usize, dest_c as usize);
                                                }
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
    }
}