use crate::coord::*;
mod coord;
use crate::grid::*;
mod grid;
use crate::possible_guess::*;
mod possible_guess;
use std::io;

fn main() {
    game_loop();
}

fn display_game(computer: &Grid, user: &Grid) {
    println!("Computer: ");
    println!("{}", computer);
    println!("You: ");
    println!("{}", user);
}

fn game_loop() {
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
