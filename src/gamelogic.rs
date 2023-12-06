use crate::grid::Grid;
use crate::possible_guess::PossibleGuesses;

use std::io;

///
/// Displays the game by displaying each grid using their Display function
///
fn display_game(computer: &Grid, user: &Grid) {
    println!("Computer: ");
    println!("{}", computer);
    println!("You: ");
    println!("{}", user);
}

///
/// Driver for the whole program
/// Calls the game function and will loop the game function
/// until the user no longer wants a new game
///
pub fn game_loop() {
    loop {
        game();
        println!("Play again?\nPress 'n' for new game, any other key to quit.");
        let mut in_string = "".to_string();

        io::stdin()
            .read_line(&mut in_string)
            .expect("error: unable to read input");
        in_string = in_string.trim().to_string();
        if in_string != *"n" {
            break;
        }
    }
}

///
/// Sets up a new game and then loops the guesses until the game is finished
/// Creates a new computer_grid (the computer's gameboard), user_grid (the user's
/// game board) and guessing_points (a reference for the computer to guide it's
/// guesses).
///  
fn game() {
    let mut computer_grid = Grid::new();
    let mut user_grid = Grid::new();
    let mut guessing_points = PossibleGuesses::new();

    println!("What level of difficulty do you want? Press 'h' for hard and 'e' for easy");
    let mut diff = "".to_string();
    io::stdin()
        .read_line(&mut diff)
        .expect("error: unable to read input");
    let diff = diff.trim().to_string();
    if diff == *"h" {
        guessing_points.difficult = true;
    }

    computer_grid.set_computer_ships();

    user_grid.request_user_ships();

    loop {
        display_game(&computer_grid, &user_grid);
        let finished = computer_grid.request_user_guess();
        if finished {
            println!("Congratulations! You won!");
            break;
        }
        let finished = user_grid.make_computer_guess(&mut guessing_points);
        if finished {
            println!("Too bad, you lost. Try again?");
            break;
        }
    }
}
