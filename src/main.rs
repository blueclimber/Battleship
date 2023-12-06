use crate::gamelogic::game_loop;
pub mod coord;
pub mod gamelogic;
pub mod grid;
pub mod possible_guess;

fn main() {
    game_loop();
}
