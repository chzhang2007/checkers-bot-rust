use std::io;
use rand::Rng;
use crate::board::{Piece, State};
mod board;

const DR: [i8; 4] = [-1, -1, 1, 1];
const DC: [i8; 4] = [-1, 1, -1, 1];
const TERMNUM: i32 = 100000;

fn dfs(state: &State, row: usize, col: usize, comp_col: bool) -> Vec<State> {
    let mut state = state.clone();
    let mut vec = vec![];
    let mut cont = false;
    for dir in 0..4 {
        let new_r = row as i8 + DR[dir];
        let new_c = col as i8 + DC[dir];
        let dest_r = row as i8 + 2 * DR[dir];
        let dest_c = col as i8 + 2 * DC[dir];
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
                    let subvec = dfs(&state, dest_r as usize, dest_c as usize, comp_col);
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
        let mut statevec = vec![];
        statevec.push(state);
        return statevec;
    }
    return vec;
}
fn children(state: &State, row: usize, col: usize, dfs_only: bool, comp_col: bool) -> Vec<State> {
    let mut state = state.clone();
    let mut states: Vec<State> = vec![];
    let v: Vec<State> = dfs(&state, row, col, comp_col);
    for val in &v {
        if state.ne(val) {
            states.push(*val);
        }
    }
    if !dfs_only {
        for dir in 0..4 {
            let new_r = row as i8 + DR[dir];
            let new_c = col as i8 + DC[dir];
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
                        states.push(state.clone());
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
    return states;
}
fn terminal(state: &State, comp_col: bool) -> Option<bool> { //not terminal = None, black wins = Some(true), white wins = Some(false)
    let mut b: bool = false;
    let mut w: bool = false;
    for row in 0..8 {
        for col in 0..8 {
            if state.get_color(row, col) == Some(true) {
                let vec = children(&state, row, col, false, comp_col);
                if !vec.is_empty() {
                    b = true;
                }
            }
            else if state.get_color(row, col) == Some(false) {
                let vec = children(&state, row, col, false, comp_col);
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
fn eval(state: &State, comp_col: bool) -> f64 {
    match terminal(state, comp_col) {
        Some(true) => return TERMNUM as f64,
        Some(false) => return -TERMNUM as f64,
        None => ()
    }
    let mut score = 0.0;
    for row in 0..8 {
        for col in 0..8 {
            let mut base = 0.0;
            let mut mult = 1.0;
            if state.get_color(row, col) == Some(true) {
                if state.is_crowned(row, col) == Some(true) {
                    base = 50.0;
                }
                else {
                    base = 30.0 + (7.0 - row as f64);
                }
                for dir in 0..4 {
                    let new_r = row as i8 + DR[dir];
                    let new_c = col as i8 + DC[dir];
                    if new_r >= 0 && new_r < 8 && new_c >= 0 && new_c < 8 {
                        if state.get_color(new_r as usize, new_c as usize) == Some(true) {
                            mult += 0.1;
                        }
                        else if state.get_color(new_r as usize, new_c as usize) == Some(false) && (state.is_crowned(new_r as usize, new_c as usize) == Some(true) || dir < 2) {
                            let dest_r = new_r + DR[dir];
                            let dest_c = new_c + DC[dir];
                            if dest_r >= 0 && dest_r < 8 && dest_c >= 0 && dest_c < 8 && state.get_board(dest_r as usize, dest_c as usize) == None {
                                if state.player == comp_col {
                                    mult += 0.3;
                                }
                                else {
                                    base = 0.0;
                                }
                            }
                        }
                    }
                    else {
                        mult += 0.1;
                    }
                }
            }
            else if state.get_color(row, col) == Some(false) {
                if state.is_crowned(row, col) == Some(true) {
                    base = -50.0;
                }
                else {
                    base = -30.0 - row as f64;
                }
                for dir in 0..4 {
                    let new_r = row as i8 + DR[dir];
                    let new_c = col as i8 + DC[dir];
                    if new_r >= 0 && new_r < 8 && new_c >= 0 && new_c < 8 {
                        if state.get_color(new_r as usize, new_c as usize) == Some(true) {
                            mult += 0.1;
                        }
                        else if state.get_color(new_r as usize, new_c as usize) == Some(false) && (state.is_crowned(new_r as usize, new_c as usize) == Some(true) || dir > 1) {
                            let dest_r = new_r + DR[dir];
                            let dest_c = new_c + DC[dir];
                            if dest_r >= 0 && dest_r < 8 && dest_c >= 0 && dest_c < 8 && state.get_board(dest_r as usize, dest_c as usize) == None {
                                if state.player == comp_col {
                                    base = 0.0;
                                }
                                else {
                                    mult += 0.3;
                                }
                            }
                        }
                    }
                    else {
                        mult += 0.1;
                    }
                }
            }
            score += base * mult;
        }
    }
    score
}
fn minimax(state: &State, depth: u8, term: u8, comp_col: bool) -> (State, f64) { //max player = black, min player = white
    let state: State = state.clone();
    let mut new_state = state;
    if depth == term || terminal(&state, comp_col) != None {
        return (state, eval(&state, comp_col));
    }
    let mut jump = false;
    for row in 0..8 {
        for col in 0..8 {
            if state.get_color(row, col) != Some(state.player == comp_col) {
                continue;
            }
            let vec = children(&state, row, col, true, comp_col);
            if !vec.is_empty() {
                jump = true;
                break;
            }
        }
        if jump {
            break;
        }
    }
    if state.player == comp_col { //max player
        let mut val = -2.0 * TERMNUM as f64;
        for row in 0..8 {
            for col in 0..8 {
                if state.get_color(row, col) == Some(true) {
                    let vec = children(&state, row, col, jump, comp_col);
                    for i in &vec {
                        if state.eq(i) {
                            continue;
                        }
                        let mut temp = *i;
                        temp.player = !state.player;
                        let tup = minimax(&temp, depth + 1, term, comp_col);
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
        let mut val = 2.0 * TERMNUM as f64;
        for row in 0..8 {
            for col in 0..8 {
                if state.get_color(row, col) == Some(false) {
                    let vec = children(&state, row, col, jump, comp_col);
                    for i in &vec {
                        if state.eq(i) {
                            continue;
                        }
                        let mut temp = *i;
                        temp.player = !state.player;
                        let tup = minimax(&temp, depth + 1, term, comp_col);
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
    println!("Hello! Please enter 0 for a link to the rules, or enter anything else to start playing!");
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Failed to read input.");
    if inp.trim().len() == 1 && inp.trim().chars().next().unwrap() == '0' {
        println!("You can find the rules at https://www.wikihow.com/Play-Checkers.");
    }
    else if inp.trim().len() == 1 && inp.trim().chars().next().unwrap() == 'q' {
        panic!("Quit");
    }
    loop {
        let mut state = State::new();
        let comp_col;
        println!("Please enter your desired difficulty: 0 (easiest) to 2 (hardest). Higher difficulty bots will take longer to make moves.");
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Failed to read input.");
        if inp.trim().len() != 1 {
            println!("Invalid input!");
            continue;
        }
        let inp: char = inp.trim().chars().next().unwrap();
        let term: u8;
        if inp == 'q' {
            panic!("Quit");
        }
        else if inp == '0' {
            term = 3;
        }
        else if inp == '1' {
            term = 5;
        }
        else if inp == '2' {
            term = 7;
        }
        else {
            println!("Invalid input!");
            continue;
        }
        println!("{state}");
        println!("_ : empty space");
        println!("b : uncrowned black checker");
        println!("B : crowned black checker");
        println!("w : uncrowned white checker");
        println!("W : crowned white checker");
        println!();
        let rnum: u8 = rand::thread_rng().gen_range(1..=2);
        if rnum == 1 {
            println!("You are playing black.");
            state.player = false;
            comp_col = false;
        }
        else {
            println!("You are playing white.");
            comp_col = true;
        }
        println!();
        loop {
            let end = terminal(&state, comp_col);
            if end == Some(true) {
                println!("{state}");
                println!("Black won!");
                break;
            }
            else if end == Some(false) {
                println!("{state}");
                println!("White won!");
                break;
            }
            let mut moves = false;
            let mut jump = false;
            for r in 0..8 {
                for c in 0..8 {
                    if state.get_color(r, c) == Some(state.player == comp_col) {
                        let vec = children(&state, r, c, false, comp_col);
                        if !vec.is_empty() {
                            moves = true;
                        }
                        let vec = children(&state, r, c, true, comp_col);
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
            if !moves && state.player == comp_col {
                println!("White won!");
                break;
            }
            else if !moves {
                println!("Black won!");
            }
            if state.player {
                if term == 7 {
                    println!("Please wait for the computer to make its move...");
                }
                let tup = minimax(&state, 0, term, comp_col);
                state = tup.0;
                state.player = false;
                println!("The computer has made its move.");
                println!();
            }
            else {
                loop {
                    println!("Enter the row number of the piece you would like to move: (0-7)");
                    let mut row = String::new();
                    io::stdin().read_line(&mut row).expect("Failed to read input.");
                    if row.trim().len() != 1 {
                        println!("Invalid input!");
                        continue;
                    }
                    let row: char = row.trim().chars().next().unwrap();
                    if row == 'q' {
                        panic!("Quit");
                    }
                    if row < '0' || row > '7' {
                        println!("Invalid input!");
                        continue;
                    }
                    let row: u8 = row as u8 - '0' as u8;
                    println!("Enter the column number of the piece you would like to move: (0-7)");
                    let mut col = String::new();
                    io::stdin().read_line(&mut col).expect("Failed to read input.");
                    if col.trim().len() != 1 {
                        println!("Invalid input!");
                        continue;
                    }
                    let col: char = col.trim().chars().next().unwrap();
                    if col == 'q' {
                        panic!("Quit");
                    }
                    if col < '0' || col > '7' {
                        println!("Invalid input!");
                        continue;
                    }
                    let col: u8 = col as u8 - '0' as u8;
                    if state.get_color(row as usize, col as usize) != Some(state.player == comp_col) {
                        println!("The selected space does not contain one of your pieces. Please select a different space.");
                        continue;
                    }
                    let vec = children(&state, row as usize, col as usize, false, comp_col);
                    if vec.is_empty() {
                        println!("The selected space does not have any legal moves. Please select a different space.");
                        continue;
                    }
                    println!("Enter the direction you would like to move this piece: (0 = up and left, 1 = up and right, 2 = down and left, 3 = down and right)");
                    let mut dir = String::new();
                    io::stdin().read_line(&mut dir).expect("Failed to read input.");
                    if dir.trim().len() != 1 {
                        println!("Invalid input!");
                        continue;
                    }
                    let dir: char = dir.trim().chars().next().unwrap();
                    if dir == 'q' {
                        panic!("Quit");
                    }
                    if dir < '0' || dir > '3' {
                        println!("Invalid input!");
                        continue;
                    }
                    let dir: u8 = dir as u8 - '0' as u8;
                    let new_r = row as i8 + DR[dir as usize];
                    let new_c = col as i8 + DC[dir as usize];
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
                            state.player = true;
                            if new_r == 0 && state.is_crowned(new_r as usize, new_c as usize) == Some(false) {
                                state.toggle_crown(new_r as usize, new_c as usize);
                            }
                            break;
                        }
                        let dest_r = row as i8 + 2 * DR[dir as usize];
                        let dest_c = col as i8 + 2 * DC[dir as usize];
                        if dest_r >= 0 && dest_r < 8 && dest_c >= 0 && dest_c < 8 {
                            let dest_r = dest_r as u8;
                            let dest_c = dest_c as u8;
                            if state.get_color(new_r as usize, new_c as usize) == Some(false) && state.get_board(dest_r as usize, dest_c as usize) == None {
                                state.set_board(dest_r as usize, dest_c as usize, state.get_board(row as usize, col as usize));
                                state.set_board(new_r as usize, new_c as usize, None);
                                state.set_board(row as usize, col as usize, None);
                                state.player = true;
                                if dest_r == 0 && state.is_crowned(dest_r as usize, dest_c as usize) == Some(false) {
                                    state.toggle_crown(dest_r as usize, dest_c as usize);
                                }
                                let row = dest_r;
                                let col = dest_c;
                                loop {
                                    let vec = children(&state, row as usize, col as usize, true, comp_col);
                                    if vec.is_empty() {
                                        break;
                                    }
                                    println!("{state}");
                                    println!("Enter the direction you would like to move this piece: (0 = up and left, 1 = up and right, 2 = down and left, 3 = down and right)");
                                    let mut dir = String::new();
                                    io::stdin().read_line(&mut dir).expect("Failed to read input.");
                                    if dir.trim().len() != 1 {
                                        println!("Invalid input!");
                                        continue;
                                    }
                                    let dir: char = dir.trim().chars().next().unwrap();
                                    if dir == 'q' {
                                        panic!("Quit");
                                    }
                                    if dir < '0' || dir > '3' {
                                        println!("Invalid input!");
                                        continue;
                                    }
                                    let dir: u8 = dir as u8 - '0' as u8;
                                    let new_r = row as i8 + DR[dir as usize];
                                    let new_c = col as i8 + DC[dir as usize];
                                    let dest_r = row as i8 + 2 * DR[dir as usize];
                                    let dest_c = col as i8 + 2 * DC[dir as usize];
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
                            state.player = true;
                            if new_r == 7 && state.is_crowned(new_r as usize, new_c as usize) == Some(false) {
                                state.toggle_crown(new_r as usize, new_c as usize);
                            }
                            break;
                        }
                        let dest_r = row as i8 + 2 * DR[dir as usize];
                        let dest_c = col as i8 + 2 * DC[dir as usize];
                        if dest_r >= 0 && dest_r < 8 && dest_c >= 0 && dest_c < 8 {
                            let dest_r = dest_r as u8;
                            let dest_c = dest_c as u8;
                            if state.get_color(new_r as usize, new_c as usize) == Some(true) && state.get_board(dest_r as usize, dest_c as usize) == None {
                                state.set_board(dest_r as usize, dest_c as usize, state.get_board(row as usize, col as usize));
                                state.set_board(new_r as usize, new_c as usize, None);
                                state.set_board(row as usize, col as usize, None);
                                state.player = true;
                                if dest_r == 7 && state.is_crowned(dest_r as usize, dest_c as usize) == Some(false) {
                                    state.toggle_crown(dest_r as usize, dest_c as usize);
                                }
                                let row = dest_r;
                                let col = dest_c;
                                loop {
                                    let vec = children(&state, row as usize, col as usize, true, comp_col);
                                    if vec.is_empty() {
                                        break;
                                    }
                                    println!("{state}");
                                    println!("Enter the direction you would like to move this piece: (0 = up and left, 1 = up and right, 2 = down and left, 3 = down and right)");
                                    let mut dir = String::new();
                                    io::stdin().read_line(&mut dir).expect("Failed to read input.");
                                    if dir.trim().len() != 1 {
                                        println!("Invalid input!");
                                        continue;
                                    }
                                    let dir: char = dir.trim().chars().next().unwrap();
                                    if dir == 'q' {
                                        panic!("Quit");
                                    }
                                    if dir < '0' || dir > '3' {
                                        println!("Invalid input!");
                                        continue;
                                    }
                                    let dir: u8 = dir as u8 - '0' as u8;
                                    let new_r = row as i8 + DR[dir as usize];
                                    let new_c = col as i8 + DC[dir as usize];
                                    let dest_r = row as i8 + 2 * DR[dir as usize];
                                    let dest_c = col as i8 + 2 * DC[dir as usize];
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
        println!("Enter 0 to play again, or enter anything else to quit.");
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Failed to read input.");
        if inp.trim().len() != 1 || inp.trim().chars().next().unwrap() != '0' {
            println!("You have quit the game.");
            break;
        }
        println!();
    }
}