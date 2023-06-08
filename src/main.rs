use std::io;
use rand::Rng;
use std::clone::Clone;

#[derive(Clone)]
struct State {
    board: [[char; 8]; 8],
    color: bool, //black = true. white = false
    player: bool, //computer = true, user = false
}
fn terminal(state: &State) -> i8 {
    let mut b: bool = false;
    let mut w: bool = false;
    for i in 0..8 {
        for j in 0..8 {
            if state.board[i][j] == 'B' || state.board[i][j] == 'C' {
                b = true;
            }
            else if state.board[i][j] == 'W' || state.board[i][j] == 'X' {
                w = true;
            }
        }
    }
    if !b {
        return -1;
    }
    else if !w {
        return 1;
    }
    return 0;
}
fn dfs_b(state: &mut State, row: usize, col: usize) -> Vec<State> {
    let mut vec: Vec<State> = Vec::new();
    let copy = (*state).clone();
    vec.push(copy);
    if row < 1 {
        return vec;
    }
    if col > 1 && (state.board[row - 1][col - 1] == 'W' || state.board[row - 1][col - 1] == 'X') && state.board[row - 2][col - 2] == '_' {
        let piece: char = state.board[row - 1][col - 1];
        state.board[row][col] = '_';
        state.board[row - 1][col - 1] = '_';
        state.board[row - 2][col - 2] = 'B';
        vec.append(&mut dfs_b(state, row - 2, col - 2));
        state.board[row][col] = 'B';
        state.board[row - 1][col - 1] = piece;
        state.board[row - 2][col - 2] = '_';
    }
    if col < 6 && (state.board[row - 1][col + 1] == 'W' || state.board[row - 1][col + 1] == 'X') && state.board[row - 2][col + 2] == '_' {
        let piece: char = state.board[row - 1][col + 1];
        state.board[row][col] = '_';
        state.board[row - 1][col + 1] = '_';
        state.board[row - 2][col + 2] = 'B';
        vec.append(&mut dfs_b(state, row + 2, col + 2));
        state.board[row][col] = 'B';
        state.board[row - 1][col + 1] = piece;
        state.board[row - 2][col + 2] = '_';
    }
    return vec;
}
fn dfs_c(state: &mut State, row: usize, col: usize) -> Vec<State> {
    let mut vec: Vec<State> = Vec::new();
    let copy = (*state).clone();
    vec.push(copy);
    if col > 1 && (state.board[row - 1][col - 1] == 'W' || state.board[row - 1][col - 1] == 'X') && state.board[row - 2][col - 2] == '_' {
        let piece: char = state.board[row - 1][col - 1];
        state.board[row][col] = '_';
        state.board[row - 1][col - 1] = '_';
        state.board[row - 2][col - 2] = 'C';
        vec.append(&mut dfs_b(state, row - 2, col - 2));
        state.board[row][col] = 'C';
        state.board[row - 1][col - 1] = piece;
        state.board[row - 2][col - 2] = '_';
    }
    if col < 6 && (state.board[row - 1][col + 1] == 'W' || state.board[row - 1][col + 1] == 'X') && state.board[row - 2][col + 2] == '_' {
        let piece: char = state.board[row - 1][col + 1];
        state.board[row][col] = '_';
        state.board[row - 1][col + 1] = '_';
        state.board[row - 2][col + 2] = 'C';
        vec.append(&mut dfs_b(state, row + 2, col + 2));
        state.board[row][col] = 'C';
        state.board[row - 1][col + 1] = piece;
        state.board[row - 2][col + 2] = '_';
    }
    if col > 1 && (state.board[row + 1][col - 1] == 'W' || state.board[row + 1][col - 1] == 'X') && state.board[row + 2][col - 2] == '_' {
        let piece: char = state.board[row + 1][col - 1];
        state.board[row][col] = '_';
        state.board[row + 1][col - 1] = '_';
        state.board[row + 2][col - 2] = 'C';
        vec.append(&mut dfs_b(state, row - 2, col - 2));
        state.board[row][col] = 'C';
        state.board[row + 1][col - 1] = piece;
        state.board[row + 2][col - 2] = '_';
    }
    if col < 6 && (state.board[row + 1][col + 1] == 'W' || state.board[row + 1][col + 1] == 'X') && state.board[row + 2][col + 2] == '_' {
        let piece: char = state.board[row + 1][col + 1];
        state.board[row][col] = '_';
        state.board[row + 1][col + 1] = '_';
        state.board[row + 2][col + 2] = 'C';
        vec.append(&mut dfs_b(state, row + 2, col + 2));
        state.board[row][col] = 'C';
        state.board[row + 1][col + 1] = piece;
        state.board[row + 2][col + 2] = '_';
    }
    return vec;
}
fn dfs_w(state: &mut State, row: usize, col: usize) -> Vec<State> {
    let mut vec: Vec<State> = Vec::new();
    let copy = (*state).clone();
    vec.push(copy);
    if row > 6 {
        return vec;
    }
    if col > 1 && (state.board[row + 1][col - 1] == 'B' || state.board[row + 1][col - 1] == 'C') && state.board[row + 2][col - 2] == '_' {
        let piece: char = state.board[row + 1][col - 1];
        state.board[row][col] = '_';
        state.board[row + 1][col - 1] = '_';
        state.board[row + 2][col - 2] = 'W';
        vec.append(&mut dfs_b(state, row - 2, col - 2));
        state.board[row][col] = 'W';
        state.board[row + 1][col - 1] = piece;
        state.board[row + 2][col - 2] = '_';
    }
    if col < 6 && (state.board[row + 1][col + 1] == 'B' || state.board[row + 1][col + 1] == 'C') && state.board[row + 2][col + 2] == '_' {
        let piece: char = state.board[row + 1][col + 1];
        state.board[row][col] = '_';
        state.board[row + 1][col + 1] = '_';
        state.board[row + 2][col + 2] = 'W';
        vec.append(&mut dfs_b(state, row + 2, col + 2));
        state.board[row][col] = 'W';
        state.board[row + 1][col + 1] = piece;
        state.board[row + 2][col + 2] = '_';
    }
    return vec;
}
fn dfs_x(state: &mut State, row: usize, col: usize) -> Vec<State> {
    let mut vec: Vec<State> = Vec::new();
    let copy = (*state).clone();
    vec.push(copy);
    if col > 1 && (state.board[row - 1][col - 1] == 'B' || state.board[row - 1][col - 1] == 'C') && state.board[row - 2][col - 2] == '_' {
        let piece: char = state.board[row - 1][col - 1];
        state.board[row][col] = '_';
        state.board[row - 1][col - 1] = '_';
        state.board[row - 2][col - 2] = 'X';
        vec.append(&mut dfs_b(state, row - 2, col - 2));
        state.board[row][col] = 'X';
        state.board[row - 1][col - 1] = piece;
        state.board[row - 2][col - 2] = '_';
    }
    if col < 6 && (state.board[row - 1][col + 1] == 'B' || state.board[row - 1][col + 1] == 'C') && state.board[row - 2][col + 2] == '_' {
        let piece: char = state.board[row - 1][col + 1];
        state.board[row][col] = '_';
        state.board[row - 1][col + 1] = '_';
        state.board[row - 2][col + 2] = 'X';
        vec.append(&mut dfs_b(state, row + 2, col + 2));
        state.board[row][col] = 'X';
        state.board[row - 1][col + 1] = piece;
        state.board[row - 2][col + 2] = '_';
    }
    if col > 1 && (state.board[row + 1][col - 1] == 'B' || state.board[row + 1][col - 1] == 'C') && state.board[row + 2][col - 2] == '_' {
        let piece: char = state.board[row + 1][col - 1];
        state.board[row][col] = '_';
        state.board[row + 1][col - 1] = '_';
        state.board[row + 2][col - 2] = 'X';
        vec.append(&mut dfs_b(state, row - 2, col - 2));
        state.board[row][col] = 'X';
        state.board[row + 1][col - 1] = piece;
        state.board[row + 2][col - 2] = '_';
    }
    if col < 6 && (state.board[row + 1][col + 1] == 'B' || state.board[row + 1][col + 1] == 'C') && state.board[row + 2][col + 2] == '_' {
        let piece: char = state.board[row + 1][col + 1];
        state.board[row][col] = '_';
        state.board[row + 1][col + 1] = '_';
        state.board[row + 2][col + 2] = 'X';
        vec.append(&mut dfs_b(state, row + 2, col + 2));
        state.board[row][col] = 'X';
        state.board[row + 1][col + 1] = piece;
        state.board[row + 2][col + 2] = '_';
    }
    return vec;
}
fn children(state: &mut State, row: i8, col: i8) -> Vec<State> {
    let mut vec: Vec<State> = Vec::new();
    if state.color {
        for i in 0..8 {
            for j in 0..8 {
                if state.board[i][j] == 'B' {
                    if i > 1 {
                        vec.append(&mut dfs_b(state, i, j));
                    }
                }
                else if state.board[i][j] == 'C' {

                }
            }
        }
    }
    else {

    }
    return vec;
}
fn eval(state: State) -> f64 {
    return 0.0;
}
fn minimax(state: &State, min: f64, max: f64) -> State {
    let s = State {
        board: [['_' as char; 8] ; 8],
        color: true,
        player: true,
    };
    return s;
}
fn main() {
    loop {
        println!("Hello! Please enter 1 or 2 to determine which color you will play, or enter 0 for a link to the rules.");
        let mut state = State {
            board: [['_' as char; 8] ; 8],
            color: true,
            player: true,
        };
        loop {
            let mut inp = String::new();
            io::stdin().read_line(&mut inp).expect("Failed to read input.");
            let inp: u8 = inp.trim().parse().expect("Not a number!");
            if inp == 0 {
                println!("You can find the rules at https://www.wikihow.com/Play-Checkers.");
                println!("Please enter 1 or 2.");
            }
            else if inp < 3 {
                let rnum: u8 = rand::thread_rng().gen_range(1..=2);
                if rnum == inp {
                    println!("You are playing black.");
                    state.player = false;
                    break;
                }
                println!("You are playing white.");
                break;
            }
            else {
                println!("This input is out of range! Please enter 0, 1, or 2.");
            }
        }
        for i in 0..3 {
            for j in 0..8 {
                if j % 2 == 1 {
                    state.board[i][j] = 'W';
                }
            }
        }
        for i in 5..8 {
            for j in 0..8 {
                if j % 2 == 0 {
                    state.board[i][j] = 'B';
                }
            }
        }
        for i in 0..8 {
            for j in 0..8 {
                print!("{} ", state.board[i][j]);
            }
            println!();
        }
        println!("_ : empty space");
        println!("B : uncrowned black checker");
        println!("C: crowned black checker");
        println!("W : uncrowned white checker");
        println!("X : crowned white checker");
        loop {
            let end: i8 = terminal(&state);
            if end == 1 {
                println!("Black won!");
                break;
            }
            else if end == -1 {
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