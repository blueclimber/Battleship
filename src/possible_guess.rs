use rand::random;
use std::char;

pub struct PossibleGuesses {
    pub points: Vec<(char, i32)>,
    pub reasonable_guesses: Vec<(char, i32)>,
    pub next_guesses: Vec<(char, i32)>,
    pub num_hits: i32,
    pub last_hit: (char, i32),
}

impl PossibleGuesses {
    pub fn new() -> Self {
        let columns: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
        let rows: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let mut create_points: Vec<(char, i32)> = vec![];

        for column in &columns {
            for row in &rows {
                create_points.push((*column, *row))
            }
        }

        Self {
            points: create_points.clone(),
            reasonable_guesses: vec![],
            next_guesses: vec![],
            num_hits: 0,
            last_hit: ('a', 0),
        }
    }

    pub fn random_guess(&mut self) -> (char, i32) {
        // for easy stupid play
        let index = (random::<f32>() * self.points.len() as f32).floor() as usize;
        self.points.remove(index)
    }

    pub fn non_random_guess(&mut self) -> (char, i32) {
        let mut guess = ('a', 0);
        if !self.reasonable_guesses.is_empty() {
            let index = (random::<f32>() * self.reasonable_guesses.len() as f32).floor() as usize;
            guess = self.reasonable_guesses.remove(index);
        } else {
            let index = (random::<f32>() * self.next_guesses.len() as f32).floor() as usize;
            guess = self.next_guesses.remove(index);
        }

        if let Some(pos) = self.points.iter().position(|x| *x == guess) {
            self.points.remove(pos);
        }
        guess
    }

    pub fn update_guesses(&mut self, report: (char, (char, i32)), guess_col: char, guess_row: i32) {
        let (hit, (sunk, length)) = report;

        let columns: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
        // report.0 - h or m for hit or miss
        // report.1.0 - n or s for not sunk or sunk
        // report.1.1 - length of sunk ship
        if hit == 'h' {
            if sunk == 's' {
                if length < self.num_hits {
                    self.num_hits = 0;
                    self.reasonable_guesses = self.next_guesses.clone();
                    self.next_guesses = vec![];
                } else {
                    self.num_hits -= report.1 .1;
                    self.reasonable_guesses = vec![];
                    self.next_guesses = vec![];
                }
            }
            if self.num_hits == 0 {
                self.num_hits = 1;
                let col_idx = columns.iter().position(|&r| r == guess_col).unwrap();
                if col_idx < 9 {
                    let new_point = (columns[col_idx + 1], guess_row);
                    if self.points.contains(&new_point) {
                        self.reasonable_guesses.push(new_point);
                    }
                }
                if col_idx > 0 {
                    let new_point = (columns[col_idx - 1], guess_row);
                    if self.points.contains(&new_point) {
                        self.reasonable_guesses.push(new_point);
                    }
                }
                if guess_row < 10 {
                    let new_point = (guess_col, guess_row + 1);
                    if self.points.contains(&new_point) {
                        self.reasonable_guesses.push(new_point);
                    }
                }
                if guess_row > 1 {
                    let new_point = (guess_col, guess_row - 1);
                    if self.points.contains(&new_point) {
                        self.reasonable_guesses.push(new_point);
                    }
                }
            } else {
                self.num_hits += 1;

                let (last_col, last_row) = self.last_hit;
                if last_col == guess_col {
                    if last_row < guess_row && guess_row < 10 {
                        let new_point = (guess_col, guess_row + 1);
                        if self.points.contains(&new_point) {
                            self.reasonable_guesses.push(new_point);
                        }
                    } else if last_row > guess_row && guess_row > 1 {
                        let new_point = (guess_col, guess_row - 1);
                        if self.points.contains(&new_point) {
                            self.reasonable_guesses.push(new_point);
                        }
                    }
                    let unlikely_guesses: Vec<(char, i32)> = self
                        .reasonable_guesses
                        .iter()
                        .copied()
                        .filter(|&p| p.0 != last_col)
                        .collect();
                    for coord in unlikely_guesses {
                        self.next_guesses.push(coord);
                        if let Some(pos) = self.reasonable_guesses.iter().position(|x| *x == coord)
                        {
                            self.reasonable_guesses.remove(pos);
                        }
                    }
                } else {
                    // else if last_row == guess_row
                    let col_idx = columns.iter().position(|&r| r == guess_col).unwrap();
                    let last_col_idx = columns.iter().position(|&r| r == last_col).unwrap();
                    if last_col_idx < col_idx && col_idx < 9 {
                        let new_point = (columns[col_idx + 1], guess_row);
                        if self.points.contains(&new_point) {
                            self.reasonable_guesses.push(new_point);
                        }
                    }
                    if last_col_idx > col_idx && col_idx > 0 {
                        let new_point = (columns[col_idx - 1], guess_row);
                        if self.points.contains(&new_point) {
                            self.reasonable_guesses.push(new_point);
                        }
                    }
                    let unlikely_guesses: Vec<(char, i32)> = self
                        .reasonable_guesses
                        .iter()
                        .copied()
                        .filter(|&p| p.1 != last_row)
                        .collect();
                    for coord in unlikely_guesses {
                        self.next_guesses.push(coord);
                        if let Some(pos) = self.reasonable_guesses.iter().position(|x| *x == coord)
                        {
                            self.reasonable_guesses.remove(pos);
                        }
                    }
                }
                // else {} look at first guess
            }
        }
        self.last_hit = (guess_col, guess_row);
    }
}
