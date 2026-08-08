# Rust CLI Calculator

A simple command-line calculator built in Rust as part of my learning journey.

---

## Features

- Basic operations:
  - add
  - sub
  - mul
  - div
- Unary operation:
  - square
  - sqrt
- Input validation (prevents invalid numbers)
- Continuous loop until user exits
- Handles invalid operations safely

---

## What I Learned

- Using `HashMap` to map user input to behavior
- Replacing repetitive `match` logic with a scalable design
- Using enums to represent different types of operations
- Function pointers (`fn`) in Rust
- Pattern matching with `Option` and custom enums
- Structuring CLI-based programs
- Initially struggled to understand enum vs struct usage
- Found hashmap-based design much cleaner than match-based logic

---

## Design Approach

Instead of using large match statements for every operation, this project uses:

- `HashMap<&str, OperationType>` for lookup
- `enum OperationType` to define unary and binary operations

This makes the code easier to extend and maintain.

---

## How to Run

```bash
cargo run
```

---

## Future Improvements

- Add more operations (cube, power, etc.)
- Support more than 2 inputs
- Handle floating-point inputs directly
- Improve error handling using `Result`
- Add history of calculations
- Convert into a reusable library
- Improve CLI experience (better prompts, formatting)

---

## Note

This project is focused on learning Rust concepts rather than production-level code. More improvements and refactoring will be done as I progress.
