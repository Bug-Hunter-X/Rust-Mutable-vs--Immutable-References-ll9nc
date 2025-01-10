# Rust Mutable vs. Immutable References
This repository demonstrates a common error in Rust related to mutable and immutable references. The `bug.rs` file contains code that correctly utilizes mutable references, while attempting to modify through an immutable reference is commented out to highlight the error.

## Bug Description
The bug is the misconception that immutability is automatically enforced even with mutable references. The commented-out line in `bug.rs` would produce a compile-time error if uncommented.