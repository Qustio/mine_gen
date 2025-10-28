use std::mem::MaybeUninit;
use cubiomes_sys::{allocCache, applySeed, genArea, genBiomes, getBiomeAt, setupGenerator};
use super::*;

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

    pub fn get_biome_at(&self, scale: i32, x: i32, y: i32, z: i32) -> Biome {
        unsafe {
            Biome::try_from(getBiomeAt(self.generator.as_ptr(), scale, x, y, z)).unwrap()
        }
    }

    pub fn alloc_cache(&self, range: &mut Range)
    {
        unsafe {
            let c = allocCache(self.generator.as_ptr(), range.get_range());
            range.cache = Some(c);
        }
    }

    pub fn gen_biomes(&self, range: &mut Range) -> Result<(), i32> {
        unsafe {
            if let Some(cache) = range.cache {
                match genBiomes(self.generator.as_ptr(), cache, range.get_range()) {
                    0 => Ok(()),
                    e => Err(e)
                }
            } else {
                Err(0)
            }
        }
    }
}


pub struct Range {
    pub scale: i32,
    pub x: i32,
    pub z: i32,
    pub sx: i32,
    pub sz: i32,
    pub y: i32,
    pub sy: i32,
    cache: Option<*mut i32>,
}

impl Range {
    pub fn new(
        scale: i32,
        x: i32,
        y: i32,
        z: i32,
        sx: i32,
        sy: i32,
        sz: i32
    ) -> Self {
        Self {
            scale,
            x,
            z,
            sx,
            sz,
            y,
            sy,
            cache: None,
        }
    }
    
    pub fn get_biome_at(&self, x: i32, y: i32, z: i32) -> Result<Biome, ()> {
        if let Some(cache) = self.cache {
            unsafe {
                let b = Biome::try_from(*cache.offset((
                    (y - self.y) * self.sx * self.sz +
                    (z - self.z) * self.sx +
                    (x - self.x)
                ) as isize));
                match b {
                    Ok(b) => Ok(b),
                    Err(_) => Err(()),
                }
            }
        } else {
            Err(())
        }
    }

    fn get_range(&self) -> cubiomes_sys::Range {
        cubiomes_sys::Range {
            scale: self.scale,
            x: self.x,
            z: self.z,
            sx: self.sx,
            sz: self.sz,
            y: self.y,
            sy: self.sy,
        }
    }
}