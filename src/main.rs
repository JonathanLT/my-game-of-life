use rand::Rng;

/// Struct representing a creature in the game of life.
///
/// # Fields
///
/// - `alive`: A boolean indicating whether the creature is alive or not.
/// - `x`: The x-coordinate of the creature.
/// - `y`: The y-coordinate of the creature.
#[derive(Debug, Clone, PartialEq)]
pub struct Creature {
    alive: bool,
    x: usize,
    y: usize,
}

impl Creature {
    /// Creates a new creature with the given coordinates and alive state.
    ///
    /// # Arguments
    ///
    /// - `x`: The x-coordinate of the creature.
    /// - `y`: The y-coordinate of the creature.
    /// - `alive`: The alive state of the creature, represented as an i8.
    ///           0 or 2 represents a live creature, while any other value represents a dead creature.
    #[must_use]
    pub fn new(x: usize, y: usize, alive: i8) -> Creature {
        Creature {
            alive: match alive {
                0 | 2 => true,
                _ => false,
            },
            x,
            y,
        }
    }

    /// Sets the alive state of the creature.
    ///
    /// # Arguments
    ///
    /// - `alive`: The new alive state of the creature.
    pub fn set_alive(&mut self, alive: bool) {
        self.alive = alive;
    }

    /// Checks the neighboring creatures of the current creature and updates its alive state accordingly.
    ///
    /// # Arguments
    ///
    /// - `matrix`: The game matrix containing all the creatures.
    pub fn check_still_alive(&mut self, matrix: Vec<Vec<Creature>>) {
        match self.check_neighbors(matrix) {
            2 => {
                if self.alive {
                    self.set_alive(true)
                }
            }
            3 => {
                if !self.alive {
                    self.set_alive(true)
                }
            }
            _ => self.set_alive(false),
        };
    }

    /// Counts the number of alive neighbors of the current creature.
    ///
    /// # Arguments
    ///
    /// - `matrix`: The game matrix containing all the creatures.
    ///
    /// # Returns
    ///
    /// The number of alive neighbors of the current creature, represented as a u8.
    #[must_use]
    pub fn check_neighbors(&self, matrix: Vec<Vec<Creature>>) -> u8 {
        let max_i = matrix.len() - 1;
        let max_j = matrix[0].len() - 1;

        let mut count: u8 = 0;
        for x in (self.x.saturating_sub(1))..=(self.x + 1).min(max_i) {
            for y in (self.y.saturating_sub(1))..=(self.y + 1).min(max_j) {
                if x != self.x || y != self.y {
                    count += matrix[x][y].alive as u8;
                }
            }
        }
        count
    }
}

/// This struct implements the `Display` trait, which allows us to
/// print the `Creature` struct in a nice format.
impl std::fmt::Display for Creature {
    /// Formats the `Creature` struct into a string, which is then
    /// returned as a `Result` of type `std::fmt::Result`. The formatted
    /// string represents the state of the creature, whether it is alive
    /// or not, and is enclosed in square brackets.
    ///
    /// # Arguments
    ///
    /// * `f`: A formatter struct, which allows us to specify how to
    ///        format the data.
    ///
    /// # Returns
    ///
    /// A formatted string, representing the state of the creature.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format the string, representing the state of the creature.
        write!(f, "[{}]", self.alive as u8)
    }
}

/// Main function that initializes and updates the game matrix.
///
/// This function creates a new game matrix with 5x5 dimensions,
/// and iteratively prints the matrix state, clones the matrix,
/// updates the matrix state, and prints the updated matrix state.
fn main() {
    // Create a new RNG for random number generation
    let mut rng = rand::thread_rng();

    // Initialize the game matrix with 5x5 dimensions,
    // and populate each cell with a new creature.
    let mut matrix: Vec<Vec<Creature>> = (0..5)
        .map(|x| {
            (0..5)
                .map(|y| Creature::new(x, y, rng.gen_range(0..=4)))
                .collect()
        })
        .collect();

    // Print the initial reference matrix state
    println!("Ref");
    print_matrix(&matrix);

    // Iterate 5 times
    for i in 0..5 {
        // Clone the matrix
        let ref_mat: Vec<Vec<Creature>> = matrix.clone();

        // Print the cloned matrix state
        println!("cloned {}", i);
        print_matrix(&ref_mat);

        // Update the original matrix state by calling
        // the check_still_alive function on each creature
        update_matrix(&mut matrix, &ref_mat);

        // Print the updated reference matrix state
        println!("ref {}", i);
        print_matrix(&matrix);

        // Print the cloned matrix state again
        println!("cloned {}", i);
        print_matrix(&ref_mat);
    }
}

/// Prints the given matrix to the console.
///
/// # Arguments
///
/// - `matrix`: The matrix to print.
fn print_matrix(matrix: &Vec<Vec<Creature>>) {
    for row in matrix.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }
}

/// Updates the given matrix by calling the check_still_alive
/// function on each creature.
///
/// # Arguments
///
/// - `matrix`: The matrix to update.
/// - `ref_mat`: The cloned matrix to reference.
fn update_matrix(matrix: &mut Vec<Vec<Creature>>, ref_mat: &Vec<Vec<Creature>>) {
    for row in matrix.iter_mut() {
        for c in row.iter_mut() {
            c.check_still_alive(ref_mat.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Creature;

    #[test]
    fn test_new_creature() {
        let creature = Creature::new(0, 0, 2);
        assert_eq!(creature.alive, true);
        assert_eq!(creature.x, 0);
        assert_eq!(creature.y, 0);
    }

    #[test]
    fn test_set_alive() {
        let mut creature = Creature::new(0, 0, 2);
        creature.set_alive(false);
        assert_eq!(creature.alive, false);
    }

    #[test]
    fn test_check_neighbors() {
        let matrix = vec![
            vec![Creature::new(0, 0, 2), Creature::new(0, 0, 2)],
            vec![Creature::new(0, 0, 2), Creature::new(0, 0, 1)],
        ];
        let c = matrix[0][0].clone();
        assert_eq!(c.check_neighbors(matrix), 2);
    }

    #[test]
    fn test_check_still_alive() {
        let matrix: Vec<Vec<Creature>> = vec![
            vec![Creature::new(0, 0, 2), Creature::new(0, 0, 2)],
            vec![Creature::new(0, 0, 2), Creature::new(0, 0, 1)],
        ];
        let mut c_0 = matrix[0][0].clone();
        c_0.check_still_alive(matrix);
        assert_eq!(c_0.alive, true);

        let matrix: Vec<Vec<Creature>> = vec![
            vec![Creature::new(0, 0, 1), Creature::new(0, 0, 2)],
            vec![Creature::new(0, 0, 1), Creature::new(0, 0, 1)],
        ];
        let mut c_0 = matrix[0][0].clone();
        c_0.check_still_alive(matrix);
        assert_eq!(c_0.alive, false);
    }
}
