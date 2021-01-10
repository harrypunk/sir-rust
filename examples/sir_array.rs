use sir_rust::algorithm::sir::Euler;
use sir_rust::algorithm::{Coefficients, InitialValue, Method, SimResult};

pub fn main() {
    let population = 500000;
    let init_val1 = InitialValue {
        n: population,
        s0: 400000,
        i0: 100000,
        r0: 10,
    };
    let euler = Euler {
        init_val: init_val1,
        rates: Coefficients {
            infection_rate: 0.5,
            recover_rate: 0.05,
        },
    };

    let days = 30;

    let result1: SimResult = euler.calc(days);

    let n: f64 = population as f64;
    result1
        .data
        .iter()
        .map(|(s, i, r)| (s * n, i * n, r * n))
        .map(|(s, i, r)| (s as u32, i as u32, r as u32))
        .enumerate()
        .for_each(|(i, sir)| println!("day: {}, sir: {:?}", i, sir));
}
