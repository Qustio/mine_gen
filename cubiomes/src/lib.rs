use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::mem::MaybeUninit;

use cubiomes_sys::{applySeed, setupGenerator};

pub struct Generator {
    generator: MaybeUninit<cubiomes_sys::Generator>,
}

impl Generator {
    pub fn new(version: MCVersion) -> Self {
        let mut generator = MaybeUninit::<cubiomes_sys::Generator>::zeroed();
        unsafe {
            setupGenerator(generator.as_mut_ptr(), version as i32, 0);
        }
        Self { generator }
    }

    pub fn set_seed(&mut self, dimention: Dimension, seed: u64) {
        unsafe {
            applySeed(self.generator.as_mut_ptr(), dimention.into(), seed);
        }
    }

    pub fn get_seed(&self) -> u64 {
        unsafe { self.generator.assume_init().seed }
    }

    pub fn get_dim(&self) -> Dimension {
        unsafe { Dimension::try_from(self.generator.assume_init().dim).unwrap() }
    }
}

#[allow(non_camel_case_types)]
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum MCVersion {
    MC_UNDEF = 0,
    MC_B1_7 = 1,
    MC_B1_8 = 2,
    MC_1_0_0 = 3,
    MC_1_1_0 = 4,
    MC_1_2_5 = 5,
    MC_1_3_2 = 6,
    MC_1_4_7 = 7,
    MC_1_5_2 = 8,
    MC_1_6_4 = 9,
    MC_1_7_10 = 10,
    MC_1_8_9 = 11,
    MC_1_9_4 = 12,
    MC_1_10_2 = 13,
    MC_1_11_2 = 14,
    MC_1_12_2 = 15,
    MC_1_13_2 = 16,
    MC_1_14_4 = 17,
    MC_1_15_2 = 18,
    MC_1_16_1 = 19,
    MC_1_16_5 = 20,
    MC_1_17_1 = 21,
    MC_1_18_2 = 22,
    MC_1_19_2 = 23,
    MC_1_19_4 = 24,
    MC_1_20_6 = 25,
    MC_1_21_1 = 26,
    MC_1_21_3 = 27,
    MC_1_21_WD = 28,
}

#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum Dimension {
    NETHER = -1,
    OVERWORLD = 0,
    END = 1,
    UNDEF = 1000,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed() {
        let mut g = Generator::new(MCVersion::MC_1_21_WD);
        g.set_seed(Dimension::NETHER, 728201557363502228);
        let seed = g.get_seed();
        assert_eq!(seed, 728201557363502228);
    }

    #[test]
    fn test_dim() {
        let mut g = Generator::new(MCVersion::MC_1_21_WD);
        g.set_seed(Dimension::NETHER, 728201557363502228);
        let seed = g.get_dim();
        assert_eq!(seed, Dimension::NETHER);
    }
}
