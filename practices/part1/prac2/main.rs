#![deny(clippy::all)]
#![warn(clippy::pedantic)]

const N: u8 = 10;

fn main() -> anyhow::Result<()> {
    let x = 1.0;
    let cos_value = cos(x)?;
    let exp_value = exp(x)?;
    let log_value = log(x)?;

    println!("cos({x:?}) = {cos_value}");
    println!("exp({x:?}) = {exp_value}");
    println!("log({x:?}) = {log_value}");

    Ok(())
}

fn cos(x: f64) -> anyhow::Result<f64> {
    let mut sum = 1.0;
    let mut term = 1.0;
    let mut i = 1;
    while i < N {
        term *= -x * x / f64::try_from(2 * i)? / f64::try_from(2 * i - 1)?;
        sum += term;
        i += 1;
    }

    Ok(sum)
}

fn exp(x: f64) -> anyhow::Result<f64> {
    let mut sum = 1.0;
    let mut term = 1.0;
    let mut i = 1;
    while i < N {
        term *= x / f64::try_from(i)?;
        sum += term;
        i += 1;
    }

    Ok(sum)
}

fn log(x: f64) -> anyhow::Result<f64> {
    let mut sum = 0.0;
    let mut term = (x - 1.0) / x;
    let mut i = 1;
    while i < N {
        sum += term / f64::try_from(i)?;
        term *= (x - 1.0) / x;
        i += 1;
    }

    Ok(sum)
}
