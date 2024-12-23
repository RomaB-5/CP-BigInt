mod bigint;

fn main() {
    // BigInt with 200 digits
    type I200 = bigint::BigInt<200>;

    // calculate 100! at compile time
    const FACTORIAL_100: I200 = {
        let mut result = I200::from_str("1");
        let mut i = 0;
        while i < 100 {
            result = result.mul(I200::from_i128(i as i128 + 1));
            i += 1;
        }
        result
    };

    println!("Compile time calculation:");
    println!("100! = {}", FACTORIAL_100);


    const A: I200 = I200::from_str("1234567890001");
    const B: I200 = I200::from_str("9876543210001");
    // calculate 1234567890001 * 9876543210001 at compile time
    const X: I200 = A.mul(B);
    // calculate 1234567890001 + 9876543210001 at compile time
    const Y: I200 = A.add(B);
    // calculate 1234567890001 - 9876543210001 at compile time
    const Z: I200 = A.sub(B);

    println!("1234567890001 * 9876543210001 = {}", X);
    println!("1234567890001 + 9876543210001 = {}", Y);
    println!("1234567890001 - 9876543210001 = {}", Z);


    // calculate 100! at runtime
    let factorial_100 = {
        let mut result = I200::from_str("1");
        let mut i = 0;
        while i < 100 {
            result = result.mul(I200::from_i128(i as i128 + 1));
            i += 1;
        }
        result
    };

    println!("\nRuntime calculation:");
    println!("100! = {}", factorial_100);

    let a = I200::from_str("1234567890001");
    let b = I200::from_str("9876543210001");
    // calculate 1234567890001 * 9876543210001 at runtime
    let x = a * b;
    // calculate 1234567890001 + 9876543210001 at runtime
    let y = a + b;
    // calculate 1234567890001 - 9876543210001 at runtime
    let z = a - b;

    println!("1234567890001 * 9876543210001 = {}", x);
    println!("1234567890001 + 9876543210001 = {}", y);
    println!("1234567890001 - 9876543210001 = {}", z);


    assert_eq!(FACTORIAL_100, factorial_100);
    assert_eq!(X, x);
    assert_eq!(Y, y);
    assert_eq!(Z, z);
}