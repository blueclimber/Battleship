use std::{
    char,
    fmt::{self, Display},
    io,
};

#[derive(Copy, Clone, Debug)]
pub enum CoordState {
    Blank,
    Hit,
    Miss,
    Ship,
}

#[derive(Debug, Copy, Clone)]
pub struct Coordinate {
    pub state: CoordState,
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

pub fn get_coord_index(row: i32, column: char) -> (usize, usize) {
    let columns: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
    let rows: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let c_idx = columns.iter().position(|&r| r == column).unwrap();
    let r_idx = rows.iter().position(|&r| r == row).unwrap();

    (r_idx, c_idx)
}

pub fn get_input_coord() -> ((i32, char), bool) {
    let mut valid = true;
    let mut final_col = 'z';
    let mut final_row = 0;

    let columns: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];

    let mut coord = String::new();
    io::stdin()
        .read_line(&mut coord)
        .expect("error: unable to read input");

    if !coord.trim().is_empty() {
        let (col, row) = coord.split_at(1);
        let col: char = col
            .trim()
            .parse()
            .expect("Coordinate should be in the format: A8");
        if col.is_alphabetic() {
            let col = col.to_string().to_uppercase();
            final_col = col
                .as_str()
                .parse()
                .expect("Guess should be in the format: A8");
        } else {
            valid = false;
        }
        if !columns.contains(&final_col) {
            println!("Invalid column: {final_col}");
            valid = false;
        }

        let row = row.trim();
        let mut is_num = true;
        for digit in row.chars() {
            if !digit.is_ascii_digit() {
                is_num = false;
            }
        }
        if is_num {
            final_row = row
                .to_string()
                .parse()
                .expect("Guess should be in the format: A8");
        } else {
            valid = false;
        }
        if final_row <= 0 || final_row > 11 {
            println!("Invalid row: {final_row}");
            valid = false;
        }
    } else {
        final_row = 0;
        final_col = 'z';
        valid = false;
    }

    ((final_row, final_col), valid)
}
