use serde::Deserialize;
use serde_json;
use sir_rust::algorithm::sir::Euler;
use sir_rust::algorithm::{Coefficients, InitialValue, Method, SimResult};
use std::fs;

pub fn main() {
    let json_str: String = fs::read_to_string("./example-js-api.json").expect("read error");
    let example: Example = serde_json::from_str(&json_str).expect("json error");

    let Body {
        n,
        s0,
        i0,
        r0,
        infection_rate,
        recover_rate,
        days,
    } = example.request.body;
    let init_val1 = InitialValue { n, s0, i0, r0 };
    let euler = Euler {
        init_val: init_val1,
        rates: Coefficients {
            infection_rate,
            recover_rate,
        },
    };

    let result1: SimResult = euler.calc(days);

    let n: f64 = n as f64;
    result1
        .data
        .iter()
        .map(|(s, i, r)| (s * n, i * n, r * n))
        .map(|(s, i, r)| (s as u32, i as u32, r as u32))
        .zip(example.response.sir.iter())
        //.enumerate()
        .map(|((s, i, r), arr)| {
            let s1 = arr[0];
            let i1 = arr[1];
            let r1 = arr[2];
            (diff(s, s1), diff(i, i1), diff(r, r1))
        })
        .take(30)
        .for_each(|x| println!("{:?}", x));
}

fn diff(a: u32, b: u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

type SIRTuple = Vec<u32>;

#[derive(Deserialize)]
#[allow(dead_code)]
struct Example {
    request: Request,
    response: Response,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Request {
    body: Body,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Body {
    #[serde(rename(deserialize = "N"))]
    n: u32,
    i0: u32,
    s0: u32,
    r0: u32,
    #[serde(rename(deserialize = "infectionRate"))]
    infection_rate: f64,
    #[serde(rename(deserialize = "recoveryRate"))]
    recover_rate: f64,
    days: u32,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Response {
    sir: Vec<SIRTuple>,
}
