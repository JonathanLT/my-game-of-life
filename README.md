# Game of Life

## Matrix

### Description
The `Matrix` struct represents a grid with size. It provides methods for matrix a new matrix, updating current matrix with reference and print current matrix.

### Usage
You can use the `Matrix` struct to simulate Conway's Game of Life.

```rust
use Creature;

// Example usage
let mut matrix: Matrix = Matrix {
    size: 2,
    grid: vec![
        vec![Creature::new(0, 0, 2), Creature::new(0, 1, 2)],
        vec![Creature::new(1, 0, 2), Creature::new(1, 1, 1)],
    ],
};
let ref_mat: Matrix = matrix.clone();
matrix.update_matrix(&ref_mat);
assert_eq!(
    matrix.grid,
    vec![
        vec![Creature::new(0, 0, 2), Creature::new(0, 1, 2)],
        vec![Creature::new(1, 0, 2), Creature::new(1, 1, 2)],
    ]
);
```
## Creature

### Description
The `Creature` struct represents a creature with coordinates and an alive state. It provides methods for creating a new creature, setting its alive state, checking its neighboring creatures, and counting the number of alive neighbors.

### Usage
You can use the `Creature` struct to simulate and manipulate creatures within a game matrix.

```rust
// Example usage
let matrix: Vec<Vec<Creature>> = vec![
    vec![Creature::new(0, 0, 1), Creature::new(0, 1, 2)],
    vec![Creature::new(1, 0, 1), Creature::new(1, 1, 1)]
];
let mut c_0 = matrix[0][0].clone();
c_0.check_still_alive(matrix);
assert_eq!(c_0.alive, false);
```

## Licenses
This project is licensed under the MIT License and Apache 2.0 License - see the `LICENSE_*` files for details.

