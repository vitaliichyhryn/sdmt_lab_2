# Doubly Linked List
![CI](https://github.com/vitaliichyhryn/sdmt_lab_2/actions/workflows/ci.yml/badge.svg)

This is an implementation of the **doubly linked list** data structure written in Rust.

## Documentation
Build and open the documentation:
```bash
cargo doc --open
```

## Variant
Variant calculation:
`25 mod 4 = 1`

According to the variant, stages of implementation of the doubly linked list were:
1. **Custom implementation**
2. **Refactored implementation using built-in collections**

## Instructions

### Prerequisites
Ensure you have [Rust](https://www.rust-lang.org/) installed on your system.

### Clone the repository
```bash
git clone https://github.com/vitaliichyhryn/sdmt_lab_2.git
cd sdmt_lab_2
```

### Build the crate
```bash
cargo build --release
```

### Run the binary
```bash
cargo run --release
```

### Run the tests
```bash
cargo test
```

## Failed CI build
As instructed, this repository contains a commit with [a failed CI build](https://github.com/vitaliichyhryn/sdmt_lab_2/commit/640bdabdee9962a4ae54d43e08dc1fe62e5f657d).

## Conclusions
I decided to go with test-driven development for this assignment, which meant writing the unit tests first and then coding the actual implementation to make those tests pass. The main benefit, in my opinion, is that if the tests pass, you can be pretty confident that your code is functioning as it's supposed to. I also think it's a very nice way to catch regressions, especially when performing significant refactors of the codebase. Overall, it made the whole development process feel smooth and more structured, though it took a bit more effort upfront to write the tests first.
