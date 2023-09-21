#![deny(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
    let a = 100;
    let b = 4;

    let difference = subtraction(a, b);
    let product = multiplication(a, b);
    let quotient = division(a, b);

    println!("a: {a} ({a:04b})");
    println!("b: {b} ({b:04b})");
    println!("a - b: {difference} ({difference:04b})");
    println!("a * b: {product} ({product:04b})");
    println!("a / b: {quotient} ({quotient:04b})");
}

fn subtraction(a: i32, b: i32) -> i32 {
    a + 1 + !b
}

fn multiplication(a: i32, mut b: i32) -> i32 {
    let mut product = 0;
    let mut shift = 0;

    while b > 0 {
        if (b & 1) == 1 {
            product += a << shift;
        }

        b >>= 1;
        shift += 1;
    }

    product
}

fn division(a: i32, b: i32) -> i32 {
    let mut quotient = 0;
    let mut remainder = a;

    while remainder >= b {
        let mut shift = 0;
        while remainder >= (b << shift) {
            shift += 1;
        }

        let divisor = b << (shift - 1);
        remainder -= divisor;
        quotient += 1 << (shift - 1);
    }

    quotient
}
