# Multiple Mutable Borrows in Rust
This example demonstrates a common error in Rust: attempting to create multiple mutable borrows of the same variable. Rust's borrow checker prevents this to ensure data integrity and prevent race conditions.

## The Problem
The code in `bug.rs` attempts to create two mutable references (`y` and `z`) to the same variable `x`. This violates Rust's borrowing rules, resulting in a compile-time error.

## The Solution
The solution in `bugSolution.rs` demonstrates how to correctly handle mutable borrows.  Only one mutable borrow is active at any given time.  The code refactors to use a single mutable reference or to use interior mutability techniques like `RefCell` or `Mutex` if concurrent modification is needed (but this example does not illustrate that case)