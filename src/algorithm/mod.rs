#[allow(dead_code)]
pub struct InitialValue {
    pub n: u32,
    pub s0: u32,
    pub i0: u32,
    pub r0: u32,
}

pub struct Coefficients {
    pub infection_rate: f64,
    pub recover_rate: f64,
}

// SIR percentage
type SIRTuple = (f64, f64, f64);

pub struct SimResult {
    pub method: String,
    pub data: Vec<SIRTuple>,
}

pub trait Method {
    fn calc(&self, days: u32) -> SimResult;
}

pub mod sir {
    use super::{Coefficients, InitialValue, Method, SIRTuple, SimResult};
    pub struct Euler {
        pub init_val: InitialValue,
        pub rates: Coefficients,
    }

    impl Euler {
        pub fn get_next(&self, current: &SIRTuple) -> SIRTuple {
            let (s_i, i_i, r_i) = current;
            let Coefficients {
                infection_rate,
                recover_rate,
            } = self.rates;
            let delta_s = infection_rate * i_i * s_i;
            let delta_r = recover_rate * i_i;
            (-1.0 * delta_s + s_i, delta_s - delta_r + i_i, delta_r + r_i)
        }
    }

    impl Method for Euler {
        fn calc(&self, days: u32) -> SimResult {
            let InitialValue { s0, i0, r0, n } = self.init_val;
            let n = n as f64;
            let s0 = s0 as f64;
            let i0 = i0 as f64;
            let r0 = r0 as f64;
            let sir0: SIRTuple = (s0 / n, i0 / n, r0 / n);

            let mut arr = vec![sir0];
            for i in 0..days {
                let current = arr[i as usize];
                let sir_next = self.get_next(&current);
                arr.push(sir_next);
            }

            SimResult {
                method: "euler".to_string(),
                data: arr,
            }
        }
    }
}
