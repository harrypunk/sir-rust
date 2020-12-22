#[allow(dead_code)]
pub struct InitialValue {
    pub n: u32,
    pub s0: u32,
    pub i0: u32,
    pub r0: u32,
}

pub mod sir {
    use super::InitialValue;
    impl InitialValue {
        pub fn calc(&self) -> u32 {
            return 21;
        }
    }
}
