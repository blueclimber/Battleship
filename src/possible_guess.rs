use rand::random;
use std::char;

///
/// This holds all of the information that the computer needs to make
/// logical guesses
/// points is a vector of char i32 pairs, that represent the coordinates that
/// have not yet been guesses
/// resonable_guesses is a vector of the points that should be guessed after a hit
/// next_guesses is a vector of points. When a second or third hit happen, points that are
/// not as logical get moved from reasonable_guesses to next_guesses, to be used in the
/// even that there are ships that are touching and a second guess belongs to a different ship
/// than the first one.
/// Num_hits keeps track of how many unresolved hits there are - hits not belonging to a sunk ship.
/// Last_hit keeps track of the previous hit
/// guess_version is a random bool to assist in the smart_random_guessing
/// difficult is a bool: true for difficult, false for easy.
/// guess_count keeps track of how many guesses have been made
///
pub struct PossibleGuesses {
    pub points: Vec<(char, i32)>,
    pub reasonable_guesses: Vec<(char, i32)>,
    pub next_guesses: Vec<(char, i32)>,
    pub num_hits: i32,
    pub last_hit: (char, i32),
    pub guess_version: bool,
    pub difficult: bool,
    pub guess_count: i32,
}

impl PossibleGuesses {
    ///
    /// Creates a new PossibleGuesses struct
    /// Makes a vector of points
    /// sets reasonable_guesses and next_guesses as empty vecs
    /// sets num_hits to 0
    /// sets last_hit to a point outside of the gameboard, to be replaced on the first hit
    /// sets guess_version to a random bool
    /// sets difficult as easy
    /// sets guess_count at 0
    ///
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
            last_hit: ('z', 100),
            guess_version: rand::random(),
            difficult: false,
            guess_count: 0,
        }
    }

    ///
    /// Returns (col: char, row: i32) to be used in a guess
    /// increments guess_count
    /// Checks if difficult is set and guess_count is less than 50 (to prevent hanging)
    ///     if it is, calls smart_random guess,
    ///     otherwise just returns a random guess and removes the guess from points
    ///
    pub fn random_guess(&mut self) -> (char, i32) {
        self.guess_count += 1;
        if self.difficult && self.guess_count < 50 {
            self.smart_random_guess()
        } else {
            let index = (random::<f32>() * self.points.len() as f32).floor() as usize;
            self.points.remove(index)
        }
    }

    ///
    /// Creates a vector of 100 points, represent rows and columns
    /// If guess_version is true, will only provide guesses where the row and column
    ///     added together are an even number (making a checkerboard pattern)
    /// Opposite if guess_version is false
    /// This allows for better guessing
    /// Then checks the given random guess against the avaiable guessing points (points) to
    /// make sure it is a valid guess.
    /// Loops until a valid guess is found
    ///
    pub fn smart_random_guess(&mut self) -> (char, i32) {
        let mut reference_points: Vec<(i32, i32)> = vec![];
        let mut point = ('z', 0);

        for i in 0..10 {
            for j in 1..11 {
                reference_points.push((i, j))
            }
        }

        let columns: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];

        loop {
            let (col_attempt, row_attempt) = reference_points
                [(random::<f32>() * reference_points.len() as f32).floor() as usize];
            if self.guess_version
                && (col_attempt + row_attempt) % 2 == 0
                && self
                    .points
                    .contains(&(columns[col_attempt as usize], row_attempt))
            {
                if let Some(pos) = self
                    .points
                    .iter()
                    .position(|x| x == &(columns[col_attempt as usize], row_attempt))
                {
                    point = self.points.remove(pos);
                }
                break;
            } else if !self.guess_version
                && (col_attempt + row_attempt) % 2 == 1
                && self
                    .points
                    .contains(&(columns[col_attempt as usize], row_attempt))
            {
                if let Some(pos) = self
                    .points
                    .iter()
                    .position(|x| x == &(columns[col_attempt as usize], row_attempt))
                {
                    point = self.points.remove(pos);
                }
                break;
            }
        }
        point
    }

    ///
    /// Increments self.guess_count
    /// Takes random guesses first from self.reasonable_guesses, and if that is empty, then takes
    /// a guess from self.next_guesses
    /// returns a guess
    ///
    pub fn non_random_guess(&mut self) -> (char, i32) {
        self.guess_count += 1;
        if !self.reasonable_guesses.is_empty() {
            let index = (random::<f32>() * self.reasonable_guesses.len() as f32).floor() as usize;
            let guess = self.reasonable_guesses.remove(index);

            if let Some(pos) = self.points.iter().position(|x| *x == guess) {
                self.points.remove(pos);
            }
            guess
        } else {
            let index = (random::<f32>() * self.next_guesses.len() as f32).floor() as usize;
            let guess = self.next_guesses.remove(index);

            if let Some(pos) = self.points.iter().position(|x| *x == guess) {
                self.points.remove(pos);
            }
            guess
        }
    }

    ///
    /// Updates self.reasonable_guesses and self.next_guesses based on this guess and previous hit
    /// Arguements:
    ///     report: (char, (char, i32)) -
    ///         report.0 - h or m for hit or miss
    ///         report.1.0 - n or s for not sunk or sunk
    ///         report.1.1 - length of sunk ship
    ///     guess_col: char - A-J the column of the guess
    ///     guess_row: i32 - 1-10 the row of the guess
    ///
    /// If report.0 is h, this will check for a sunk ship
    ///     If it's sunk, it will compare the length of the sunk ship to self.num_hits
    ///         If they are equal it will clear out self.next_guesses and self.reasonable_guess
    ///         Otherwise, it will append everything from self.next_guesses into self.reasonable_guesses
    ///     If the ship is not sunk, it will add appropriate points that are touching the guess into
    ///         self.reasonable guesses, and move less likely guesses into next_guesses based on
    ///         if this is the first hit or not.
    ///     
    ///
    pub fn update_guesses(&mut self, report: (char, (char, i32)), guess_col: char, guess_row: i32) {
        let (hit, (sunk, length)) = report;

        let columns: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
        // report.0 - h or m for hit or miss
        // report.1.0 - n or s for not sunk or sunk
        // report.1.1 - length of sunk ship
        if hit == 'h' {
            if sunk == 's' {
                if length < self.num_hits {
                    self.num_hits -= report.1 .1;
                    self.reasonable_guesses
                        .append(&mut self.next_guesses.clone());
                    self.next_guesses = vec![];
                } else {
                    self.num_hits = 0;
                    self.reasonable_guesses = vec![];
                    self.next_guesses = vec![];
                }
            } else {
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
                            if let Some(pos) =
                                self.reasonable_guesses.iter().position(|x| *x == coord)
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
                            if let Some(pos) =
                                self.reasonable_guesses.iter().position(|x| *x == coord)
                            {
                                self.reasonable_guesses.remove(pos);
                            }
                        }
                    }
                }
            }
        }
        self.last_hit = (guess_col, guess_row);
    }
}

impl Default for PossibleGuesses {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pg() {
        let pg = PossibleGuesses::new();
        assert_eq!(pg.points.len(), 100);
        assert!(pg.reasonable_guesses.is_empty());
        assert!(pg.next_guesses.is_empty());
        assert_eq!(pg.num_hits, 0);
        assert_eq!(pg.last_hit, ('z', 100));
        assert!(!pg.difficult);
        assert_eq!(pg.guess_count, 0);
    }

    #[test]
    fn test_random_guess() {
        let mut pg = PossibleGuesses::new();
        let cr1 = pg.random_guess();
        assert_eq!(pg.points.len(), 99);
        let cr2 = pg.random_guess();
        assert_eq!(pg.points.len(), 98);
        assert_ne!(cr1, cr2);
    }

    #[test]
    fn test_update_guesses() {
        let mut pg = PossibleGuesses::new();
        pg.guess_count += 1;
        pg.update_guesses(('h', ('n', 0)), 'B', 3);
        assert_eq!(pg.guess_count, 1);
        assert_eq!(pg.last_hit, ('B', 3));
        assert_eq!(pg.num_hits, 1);
        assert_eq!(pg.reasonable_guesses.len(), 4);
        assert!(pg.reasonable_guesses.contains(&('A', 3)));
        assert!(pg.reasonable_guesses.contains(&('C', 3)));
        assert!(pg.reasonable_guesses.contains(&('B', 2)));
        assert!(pg.reasonable_guesses.contains(&('B', 4)));

        pg.guess_count += 1;
        pg.update_guesses(('h', ('n', 0)), 'B', 4);
        assert_eq!(pg.last_hit, ('B', 4));
        assert_eq!(pg.num_hits, 2);
        assert!(pg.reasonable_guesses.contains(&('B', 2)));
        assert!(pg.reasonable_guesses.contains(&('B', 5)));
        assert!(!pg.reasonable_guesses.contains(&('A', 3)));
        assert!(!pg.reasonable_guesses.contains(&('C', 3)));
        assert!(pg.next_guesses.contains(&('A', 3)));
        assert!(pg.next_guesses.contains(&('C', 3)));

        pg.guess_count += 1;
        pg.update_guesses(('h', ('s', 3)), 'B', 5);
        assert_eq!(pg.last_hit, ('B', 5));
        assert_eq!(pg.num_hits, 0);
        assert!(pg.next_guesses.is_empty());
        assert!(pg.reasonable_guesses.is_empty());
    }
}
