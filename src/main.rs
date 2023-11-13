use std::{
    char,
    fmt::{self, Display},
};

fn main() {
    let mut computer_grid = Grid::new();
    let mut user_grid = Grid::new();

    place_user_ship(&mut user_grid, 1, 'E', 5, 'E');
    place_user_ship(&mut user_grid, 7, 'A', 7, 'B');

    place_computer_ship(&mut computer_grid, 1, 'E', 5, 'E');
    place_computer_ship(&mut computer_grid, 7, 'A', 7, 'B');

    display_game(&computer_grid, &user_grid);

    println!("user ships:{:?}", user_grid.ships);
    println!("computer ships:{:?}", computer_grid.ships);

    guess(&mut computer_grid, 2, 'E');
    guess(&mut computer_grid, 2, 'F');

    display_game(&computer_grid, &user_grid);
}

#[derive(Clone, Debug)]
struct Grid {
    grid: Vec<Vec<Coordinate>>,
    ships: Vec<Vec<(usize, usize)>>,
}

impl Grid {
    fn new() -> Self {
        let mut new_grid: Vec<Vec<Coordinate>> = vec![];
        let mut new_row: Vec<Coordinate> = vec![];

        for _i in 1..=10 {
            for _j in 1..=10 {
                new_row.push(Coordinate {
                    state: (CoordState::Blank),
                })
            }
            new_grid.push(new_row.clone());
            new_row = vec![];
        }
        Self {
            grid: (new_grid),
            ships: vec![],
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "   A    B    C    D    E    F    G    H    I    J").unwrap();
        let mut line_num = 1;
        for row in &self.grid {
            if line_num == 10 {
                write!(f, "{line_num} ").unwrap();
            } else {
                write!(f, "{line_num}  ").unwrap();
            }
            for space in row {
                write!(f, "{space}   ").unwrap();
            }
            writeln!(f, "").unwrap();
            line_num += 1;
        }
        writeln!(f, "").unwrap();
        writeln!(f, "")
    }
}

fn display_game(computer: &Grid, user: &Grid) {
    println!("Computer: ");
    println!("{}", computer);
    println!("You: ");
    println!("{}", user);
}

#[derive(Copy, Clone, Debug)]
pub enum CoordState {
    Blank,
    Hit,
    Miss,
    Ship,
}

#[derive(Debug, Copy, Clone)]
pub struct Coordinate {
    state: CoordState,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.state {
            CoordState::Blank => write!(f, "🟦"),
            CoordState::Hit => write!(f, "🟥"),
            CoordState::Miss => write!(f, "🟩"),
            CoordState::Ship => write!(f, "🟨"),
        }
    }
}

fn get_coord_index(row: i32, column: char) -> (usize, usize) {
    let columns: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
    let rows: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let c_idx = columns.iter().position(|&r| r == column).unwrap();
    let r_idx = rows.iter().position(|&r| r == row).unwrap();

    (r_idx, c_idx)
}

fn update_coord_state(g: &mut Grid, row: i32, column: char, new_state: CoordState) {
    // implement these for Grid
    // add error checking
    let (r_idx, c_idx) = get_coord_index(row, column);

    g.grid[r_idx][c_idx].state = new_state;
}

fn place_user_ship(g: &mut Grid, start_row: i32, start_col: char, end_row: i32, end_col: char) {
    // needs to return a result.
    // implement this for Grid. Also, save ship coordinates to a vector w/ in grid
    // error checking
    let columns: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];

    let mut ship: Vec<(usize, usize)> = vec![];
    if start_row != end_row {
        let (_, c_start_idx) = get_coord_index(1, start_col);
        for i in start_row..=end_row {
            ship.push((i as usize, c_start_idx));
            update_coord_state(g, i, start_col, CoordState::Ship);
        }
    } else {
        let (_, c_start_idx) = get_coord_index(1, start_col);
        let (_, c_end_idx) = get_coord_index(1, end_col);

        for i in c_start_idx..=c_end_idx {
            ship.push((start_row as usize, i));
            update_coord_state(g, start_row, columns[i], CoordState::Ship);
        }
    }
    g.ships.push(ship);
}

fn place_computer_ship(g: &mut Grid, start_row: i32, start_col: char, end_row: i32, end_col: char) {
    // needs to return a result.
    // implement this for Grid. Also, save ship coordinates to a vector w/ in grid
    // error checking
    let mut ship: Vec<(usize, usize)> = vec![];
    if start_row != end_row {
        let (_, c_start_idx) = get_coord_index(1, start_col);
        for i in start_row..=end_row {
            ship.push((i as usize, c_start_idx));
        }
    } else {
        let (_, c_start_idx) = get_coord_index(1, start_col);
        let (_, c_end_idx) = get_coord_index(1, end_col);

        for i in c_start_idx..=c_end_idx {
            ship.push((start_row as usize, i));
        }
    }
    g.ships.push(ship);
}

fn create_computer_ship() {}

fn request_user_guess() {}

fn guess(g: &mut Grid, row: i32, column: char) {
    let rc = get_coord_index(row, column);

    if g.ships.iter().flatten().any(|&i| i == rc) {
        update_coord_state(g, row, column, CoordState::Hit);
        println!("Hit!");
    } else {
        update_coord_state(g, row, column, CoordState::Miss);
        println!("Miss");
    }
}

fn make_computer_guess() {}

pub enum ShipState {
    Floating,
    Sunk,
}
pub struct Ship {
    coordinates: Vec<Coordinate>,
    state: ShipState,
}

pub struct ComputerShips {
    carrier: Ship,
    battleship: Ship,
    cruiser: Ship,
    submarine: Ship,
    destroyer: Ship,
}

pub struct PlayerShips {
    carrier: Ship,
    battleship: Ship,
    cruiser: Ship,
    submarine: Ship,
    destroyer: Ship,
}
