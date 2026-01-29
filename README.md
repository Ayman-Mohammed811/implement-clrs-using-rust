# Implement CLRS using Rust

>>This repository contains implementations of algorithms and concepts from the book **"Introduction to Algorithms" (4th Edition)** using the Rust programming language.

## Project Structure

The project is organized into three main modules:

* **[Algorithms](./src/algorithms)**: Contains the implementation of various algorithms. Each sub-module includes a `.md` for definitions and theoretical notes.
* **[Data Structures](./src/data_structures)**: Contains the organizational structures used to manage data efficiently.
* **[Math](./src/math)**: A helper module focusing on mathematical theories and proofs related to the algorithms (e.g., summations, recurrences).

```text
src/
├── algorithms/      # Algorithm implementations & notes
├── data_structures/ # Data organization
├── math/            # Mathematical foundations
└── lib.rs           # Main library entry point
```

### BOOK ROADMAP

- **Foundations**
   - **[The Role of Algorithms in Computing](./src/algorithms/role_of_algorithms_in_computing.md)**
   - **Getting Started**
     - [Insertion sort](./src/algorithms/insertion-sort.md)
     - [Analyzing Algorithms](./src/algorithms/analyzing-algorithms.md)
   - **[Characterizing Running Times](./src/math/running-times.md)**