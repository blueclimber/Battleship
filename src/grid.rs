use crate::coord::*;
use crate::possible_guess::*;
use rand::random;
use std::{
    char,
    collections::HashMap,
    fmt::{self, Display},
};

#[derive(Clone, Debug)]
pub struct Grid {
    pub grid: Vec<Vec<Coordinate>>,
    pub ships: HashMap<String, Vec<(usize, usize)>>,
}

impl Grid {
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

    fn update_coord_state(&mut self, row: i32, column: char, new_state: CoordState) {
        // add error checking
        let (r_idx, c_idx) = get_coord_index(row, column);

        self.grid[r_idx][c_idx].state = new_state;
    }

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

    pub fn make_computer_guess(&mut self, possible_guesses: &mut PossibleGuesses) -> bool {
        let mut col = 'a';
        let mut row = 0;
        let mut finished = false;
        if possible_guesses.reasonable_guesses.is_empty()
            && possible_guesses.next_guesses.is_empty()
        {
            (col, row) = possible_guesses.random_guess();
        } else {
            (col, row) = possible_guesses.non_random_guess();
        }
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

    fn guess(&mut self, row: i32, column: char) -> (char, (char, i32)) {
        // check if status of coordinate is not blank
        let rc = get_coord_index(row, column);

        let mut found = false;
        let mut found_ship = "".to_string();
        let mut idx: usize = 0;
        let mut g = 'm'; // for miss
        let mut s = ('n', 0); // for not sunk

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
            new_coords[idx] = (0, 0);
            self.ships.insert(found_ship.clone(), new_coords);
            self.update_coord_state(row, column, CoordState::Hit);
            println!("Hit!");
            g = 'h';
            s = self.check_ship_status(found_ship);
        }
        let report: (char, (char, i32)) = (g, s);
        report
    }

    fn check_ship_status(&mut self, hit_ship: String) -> (char, i32) {
        let coords = self.ships.get(&hit_ship).unwrap();
        let mut s = 'u';
        let mut l = 0;

        let sunk = coords.iter().all(|&coord| coord == (0, 0));
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

    fn check_endgame(&mut self) -> bool {
        let mut finished = false;
        if self.ships.is_empty() {
            println!("\t\tGAME OVER\n\n\n");
            finished = true;
        }
        finished
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
