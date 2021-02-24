use std::io;

// prints the grid
fn print_grid(arr: &[[char; 3]; 3]) {
    for i in arr.iter() {
        for j in i.iter() {
            print!("{} ", j);
        }
        print!{"\n\n"};
    }
}

// manipulates the grid
fn manip_grid(arr: &mut [[char; 3]; 3], play: &mut char) {
    let mut buf = String::new();

    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read from stdin!");
    
    // found this on stackoverflow
    let com: Vec<usize> = buf
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();

    // feel like this could be cleaner but oh well
    if com.is_empty() || com.len() < 2 {
        println!("Invalid input!");
    }
    else if com[0] > 2 || com[1] > 2 {
        println!("Out of bounds!");
    }
    else if arr[com[0]][com[1]] == '_' {
        arr[com[0]][com[1]] = *play;
        if *play == 'X' {
            *play = 'O';
        }
        else {
            *play = 'X';
        }
    }
    else {
        println!("Cell occupied!");
    }
}

// check for a winner
fn check_grid(arr: &[[char; 3]; 3]) -> char {
    let mut out : char = '_';

    // check rows & columns
    for i in 0..3 {
        if arr[i][0] == '_' && arr[i][1] == '_' && arr[i][2] == '_' {}
        else {
            if (arr[i][0] == arr[i][1]) && (arr[i][1] == arr[i][2]) {
                out = arr[i][0];
                break;
            }
        }
    }
    for i in 0..3 {
        if arr[0][i] == '_' && arr[0][i] == '_' && arr[0][i] == '_' {}
        else {
            if (arr[0][i] == arr[1][i]) && (arr[1][i] == arr[2][i]) {
                out = arr[0][i];
                break;
            }
        }
    }
    
    // check diagonals
    if (arr[0][0] == arr[1][1]) && (arr[1][1] == arr[2][2]) {
        out = arr[0][0];
    }
    else if (arr[0][2] == arr[1][1]) && (arr[1][1] == arr[2][0]) {
        out = arr[0][2];
    }

    // check for no more cells
    if  arr[0][0] != '_' && arr[0][1] != '_' && arr[0][2] != '_' &&
        arr[1][0] != '_' && arr[1][1] != '_' && arr[1][2] != '_' &&
        arr[2][0] != '_' && arr[2][1] != '_' && arr[2][2] != '_' {
            out = 'T';
    }

    out
}

// main game function
fn play_game() {
    let mut grid: [[char; 3]; 3] = [
        ['_', '_', '_'],
        ['_', '_', '_'],
        ['_', '_', '_']
    ];
    let mut player = 'X';
    let mut winner = '_';
    
    while winner == '_' {
        print_grid(&grid);
        println!("{}'s turn: ", player);
        manip_grid(&mut grid, &mut player);
        winner = check_grid(&grid);
    }

    if winner == 'T' {
        println!("Tie!");
    }
    else {
        println!("{} wins!", winner);
    }
}

fn main() {
    println!("Welcome to Simple Tic Tac Toe.");
    println!("Input \"y, x\" to select a cell.");
    println!("Start from top-left corner. Values from 0 -> 2.");
    
    // for the "replayability"
    loop {
        let mut play = false;
        play_game();

        println!("Play again? (y/N)");

        loop {
            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read from stdin!");
        
            match buf.trim() {
                "y" => {play = true; break;},
                "Y" => {play = true; break;},
                "n" => {break;},
                "N" => {break;},
                _ => {println!("Invalid input!")}
            }
        }

        if play == false {
            break;
        }
    }

    println!("Thanks for playing!");
}