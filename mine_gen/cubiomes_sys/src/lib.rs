#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;


    #[rstest]
    fn test_internal() {
        unsafe {
            let _ = testGeneration();
            //assert_eq!(r, value);
        }
    }
}
