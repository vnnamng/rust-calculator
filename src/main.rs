mod big_num;

use big_num::BigNum;

fn main() {
    let big_num1 = BigNum::from_str("-123");
    let big_num2 = BigNum::from_str("12");

    println!("BigNum1: {}", big_num1);
    println!("BigNum2: {}", big_num2);

    let sum = big_num1.add(&big_num2);
    println!("Sum: {}", sum);

    let difference = big_num1.minus(&big_num2);
    println!("Difference: {}", difference);

    let big_num3 = BigNum::from_str("1000000000000000000000000000000000000001");
    let big_num4 = BigNum::from_str("99");

    let product = big_num3.multiply(&big_num4);
    println!("Product: {}", product);
}
