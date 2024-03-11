# Creature

## Description
The `Creature` struct represents a creature with coordinates and an alive state. It provides methods for creating a new creature, setting its alive state, checking its neighboring creatures, and counting the number of alive neighbors.

## Usage
You can use the `Creature` struct to simulate and manipulate creatures within a game matrix.

```rust
// Example usage
let matrix: Vec<Vec<Creature>> = vec![
    vec![Creature::new(0, 0, 1), Creature::new(0, 0, 2)],
    vec![Creature::new(0, 0, 1), Creature::new(0, 0, 1)]
];
let mut c_0 = matrix[0][0].clone();
c_0.check_still_alive(matrix);
assert_eq!(c_0.alive, false);
```

## Licenses
This project is licensed under the MIT License and Apache 2.0 License - see the `LICENSE_*` files for details.

