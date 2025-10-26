#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// temporary unsafe impl for casting enums
impl TryFrom<i32> for BiomeID {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        unsafe {
            Ok(std::mem::transmute::<i32, BiomeID>(value))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_internal() {
        unsafe {
            let _ = testGeneration();
        }
    }
    #[rstest]
    #[case(2773094466218948097, -80, 70, 500, true, BiomeID::cherry_grove)]
	#[case(3663763793056879556, 10694, 70, 7993, false, BiomeID::lush_caves)]
    fn test_seed(
        #[case] seed: u64,
        #[case] x: i32,
        #[case] y: i32,
        #[case] z: i32,
        #[case] large_b: bool,
        #[case] biome: BiomeID
    ) {
        const scale: i32 = 1;
        unsafe {
            let mut g = std::mem::MaybeUninit::<Generator>::zeroed();
            let featuers = match large_b {
                true => LARGE_BIOMES as u32,
                false => 0,
            };
            setupGenerator(g.as_mut_ptr(), MCVersion::MC_1_21_WD as i32, featuers);
            applySeed(g.as_mut_ptr(), Dimension::DIM_OVERWORLD as i32, seed);
            let res = BiomeID::try_from(getBiomeAt(g.as_ptr(), scale, x, y, z));
            assert_eq!(res, Ok(biome))
        }
    }
}
