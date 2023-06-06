use std::io;
use rand::Rng;

struct State {
    board: [[char; 8] ; 8],
    player: bool,
}
fn terminal(state: State) -> i8 {
    return 0;
}
fn children(state: State) -> Vec<State> {
    let vec: Vec<State> = Vec::new();
    return vec;
}
fn minimax(state: State, min: i8, max: i8) -> State {
    return state;
}
fn main() {
    loop {
        println!("Hello! Please enter 1 or 2 to determine which color you will play, or enter 0 for a link to the rules.");
        let mut state = State {
            board: [['_' as char; 8] ; 8],
            player: true, //true for black, false for white
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
            let end: i8 = terminal(state);
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
        println!("Enter 0 to play again or 1 to quit");
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