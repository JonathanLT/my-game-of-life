use std::{thread, time};

mod classes;

/// Main function that initializes and updates the game matrix.
///
/// This function creates a new game matrix with 5x5 dimensions,
/// and iteratively prints the matrix state, clones the matrix,
/// updates the matrix state, and prints the updated matrix state.
fn main() {
    // Initialize the game matrix with 6x6 dimensions.
    // This matrix will be used to simulate the game of life.
    // The 'classes' module contains the Matrix struct that
    // represents the game matrix and its operations.
    let mut matrix: classes::matrix::Matrix = classes::matrix::Matrix::new(6);

    // Define the duration of 1 second in milliseconds.
    // This duration will be used to pause the execution
    // for 1 second in the main loop.
    let a_sec: time::Duration = time::Duration::from_millis(1000);

    // Initialize the iteration counter to 0.
    // This counter will be used to keep track of the
    // number of iterations in the main loop.
    let mut ite: u128 = 0;

    // Main game loop that iteratively prints the matrix state,
    // clones the matrix, updates the matrix state, and prints
    // the updated matrix state until no creature is still alive.
    //
    // The loop runs indefinitely, breaking when no creature
    // is still alive.
    loop {
        // Print the current iteration number
        println!("Iteration: {}", ite);

        // Increment the iteration counter
        ite += 1;

        // Print the current state of the matrix
        matrix.print_matrix();

        // Update the matrix state based on the neighboring
        // creatures
        if !matrix.update_matrix(&matrix.clone()) {
            // If no creature is still alive, break the loop
            break;
        }

        // Pause the execution for 1 second
        thread::sleep(a_sec);
    }
}
