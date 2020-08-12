use std::time::{Duration, Instant};

pub mod lib;

use lib::*;

fn main() {
    let upper = 1000000000;

    for (f, fnname) in [
        (nothing as fn(u64) -> u64, "empty"),
        (fnaive1, "fnaive1"),
        (fopt1, "fopt1"),
        (fopt2, "fopt2"),
        (fopt3, "fopt3"),
        (fzbz2, "fzbz2"),
        (fzbz1, "fzbz1"),
        (fbf, "fbf"),
    ].iter() {
        //let f: &fn(u64) -> u64 = f;
        let start = Instant::now();
        let result = f(upper);
        let duration = start.elapsed();

        println!("Got result {} with duration {:?} from {}", result, duration, fnname);
    }
}
