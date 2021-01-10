use sir_rust::algorithm::sir::Euler;
use sir_rust::algorithm::{Coefficients, InitialValue, Method, SimResult};

pub fn main() {
    let init_val1 = InitialValue {
        n: 1000,
        s0: 999,
        i0: 10,
        r0: 2,
    };
    let euler = Euler {
        init_val: init_val1,
        rates: Coefficients {
            infection_rate: 0.3,
            recover_rate: 0.5,
        },
    };

    let days = 30;

    let result1: SimResult = euler.calc(days);

    result1
        .data
        .iter()
        .enumerate()
        .for_each(|(i, sir)| println!("day: {}, sir: {}", i, sir.0));
}
