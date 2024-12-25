# CP-BigInt

A simple big integer crate written in Rust.  
Currently, it supports compile-time addition, subtraction, multiplication, and division.

## Motivation

I needed a simple arbitrary-precision integer implementation to copy-paste for competitive programming problems.

## Usage Example

```rust
type I200 = bigint::BigInt<200>;

// Compile-Time: Calculate 100!
const FACTORIAL_100: I200 = {
    let mut result = I200::from_i128(1);
    let mut i = 0;
    while i < 100 {
        // stable const evaluation does not allow '*' operator yet
        result = result.mul(I200::from_i128(i as i128 + 1));
        i += 1;
    }
    result
};

// Runtime: Multiply two big integers
let a = I200::from_str("1234567890001");
let b = I200::from_str("9876543210001");
let mul = a * b;
```
