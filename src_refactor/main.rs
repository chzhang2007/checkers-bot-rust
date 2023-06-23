
mod board;
mod state;
mod minimax;

use state::State;

fn main() {
    let mut state = State::new();
    println!("You can find the rules at https://www.wikihow.com/Play-Checkers.");
    println!("_ : empty space");
    println!("b : uncrowned black checker"); // There are very good libraries to color printed text in rust
    println!("B : crowned black checker");
    println!("w : uncrowned white checker");
    println!("W : crowned white checker");
    println!("{}", state.board);

    'running: loop {
        match state.is_terminal() {
            Some(color) => {
                if color { println!("White Won!") } 
                else { println!("White Won!") }
                break 'running
            },
            _ => {}
        }
        /* Gameplay logic */
    }
}