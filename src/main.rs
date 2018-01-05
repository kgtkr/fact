extern crate num_bigint;
use std::env;
use num_bigint::ToBigUint;
use std::time::Instant;

fn main() {
    let max = env::args().nth(1).unwrap().parse::<i32>().unwrap();

    let mut i = 1;
    let mut n = 1.to_biguint().unwrap();

    let start = Instant::now();
    loop {
        n = n * i.to_biguint().unwrap();
        if i == max {
            break;
        }
        i += 1;
    }
    let end = start.elapsed();

    println!("{}! = {}", i, n);
    println!("len = {}", n.to_string().len());
    println!(
        "time = {}.{:04}s",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
