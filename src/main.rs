mod bigint;

fn main() {
    // BigInt with 200 digits
    type I200 = bigint::BigInt<200>;

    // calculate 100! at compile time
    const FACTORIAL_100: I200 = {
        let mut result = I200::from_i128(1);
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
    const MUL: I200 = A.mul(B);
    // calculate 1234567890001 + 9876543210001 at compile time
    const ADD: I200 = A.add(B);
    // calculate 1234567890001 - 9876543210001 at compile time
    const SUB: I200 = A.sub(B);
    // calculate FACTORIAL_100 / A at compile time
    const DIV: I200 = FACTORIAL_100.div(A).0;
    // calculate FACTORIAL_100 % A at compile time
    const REM: I200 = FACTORIAL_100.div(A).1;

    println!("1234567890001 * 9876543210001 = {}", MUL);
    println!("1234567890001 + 9876543210001 = {}", ADD);
    println!("1234567890001 - 9876543210001 = {}", SUB);
    println!("100! / 1234567890001 = {}", DIV);
    println!("100! % 1234567890001 = {}", REM);


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
    let mul = a * b;
    // calculate 1234567890001 + 9876543210001 at runtime
    let add = a + b;
    // calculate 1234567890001 - 9876543210001 at runtime
    let sub = a - b;
    // calculate 100! / 1234567890001 at runtime
    let div = factorial_100 / a;
    // calculate 100! % 1234567890001 at runtime
    let rem = factorial_100 % a;

    println!("1234567890001 * 9876543210001 = {}", mul);
    println!("1234567890001 + 9876543210001 = {}", add);
    println!("1234567890001 - 9876543210001 = {}", sub);
    println!("100! / 1234567890001 = {}", div);
    println!("100! % 1234567890001 = {}", rem);


    assert_eq!(FACTORIAL_100, factorial_100);
    assert_eq!(MUL, mul);
    assert_eq!(ADD, add);
    assert_eq!(SUB, sub);
    assert_eq!(DIV, div);
    assert_eq!(REM, rem);
}