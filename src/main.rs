use crate::classes::matrix::Matrix;

mod classes;

/// Main function that initializes and updates the game matrix.
///
/// This function creates a new game matrix with 5x5 dimensions,
/// and iteratively prints the matrix state, clones the matrix,
/// updates the matrix state, and prints the updated matrix state.
fn main() {
    // Initialize the game matrix with 5x5 dimensions,
    // and populate each cell with a new creature.
    let mut matrix: classes::matrix::Matrix = classes::matrix::Matrix::new(5);

    // Print the initial reference matrix state
    println!("Ref");
    matrix.print_matrix();

    // Iterate 5 times
    for i in 0..5 {
        // Clone the matrix
        let ref_mat: Matrix = matrix.clone();

        // Print the cloned matrix state
        println!("cloned {}", i);
        ref_mat.print_matrix();

        // Update the original matrix state by calling
        // the check_still_alive function on each creature
        matrix.update_matrix(&ref_mat);

        // Print the updated reference matrix state
        println!("ref {}", i);
        matrix.print_matrix();

        // Print the cloned matrix state again
        println!("cloned {}", i);
        ref_mat.print_matrix();
    }
}
