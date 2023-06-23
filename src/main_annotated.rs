use std::io;
use rand::Rng;

//! Clone is part of the rust prelude, does not need to be imported to be derived onto a struct
use std::clone::Clone;

#[derive(Clone)]
struct State {
    //! Consider using a struct to represent the board so that you can have more abstraction over board actions.
    //! Furthermore, use a specific type to represent the pieces themselves. A char can be any unicode character.
    //! This creates the opportunity for bugs in the future if you decide to change the way you implement logic 
    //! in your code. Using a specific type like an enum would also make the code easier to read.
    board: [[char; 8]; 8],
    color: bool, //black = true. white = false
    player: bool, //computer = true, user = false
}

//! This should be a method of the State struct
//! Why does this function return an i8? An i8 is a signed 
//! integer that can represent anything, the more specific your 
//! function signature is the less error prone it will be
fn terminal(state: State) -> (i8, State) {
    //! Take a pointer to the state instead of returning the state.
    //! This is a common rust beginner pattern but it involves unnecessary calls to copy, the struct is >1026 bytes of data so it gets expensive fast
    let mut b: bool = false;
    let mut w: bool = false;
    //! I and J are not very descriptive variable names, consider using row/col or x/y instead
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
        return (-1, state);
    }
    else if !w {
        return (1, state);
    }
    //! Use todo sparingly, it is not something to be put all over a repo, this is a complete implementation...
    todo!("Add terminal if a player has no more moves");
    return (0, state);
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
    if col > 1 && (state.board[row - 1][col - 1] == 'W' || state.board[row - 1][col - 1] == 'X') && state.board[row - 2][col - 2] == '_' {
        let piece: char = state.board[row - 1][col - 1];
        state.board[row][col] = '_';
        state.board[row - 1][col - 1] = '_';
        state.board[row - 2][col - 2] = 'B';
        //! Let the compiler infer types, you don't need to explicitly write it out every time you declare a variable
        let tup: (Vec<State>, State, usize, usize) = dfs_b(state, row - 2, col - 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 + 2;
        col = tup.3 + 2;
        state.board[row][col] = 'B';
        state.board[row - 1][col - 1] = piece;
        state.board[row - 2][col - 2] = '_';
    }
    if col < 6 && (state.board[row - 1][col + 1] == 'W' || state.board[row - 1][col + 1] == 'X') && state.board[row - 2][col + 2] == '_' {
        let piece: char = state.board[row - 1][col + 1];
        state.board[row][col] = '_';
        state.board[row - 1][col + 1] = '_';
        state.board[row - 2][col + 2] = 'B';
        let tup: (Vec<State>, State, usize, usize) = dfs_b(state, row - 2, col + 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 + 2;
        col = tup.3 + 2;
        state.board[row][col] = 'B';
        state.board[row - 1][col + 1] = piece;
        state.board[row - 2][col + 2] = '_';
    }
    todo!("Return only final moves");
    return (vec, state, row, col);
}
fn dfs_c(state: State, row: usize, col: usize) -> (Vec<State>, State, usize, usize) {
    let mut vec: Vec<State> = Vec::new();
    let mut state: State = state;
    let mut row: usize = row;
    let mut col: usize = col;
    let copy = state.clone();
    vec.push(copy);
    if col > 1 && (state.board[row - 1][col - 1] == 'W' || state.board[row - 1][col - 1] == 'X') && state.board[row - 2][col - 2] == '_' {
        let piece: char = state.board[row - 1][col - 1];
        state.board[row][col] = '_';
        state.board[row - 1][col - 1] = '_';
        state.board[row - 2][col - 2] = 'C';
        let tup: (Vec<State>, State, usize, usize) = dfs_c(state, row - 2, col - 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 + 2;
        col = tup.3 + 2;
        state.board[row][col] = 'C';
        state.board[row - 1][col - 1] = piece;
        state.board[row - 2][col - 2] = '_';
    }
    if col < 6 && (state.board[row - 1][col + 1] == 'W' || state.board[row - 1][col + 1] == 'X') && state.board[row - 2][col + 2] == '_' {
        let piece: char = state.board[row - 1][col + 1];
        state.board[row][col] = '_';
        state.board[row - 1][col + 1] = '_';
        state.board[row - 2][col + 2] = 'C';
        let tup: (Vec<State>, State, usize, usize) = dfs_c(state, row - 2, col + 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 + 2;
        col = tup.3 - 2;
        state.board[row][col] = 'C';
        state.board[row - 1][col + 1] = piece;
        state.board[row - 2][col + 2] = '_';
    }
    if col > 1 && (state.board[row + 1][col - 1] == 'W' || state.board[row + 1][col - 1] == 'X') && state.board[row + 2][col - 2] == '_' {
        let piece: char = state.board[row + 1][col - 1];
        state.board[row][col] = '_';
        state.board[row + 1][col - 1] = '_';
        state.board[row + 2][col - 2] = 'C';
        let tup: (Vec<State>, State, usize, usize) = dfs_c(state, row + 2, col - 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 - 2;
        col = tup.3 + 2;
        state.board[row][col] = 'C';
        state.board[row + 1][col - 1] = piece;
        state.board[row + 2][col - 2] = '_';
    }
    if col < 6 && (state.board[row + 1][col + 1] == 'W' || state.board[row + 1][col + 1] == 'X') && state.board[row + 2][col + 2] == '_' {
        let piece: char = state.board[row + 1][col + 1];
        state.board[row][col] = '_';
        state.board[row + 1][col + 1] = '_';
        state.board[row + 2][col + 2] = 'C';
        let tup: (Vec<State>, State, usize, usize) = dfs_c(state, row + 2, col + 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 - 2;
        col = tup.3 - 2;
        state.board[row][col] = 'C';
        state.board[row + 1][col + 1] = piece;
        state.board[row + 2][col + 2] = '_';
    }
    todo!("Return only final moves");
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
    if col > 1 && (state.board[row + 1][col - 1] == 'B' || state.board[row + 1][col - 1] == 'C') && state.board[row + 2][col - 2] == '_' {
        let piece: char = state.board[row + 1][col - 1];
        state.board[row][col] = '_';
        state.board[row + 1][col - 1] = '_';
        state.board[row + 2][col - 2] = 'W';
        let tup: (Vec<State>, State, usize, usize) = dfs_w(state, row + 2, col - 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 - 2;
        col = tup.3 + 2;
        state.board[row][col] = 'W';
        state.board[row + 1][col - 1] = piece;
        state.board[row + 2][col - 2] = '_';
    }
    if col < 6 && (state.board[row + 1][col + 1] == 'B' || state.board[row + 1][col + 1] == 'C') && state.board[row + 2][col + 2] == '_' {
        let piece: char = state.board[row + 1][col + 1];
        state.board[row][col] = '_';
        state.board[row + 1][col + 1] = '_';
        state.board[row + 2][col + 2] = 'W';
        let tup: (Vec<State>, State, usize, usize) = dfs_w(state, row + 2, col + 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 - 2;
        col = tup.3 - 2;
        state.board[row][col] = 'W';
        state.board[row + 1][col + 1] = piece;
        state.board[row + 2][col + 2] = '_';
    }
    todo!("Return only final moves");
    return (vec, state, row, col);
}
fn dfs_x(state: State, row: usize, col: usize) -> (Vec<State>, State, usize, usize) {
    let mut vec: Vec<State> = Vec::new();
    let mut state: State = state;
    let mut row: usize = row;
    let mut col: usize = col;
    let copy = state.clone();
    vec.push(copy);
    if col > 1 && (state.board[row - 1][col - 1] == 'B' || state.board[row - 1][col - 1] == 'C') && state.board[row - 2][col - 2] == '_' {
        let piece: char = state.board[row - 1][col - 1];
        state.board[row][col] = '_';
        state.board[row - 1][col - 1] = '_';
        state.board[row - 2][col - 2] = 'X';
        let tup: (Vec<State>, State, usize, usize) = dfs_x(state, row - 2, col - 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 + 2;
        col = tup.3 + 2;
        state.board[row][col] = 'X';
        state.board[row - 1][col - 1] = piece;
        state.board[row - 2][col - 2] = '_';
    }
    if col < 6 && (state.board[row - 1][col + 1] == 'B' || state.board[row - 1][col + 1] == 'C') && state.board[row - 2][col + 2] == '_' {
        let piece: char = state.board[row - 1][col + 1];
        state.board[row][col] = '_';
        state.board[row - 1][col + 1] = '_';
        state.board[row - 2][col + 2] = 'X';
        let tup: (Vec<State>, State, usize, usize) = dfs_x(state, row - 2, col + 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 + 2;
        col = tup.3 - 2;
        state.board[row][col] = 'X';
        state.board[row - 1][col + 1] = piece;
        state.board[row - 2][col + 2] = '_';
    }
    if col > 1 && (state.board[row + 1][col - 1] == 'B' || state.board[row + 1][col - 1] == 'C') && state.board[row + 2][col - 2] == '_' {
        let piece: char = state.board[row + 1][col - 1];
        state.board[row][col] = '_';
        state.board[row + 1][col - 1] = '_';
        state.board[row + 2][col - 2] = 'X';
        let tup: (Vec<State>, State, usize, usize) = dfs_x(state, row + 2, col - 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 - 2;
        col = tup.3 + 2;
        state.board[row][col] = 'X';
        state.board[row + 1][col - 1] = piece;
        state.board[row + 2][col - 2] = '_';
    }
    if col < 6 && (state.board[row + 1][col + 1] == 'B' || state.board[row + 1][col + 1] == 'C') && state.board[row + 2][col + 2] == '_' {
        let piece: char = state.board[row + 1][col + 1];
        state.board[row][col] = '_';
        state.board[row + 1][col + 1] = '_';
        state.board[row + 2][col + 2] = 'X';
        let tup: (Vec<State>, State, usize, usize) = dfs_x(state, row + 2, col + 2);
        for i in tup.0 {
            vec.push(i);
        }
        state = tup.1;
        row = tup.2 - 2;
        col = tup.3 - 2;
        state.board[row][col] = 'X';
        state.board[row + 1][col + 1] = piece;
        state.board[row + 2][col + 2] = '_';
    }
    todo!("Return only final moves");
    return (vec, state, row, col);
}
fn children(state: State, row: usize, col: usize) -> (Vec<State>, State) {
    let mut vec: Vec<State> = Vec::new();
    let mut state: State = state;
    let mut row: usize = row;
    let mut col: usize = col;
    if state.color {
        for i in 0..8 {
            for j in 0..8 {
                if state.board[i][j] == 'B' {
                    if i > 1 {
                        let tup: (Vec<State>, State, usize, usize) = dfs_b(state, i, j);
                        for i in tup.0 {
                            vec.push(i);
                        }
                        state = tup.1;
                    }
                }
                else if state.board[i][j] == 'C' {

                }
            }
        }
    }
    else {

    }
    todo!();
    return (vec, state);
}
fn eval(state: State) -> f64 {
    todo!();
    return 0.0;
}
fn minimax(state: State, min: f64, max: f64) -> State {
    let s = State {
        board: [['_' as char; 8] ; 8],
        color: true,
        player: true,
    };
    todo!();
    return s;
}
fn main() {
    loop {
        //! What is the point of making the player chose if the selection is random anyway, 
        //! from a statistical point of view the players choice has no affect on the outcome. 
        //! (correct me if I am wrong)
        println!("Hello! Please enter 1 or 2 to determine which color you will play, or enter 0 for a link to the rules.");
        //! Use a constructor here, though there isn't an official constructor in Rust
        //! Self::new() is commonly accepted to be the constructor
        let mut state = State {
            board: [['_' as char; 8] ; 8],
            color: true,
            player: true,
        };
        //! Loop inside loop? Is there a better way to do this?
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
            let tup: (i8, State) = terminal(state);
            let end: i8 = tup.0;
            state = tup.1;
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