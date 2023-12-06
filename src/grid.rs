use crate::coord::*;
use crate::possible_guess::*;
use rand::random;
use std::{
    char,
    collections::HashMap,
    fmt::{self, Display},
};

///
/// This holds the information about the game board and game pieces.
/// The grid is the game board, the ships are the ships.
///
#[derive(Clone, Debug)]
pub struct Grid {
    pub grid: Vec<Vec<Coordinate>>,
    pub ships: HashMap<String, Vec<(usize, usize)>>,
}

impl Grid {
    ///
    /// We set up the board with a two layer 10 X 10 vector of coordinates
    /// Coordinate is a struct in the coord file
    ///
    /// The ships are set up as an empty hashmap that will be added to as
    /// ships are legally placed.
    ///
    pub fn new() -> Self {
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
            ships: HashMap::new(),
        }
    }

    ///
    /// Coordinates have 4 states -
    /// Arguements:
    ///     row: i32 - between 1-10 - the row of the coordinate that will change
    ///     column: char - between A - J - the column of the coordinate that will change
    /// blank, ship, hit, and miss
    ///
    fn update_coord_state(&mut self, row: i32, column: char, new_state: CoordState) {
        let (r_idx, c_idx) = get_coord_index(row, column);

        self.grid[r_idx][c_idx].state = new_state;
    }

    ///
    /// Driver to place all of the user's ships
    /// Set up to get the user input for ship placement.
    /// Create a vec of possible ships points to make sure that ships are not placed on
    /// top of each other.
    /// Create a hashmap with ship names and their length
    /// Pull each ship from the hashmap, and call self.request_ship on it
    /// Print the user board before beginning as a reference, and after each ship placement
    /// no return value
    ///
    pub fn request_user_ships(&mut self) {
        let mut points = vec![];
        for i in 0..10 {
            for j in 0..10 {
                points.push((i as usize, j as usize));
            }
        }

        let mut possible_ships: HashMap<String, u32> = HashMap::new();
        possible_ships.insert("Carrier".to_string(), 5);
        possible_ships.insert("Battleship".to_string(), 4);
        possible_ships.insert("Cruiser".to_string(), 3);
        possible_ships.insert("Submarine".to_string(), 3);
        possible_ships.insert("Destroyer".to_string(), 2);

        println!("Player board:\n{self}");

        for (ship, length) in possible_ships {
            self.request_ship(ship, length, &mut points);
            println!("Player board:\n{self}");
        }
    }

    ///
    /// Asks for user imput for ship placement
    /// Arguements:
    ///     name: String - this is the name of the ship and how it will be saved
    ///     size: u32 - this is the length of the ship
    ///     points: &mut Vec<(usize, usize)> This holds all of the unused coordinates
    ///         in the user game board for validating ship placement
    /// calls get_input_coord to get valid coordinates
    /// checks that ship placement is valid
    /// if it is calls place_user_ship to check for more validity, and record the ship
    /// Loops until a valid ship is placed successfully.
    /// no return value
    ///
    fn request_ship(&mut self, name: String, size: u32, points: &mut Vec<(usize, usize)>) {
        let mut valid_ship = false;

        println!("Where would you like to place your {name}? It is {size} spaces long.",);
        while !valid_ship {
            println!("Please enter the starting coordinate");
            let mut start_row = 0;
            let mut start_col = 'z';

            let mut valid_start = false;
            while !valid_start {
                ((start_row, start_col), valid_start) = get_input_coord();
                if !valid_start {
                    println!("Please enter location in the format: A8")
                }
            }

            println!("Please enter the ending coordinate");
            let mut end_row = 0;
            let mut end_col = 'z';
            let mut valid_end = false;

            while !valid_end {
                ((end_row, end_col), valid_end) = get_input_coord();
                if !valid_end {
                    println!("Please enter location in the format: A8")
                }
            }

            valid_ship = self.place_user_ship(
                name.clone(),
                (start_row, start_col),
                (end_row, end_col),
                size,
                points,
            );
            if !valid_ship {
                println!("Invalid ship placement. Please try again.")
            }
        }
    }

    ///
    /// Arguements:
    ///     name: Sring - the name of the ship to be places
    ///     start_point: (i32, char) - the coordinate of one end of the ship
    ///     end_point: (i32, char) - the coordinate of the other end of the ship
    ///     given_size: u32 - the expected size of the ship, for validation
    ///     points: &mut Vec<(usize, usize)> This holds all of the unused coordinates
    ///         in the user game board for validating ship placement. If the placement is valid
    ///         all of the coordinates being used will be deleted from points.
    /// Returns bool - whether or not the ship placement was valid
    ///     
    fn place_user_ship(
        &mut self,
        name: String,
        start_point: (i32, char),
        end_point: (i32, char),
        given_size: u32,
        points: &mut Vec<(usize, usize)>,
    ) -> bool {
        let columns: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
        let mut valid = true;
        let size = given_size - 1;
        let (start_row, start_col) = start_point;
        let (end_row, end_col) = end_point;

        let mut ship: Vec<(usize, usize)> = vec![];

        if start_row != end_row && start_col != end_col {
            println!("Your ship must go in a straight line");
            valid = false;
        } else if start_row == end_row && start_col == end_col {
            println!("Your ship must take up more than one space");
            valid = false;
        }

        if valid {
            if start_row != end_row {
                let (_, c_start_idx) = get_coord_index(start_row, start_col);
                if start_row < end_row {
                    if end_row - start_row != size as i32 {
                        println!("Your ship must be {given_size} spaces long");
                        valid = false
                    }
                    for i in start_row..=end_row {
                        let point = (i as usize - 1, c_start_idx);
                        if !points.contains(&point) {
                            println!("You can not place a ship on top of another ship.");
                            valid = false
                        }
                        ship.push(point);
                    }
                } else {
                    if start_row - end_row != size as i32 {
                        println!("Your ship must be {given_size} spaces long");
                        valid = false;
                    }
                    for i in end_row..=start_row {
                        let point = (i as usize - 1, c_start_idx);
                        if !points.contains(&point) {
                            println!("You can not place a ship on top of another ship.");
                            valid = false
                        }
                        ship.push(point);
                    }
                }
            } else {
                let (_, c_start_idx) = get_coord_index(1, start_col);
                let (_, c_end_idx) = get_coord_index(1, end_col);

                if c_start_idx < c_end_idx {
                    if c_end_idx - c_start_idx != size as usize {
                        println!("Your ship must be {given_size} spaces long");
                        valid = false;
                    }
                    for i in c_start_idx..=c_end_idx {
                        let point = ((start_row - 1) as usize, i);
                        if !points.contains(&point) {
                            println!("You can not place a ship on top of another ship.");
                            valid = false
                        }
                        ship.push(point);
                    }
                } else {
                    if c_start_idx - c_end_idx != size as usize {
                        println!("Your ship must be {given_size} spaces long");
                        valid = false
                    }
                    for i in c_end_idx..=c_start_idx {
                        let point = ((start_row - 1) as usize, i);
                        if !points.contains(&point) {
                            println!("You can not place a ship on top of another ship.");
                            valid = false
                        }
                        ship.push(point);
                    }
                }
            }
        }

        if valid {
            for (row, col) in &ship {
                self.update_coord_state(*row as i32 + 1, columns[*col], CoordState::Ship);
                if let Some(pos) = points.iter().position(|x| *x == (*row, *col)) {
                    points.remove(pos);
                }
            }
            self.ships.insert(name.clone(), ship);
        }
        valid
    }

    ///
    /// Driver for setting computer ships
    /// Creates a hashmap of 4 biggest ships
    /// Creates vector of points for placement validation
    /// creates vector of quadrants to aid in legal ship placement
    /// randomly selects ship and quadrant, then calls create_ship() which creates a legal ship
    /// Once the 4 biggest ships are placed, create_ship is called for the smallest ship and a random quadrant
    /// no return value
    ///
    pub fn set_computer_ships(&mut self) {
        let mut possible_ships: HashMap<String, u32> = HashMap::new();
        possible_ships.insert("Carrier".to_string(), 5);
        possible_ships.insert("Battleship".to_string(), 4);
        possible_ships.insert("Submarine".to_string(), 3);
        possible_ships.insert("Cruiser".to_string(), 3);

        let mut points = vec![];
        for i in 0..10 {
            for j in 0..10 {
                points.push((i, j));
            }
        }

        let mut quadrants = vec![0, 1, 2, 3];

        for ship in possible_ships {
            let q_idx = (random::<f32>() * quadrants.len() as f32).floor() as usize;
            let quadrant = quadrants.remove(q_idx);
            self.create_ship(ship, &mut points, quadrant);
        }

        let quad = (random::<f32>() * 4.).floor() as usize;
        self.create_ship(("Destroyer".to_string(), 2), &mut points, quad)
    }

    ///
    /// Arguements:
    ///     ship_info: (String, u32) - Name and size of the ship to be created
    ///     points: &mut Vec<(i32, i32)> - a vector of available coordinates for legal ship placement
    ///     quadrant: usize, between 0-3, 0 being upper left quarter, 1 is upper right, 2 is
    ///         lower left, 3 is lower right. The starting point of the ship will be within the given (random)
    ///         quadrant, to stop potential hanging loops as a result of not having an valid places to put the ships.
    /// No return value
    /// Matches on the quadrant, randomly chooses a start coordinate that is in the points vector, and tries to place a ship
    ///     going away from the corner.
    ///     For example, in quadrant 0, the starting location for the ship will be in row 1-5 and column A-E. Then the ending
    ///         location will randomly either be to the right or directly below the start location.
    /// The functions loops until a valid placement has been made.
    /// Once a valid placement has been found, the ship is pushed onto the grid.ships HashMap
    ///
    fn create_ship(
        &mut self,
        ship_info: (String, u32),
        points: &mut Vec<(i32, i32)>,
        quadrant: usize,
    ) {
        let (name, length) = ship_info;

        let mut ship: Vec<(usize, usize)> = vec![];
        let mut valid_ship = false;
        let mut row = 0;
        let mut col = 0;

        match quadrant {
            0 => {
                while !valid_ship {
                    let mut valid_start = false;
                    while !valid_start {
                        row = (random::<f32>() * 5.).floor() as i32;
                        col = (random::<f32>() * 5.).floor() as i32;
                        if points.contains(&(row, col)) {
                            valid_start = true
                        }
                    }
                    let direction = (random::<f32>() * 2.).floor() as i32;
                    match direction {
                        0 => {
                            let mut v = true;
                            for i in 0..length as i32 {
                                let point = (row + i, col);
                                if !points.contains(&point) {
                                    v = false;
                                    ship = vec![];
                                    break;
                                } else {
                                    ship.push((point.0 as usize, point.1 as usize))
                                }
                            }
                            valid_ship = v
                        }
                        1 => {
                            let mut v = true;
                            for i in 0..length as i32 {
                                let point = (row, col + i);
                                if !points.contains(&point) {
                                    v = false;
                                    ship = vec![];
                                    break;
                                } else {
                                    ship.push((point.0 as usize, point.1 as usize))
                                }
                            }
                            valid_ship = v
                        }
                        _ => {}
                    }
                }
            }
            1 => {
                while !valid_ship {
                    let mut valid_start = false;
                    while !valid_start {
                        row = (random::<f32>() * 5.).floor() as i32;
                        col = (random::<f32>() * 5.).floor() as i32 + 5;
                        if points.contains(&(row, col)) {
                            valid_start = true
                        }
                    }
                    let direction = (random::<f32>() * 2.).floor() as i32;
                    match direction {
                        0 => {
                            let mut v = true;
                            for i in 0..length as i32 {
                                let point = (row + i, col);
                                if !points.contains(&point) {
                                    v = false;
                                    ship = vec![];
                                    break;
                                } else {
                                    ship.push((point.0 as usize, point.1 as usize))
                                }
                            }
                            valid_ship = v
                        }
                        1 => {
                            let mut v = true;
                            for i in 0..length as i32 {
                                let point = (row, col - i);
                                if !points.contains(&point) {
                                    v = false;
                                    ship = vec![];
                                    break;
                                } else {
                                    ship.push((point.0 as usize, point.1 as usize))
                                }
                            }
                            valid_ship = v
                        }
                        _ => {}
                    }
                }
            }

            2 => {
                while !valid_ship {
                    let mut valid_start = false;
                    while !valid_start {
                        row = (random::<f32>() * 5.).floor() as i32 + 5;
                        col = (random::<f32>() * 5.).floor() as i32;
                        if points.contains(&(row, col)) {
                            valid_start = true
                        }
                    }
                    let direction = (random::<f32>() * 2.).floor() as i32;
                    match direction {
                        0 => {
                            let mut v = true;
                            for i in 0..length as i32 {
                                let point = (row - i, col);
                                if !points.contains(&point) {
                                    v = false;
                                    ship = vec![];
                                    break;
                                } else {
                                    ship.push((point.0 as usize, point.1 as usize))
                                }
                            }
                            valid_ship = v
                        }
                        1 => {
                            let mut v = true;
                            for i in 0..length as i32 {
                                let point = (row, col + i);
                                if !points.contains(&point) {
                                    v = false;
                                    ship = vec![];
                                    break;
                                } else {
                                    ship.push((point.0 as usize, point.1 as usize))
                                }
                            }
                            valid_ship = v
                        }
                        _ => {}
                    }
                }
            }

            3 => {
                while !valid_ship {
                    let mut valid_start = false;
                    while !valid_start {
                        row = (random::<f32>() * 5.).floor() as i32 + 5;
                        col = (random::<f32>() * 5.).floor() as i32 + 5;
                        if points.contains(&(row, col)) {
                            valid_start = true
                        }
                    }
                    let direction = (random::<f32>() * 2.).floor() as i32;
                    match direction {
                        0 => {
                            let mut v = true;
                            for i in 0..length as i32 {
                                let point = (row - i, col);
                                if !points.contains(&point) {
                                    v = false;
                                    ship = vec![];
                                    break;
                                } else {
                                    ship.push((point.0 as usize, point.1 as usize))
                                }
                            }
                            valid_ship = v
                        }
                        1 => {
                            let mut v = true;
                            for i in 0..length as i32 {
                                let point = (row, col - i);
                                if !points.contains(&point) {
                                    v = false;
                                    ship = vec![];
                                    break;
                                } else {
                                    ship.push((point.0 as usize, point.1 as usize))
                                }
                            }
                            valid_ship = v
                        }
                        _ => {}
                    }
                }
            }
            _ => (),
        }
        for point in &ship {
            let p = (point.0 as i32, point.1 as i32);

            if let Some(pos) = points.iter().position(|x| *x == p) {
                points.remove(pos);
            }
        }
        self.ships.insert(name, ship);
    }

    ///
    /// Arguments:
    ///     possible_guesses: &mut PossibleGuesses
    ///         This keeps track of what points are available to guessed, and has
    ///         methods that assist with the AI guessing.
    /// Checks status of PossibleGuesses
    /// If possible_guesses.reasonable_guesses or possible_guesses.next_guesses aren't empty,
    /// there has been a hit recently, without sinking a ship.
    /// In that case non_random_guess is called, which will pull form either
    /// of those vectors.
    ///  
    /// If those two are empty, it calls random_guess
    ///
    /// Returns a bool reporting if the game is over. (True if it is over)
    ///
    pub fn make_computer_guess(&mut self, possible_guesses: &mut PossibleGuesses) -> bool {
        if possible_guesses.reasonable_guesses.is_empty()
            && possible_guesses.next_guesses.is_empty()
        {
            let (col, row) = possible_guesses.random_guess();
            self.computer_guess_helper(col, row, possible_guesses)
        } else {
            let (col, row) = possible_guesses.non_random_guess();
            self.computer_guess_helper(col, row, possible_guesses)
        }
    }

    ///
    /// Arguements:
    ///     col: char - between A-J, the column of the guess
    ///     row: i32 - between 1-10, the row of the guess
    ///     possible_guesses: &mut PossibleGuesses
    ///         This keeps track of what points are available to guessed, and has
    ///         methods that assist with the AI guessing.
    /// A helper function for making a guess
    /// Calls guess
    /// returns a bool, true if the game over, false otherwise
    ///
    fn computer_guess_helper(
        &mut self,
        col: char,
        row: i32,
        possible_guesses: &mut PossibleGuesses,
    ) -> bool {
        let mut finished = false;
        println!("Computer guess: {col}{row}");

        let report = self.guess(row, col);
        if report.0 == 'h' {
            possible_guesses.update_guesses(report, col, row)
        }
        if report.1 .0 == 'f' {
            finished = true;
        }
        finished
    }

    ///
    /// Promts the user to make a guess
    /// Calls get get_input_coord to get a valid coordinate from the user
    /// calls guess
    /// Returns true if the game is over
    ///
    pub fn request_user_guess(&mut self) -> bool {
        let mut finished = false;
        let mut valid_guess = false;

        let mut guess_row = 0;
        let mut guess_col = 'z';

        println!("Please make a guess");
        while !valid_guess {
            ((guess_row, guess_col), valid_guess) = get_input_coord();
            if !valid_guess {
                println!("Please make a guess in the format: A8");
            }
        }

        let (_, (fin, _)) = self.guess(guess_row, guess_col);
        if fin == 'f' {
            finished = true
        }
        finished
    }

    ///
    /// Makes a guess
    /// Arguments:
    ///     row: i32 - 1-10 - the row the guess is on
    ///     col: char - A-J - the column the guess is on
    /// Returns (h: char, (s: char, l:i32))
    ///     h: hit or miss
    ///     s: n for not hit, u for not sunk, s for sunk, f for finished
    ///     l: length of hit ship (for the computer smart guesser)
    ///
    /// First checks to see that the coordinate hasn't already been guess
    /// Then looks to see if there is a ship there
    ///     If there is not, returns a miss
    ///     If there is, calls check_ship_status
    ///         and changes that coordinate in the shap to (100, 100) to show it was hit
    ///
    fn guess(&mut self, row: i32, column: char) -> (char, (char, i32)) {
        // check if status of coordinate is not blank
        let rc = get_coord_index(row, column);

        let mut found = false;
        let mut found_ship = "".to_string();
        let mut idx: usize = 0;
        let mut g = 'm'; // for miss
        let mut s = ('n', 0); // for not sunk

        if self.grid[rc.0][rc.1].state == CoordState::Hit
            || self.grid[rc.0][rc.1].state == CoordState::Miss
        {
            println!("Already guessed: {column}{row}");
            return (g, s);
        }

        for (ship, coords) in &mut self.ships {
            for (i, coord) in coords.iter().enumerate() {
                if rc == *coord {
                    found = true;
                    found_ship = ship.to_string();
                    idx = i;
                    break;
                }
            }
        }

        if !found {
            self.update_coord_state(row, column, CoordState::Miss);
            println!("Miss");
        } else {
            let mut new_coords = self.ships.get(&found_ship).unwrap().clone();
            new_coords[idx] = (100, 100);
            self.ships.insert(found_ship.clone(), new_coords);
            self.update_coord_state(row, column, CoordState::Hit);
            println!("Hit!");
            g = 'h';
            s = self.check_ship_status(found_ship);
        }
        let report: (char, (char, i32)) = (g, s);
        report
    }

    ///
    /// Arguments:
    ///     hit_ship: String - the name of the ship that was just hit
    /// Returns (s: char, l: i32)
    ///     s: s for sunk, f for finished
    ///     l: 0 if not sunk, or the length of the ship that was just sunk
    /// checks to see if the ship that was just hit is now sunk
    ///     (by seeing if all of the coordinates for that ship == (100, 100))
    /// if it is, it removes the ship from grid.ships and calls check_endgame
    ///
    fn check_ship_status(&mut self, hit_ship: String) -> (char, i32) {
        let coords = self.ships.get(&hit_ship).unwrap();
        let mut s = 'u';
        let mut l = 0;

        let sunk = coords.iter().all(|&coord| coord == (100, 100));
        if sunk {
            s = 's'; // for sunk
            l = coords.len();
            println!("{hit_ship} sunk!");
            self.ships.remove(&hit_ship);
            let f = self.check_endgame();
            if f {
                s = 'f' // for finished
            }
        }
        (s, l as i32)
    }

    ///
    /// As ships are sunk, they are deleted from the grid.ships
    /// When grid.ships is empty, this function returns true,
    /// prints a game over message and
    /// triggers endgame.
    ///
    fn check_endgame(&mut self) -> bool {
        let mut finished = false;
        if self.ships.is_empty() {
            println!("\t\tGAME OVER\n\n\n");
            finished = true;
        }
        finished
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
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
            writeln!(f).unwrap();
            line_num += 1;
        }
        writeln!(f, "\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let mut grid1 = Grid::new();
        assert_eq!(grid1.grid[0][0].state, CoordState::Blank);
        assert!(grid1.ships.is_empty());

        grid1.update_coord_state(1, 'A', CoordState::Hit);
        assert_eq!(grid1.grid[0][0].state, CoordState::Hit);

        grid1.update_coord_state(10, 'J', CoordState::Miss);
        assert_eq!(grid1.grid[9][9].state, CoordState::Miss);
    }

    #[test]
    fn test_user_ship() {
        let mut grid1 = Grid::new();
        let mut points = vec![];
        for i in 0..10 {
            for j in 0..10 {
                points.push((i as usize, j as usize));
            }
        }
        let v1 =
            grid1.place_user_ship("Battleship".to_string(), (2, 'B'), (4, 'B'), 3, &mut points);
        assert!(v1);
        assert_eq!(grid1.grid[1][1].state, CoordState::Ship);
        assert_eq!(grid1.grid[2][1].state, CoordState::Ship);
        assert_eq!(grid1.grid[3][1].state, CoordState::Ship);
        assert_eq!(
            grid1.ships[&"Battleship".to_string()],
            vec![(1, 1), (2, 1), (3, 1)]
        );

        let v2 = grid1.place_user_ship("Destroyer".to_string(), (2, 'A'), (2, 'C'), 3, &mut points);
        assert!(!v2);
        assert_eq!(grid1.grid[1][0].state, CoordState::Blank);
    }

    #[test]
    fn test_computer_ship() {
        let mut grid2 = Grid::new();
        let mut points = vec![];
        for i in 0..10 {
            for j in 0..10 {
                points.push((i, j));
            }
        }
        let len1 = points.len();
        grid2.create_ship(("Battleship".to_string(), 3), &mut points, 0);
        assert_eq!(len1 - 3, points.len());
        assert!(grid2.ships.contains_key(&"Battleship".to_string()));
        let ship = grid2.ships[&"Battleship".to_string()].clone();
        for (row, col) in ship {
            assert_eq!(grid2.grid[row][col].state, CoordState::Blank);
        }
    }

    #[test]
    fn test_computer_guess() {
        let mut grid1 = Grid::new();
        let mut points = vec![];
        for i in 0..10 {
            for j in 0..10 {
                points.push((i as usize, j as usize));
            }
        }
        grid1.place_user_ship("Battleship".to_string(), (2, 'B'), (4, 'B'), 3, &mut points);
        let mut pg = PossibleGuesses::new();

        let f1 = grid1.computer_guess_helper('E', 6, &mut pg);
        assert_eq!(grid1.grid[1][0].state, CoordState::Blank);
        assert_eq!(grid1.grid[5][4].state, CoordState::Miss);
        assert!(!f1);

        let f2 = grid1.computer_guess_helper('B', 3, &mut pg);
        assert_eq!(grid1.grid[2][1].state, CoordState::Hit);
        assert!(!f2);
        grid1.computer_guess_helper('B', 2, &mut pg);
        assert_eq!(grid1.grid[1][1].state, CoordState::Hit);
        let f3 = grid1.computer_guess_helper('B', 4, &mut pg);
        assert_eq!(grid1.grid[3][1].state, CoordState::Hit);
        assert!(f3);
    }

    #[test]
    fn test_guess() {
        //
        // also tests check_ship_status and check_endgame
        //
        let mut grid1 = Grid::new();
        let mut points = vec![];
        for i in 0..10 {
            for j in 0..10 {
                points.push((i as usize, j as usize));
            }
        }
        grid1.place_user_ship("Destroyer".to_string(), (5, 'C'), (5, 'D'), 2, &mut points);
        grid1.place_user_ship("Battleship".to_string(), (2, 'B'), (4, 'B'), 3, &mut points);
        let (h, (s, l)) = grid1.guess(6, 'C');
        assert_eq!(grid1.grid[5][2].state, CoordState::Miss);
        assert_eq!(h, 'm');
        assert_eq!(s, 'n');
        assert_eq!(l, 0);

        let (h, (s, l)) = grid1.guess(5, 'C');
        assert_eq!(grid1.grid[4][2].state, CoordState::Hit);
        assert_eq!(h, 'h');
        assert_eq!(s, 'u');
        assert_eq!(l, 0);

        let (h, (s, l)) = grid1.guess(5, 'D');
        assert_eq!(grid1.grid[4][3].state, CoordState::Hit);
        assert_eq!(h, 'h');
        assert_eq!(s, 's');
        assert_eq!(l, 2);

        grid1.guess(2, 'B');
        grid1.guess(3, 'B');
        let (h, (s, l)) = grid1.guess(4, 'B');
        assert_eq!(h, 'h');
        assert_eq!(s, 'f');
        assert_eq!(l, 3);
    }
}
