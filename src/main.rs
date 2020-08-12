use std::time::{Duration, Instant};

pub mod lib;

use lib::*;

fn main() {
    let upper = 100000000;

    for f in [fnaive1, fopt1, fopt2].iter() {
        let start = Instant::now();
        let result = f(upper);
        let duration = start.elapsed();

        println!("Got result {} with duration {:?}", result, duration);
    }
}
