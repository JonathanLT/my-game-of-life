use rand::Rng;

use crate::classes::creature::Creature;

/// Struct representing a matrix in the game of life.
///
/// # Fields
///
/// - `size`: Size of the matrix.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    size: usize,
    grid: Vec<Vec<Creature>>,
}

impl Matrix {
    /// Creates a new square matrix with the given x and y size.
    ///
    /// # Arguments
    ///
    /// - `size`: Size.
    #[must_use]
    pub fn new(size: usize) -> Matrix {
        // Create a new RNG for random number generation
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

        Matrix {
            size,
            grid: (0..size)
                .map(|x| {
                    (0..size)
                        .map(|y| Creature::new(x, y, rng.gen_range(0..=4)))
                        .collect()
                })
                .collect(),
        }
    }

    /// Prints the matrix to the console.
    pub fn print_matrix(&self) {
        for row in self.grid.iter() {
            for c in row.iter() {
                print!("{}", c);
            }
            println!();
        }
        println!();
    }

    /// Updates the matrix by calling with ref_mat the check_still_alive
    /// function on each creature.
    ///
    /// # Arguments
    ///
    /// - `ref_mat`: The cloned matrix to reference.
    pub fn update_matrix(&mut self, ref_mat: &Matrix) -> bool {
        let mut creature_still_alive: Vec<bool> = Vec::new();
        for row in self.grid.iter_mut() {
            for c in row.iter_mut() {
                c.check_still_alive(&ref_mat.grid);
                if c.is_alive() {
                    creature_still_alive.push(true);
                }
            }
        }
        match creature_still_alive.len() {
            0 => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Creature;
    use super::Matrix;

    #[test]
    fn test_update_matrix() {
        let mut matrix = Matrix {
            size: 2,
            grid: vec![
                vec![Creature::new(0, 0, 2), Creature::new(0, 1, 2)],
                vec![Creature::new(1, 0, 2), Creature::new(1, 1, 1)],
            ],
        };
        let ref_mat: Matrix = matrix.clone();
        assert_eq!(matrix.update_matrix(&ref_mat), true);
        assert_eq!(
            matrix.grid,
            vec![
                vec![Creature::new(0, 0, 2), Creature::new(0, 1, 2)],
                vec![Creature::new(1, 0, 2), Creature::new(1, 1, 2)],
            ]
        );
    }
}
