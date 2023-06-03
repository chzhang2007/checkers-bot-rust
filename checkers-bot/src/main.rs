use std::io;
use rand::Rng;

fn main() {
    println!("Hello! Please enter 1 or 2 to determine which color you will play, or enter 0 for a link to the rules.");
    let mut color: bool = true; //true for black, false for white
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
                color = false;
                break;
            }
            println!("You are playing white.");
            break;
        }
        else {
            println!("This input is out of range! Please enter 0, 1, or 2.");
        }
    }
    let mut board = [['_' as char; 8] ; 8];
    for i in 0..3 {
        for j in 0..8 {
            if j % 2 == 1 {
                board[i][j] = 'W';
            }
        }
    }
    for i in 5..8 {
        for j in 0..8 {
            if j % 2 == 0 {
                board[i][j] = 'B';
            }
        }
    }
}