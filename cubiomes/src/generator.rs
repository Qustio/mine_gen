use std::{alloc::Layout, mem::MaybeUninit, pin::Pin};
use cubiomes_sys::{allocCache, applySeed, biomesToImage, genBiomes, getBiomeAt, getMinCacheSize, initBiomeColors, setupGenerator};
use super::*;

#[derive(Debug)]
pub struct Generator {
    generator: MaybeUninit<cubiomes_sys::Generator>,
}

impl Generator {
    pub fn new(version: MCVersion) -> Self {
        let mut generator = MaybeUninit::<cubiomes_sys::Generator>::zeroed();
        unsafe {
            setupGenerator(generator.as_mut_ptr(), version as i32, 0);
        }
        let generator = generator;
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

    pub fn alloc_cache(&self, range: &mut Range) -> *mut i32
    {
        unsafe {
            let size = getMinCacheSize(
                self.generator.as_ptr(),
                range.scale,
                range.sx,
                range.sy,
                range.sz
            );
            allocCache(self.generator.as_ptr(), range.get_range())
            //range.cache = Some(c);
        }
    }

    pub fn gen_biomes(&self, range: &mut Range, cache: *mut i32) -> Result<(), i32> {
        unsafe {
            match genBiomes(
                self.generator.as_ptr(), 
                cache, 
                range.get_range()
            ) {
                0 => Ok(()),
                e => Err(e)
            }
        }
    }
}

#[derive(Debug)]
pub struct Range {
    pub scale: i32,
    pub x: i32,
    pub z: i32,
    pub sx: i32,
    pub sz: i32,
    pub y: i32,
    pub sy: i32,
    pub cache: Option<*mut i32>,
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
    
    pub fn biomes_to_image(&mut self, colors: &mut[[u8; 3]; 256]) -> Result<Vec<u8>, ()> {
        let p4c = 1;
        let (img_height, img_width) = (self.sx * p4c, self.sz * p4c);
        let mut rgb = vec![0u8; (img_height * img_width * 3) as usize];
        unsafe {
            biomesToImage(
                rgb.as_mut_ptr(),
                colors.as_mut_ptr(),
                self.cache.unwrap(),
                self.sx as u32,
                self.sz as u32,
                p4c as u32,
                2
            );
        }
        Ok(rgb)
    }

    pub fn get_biome_at(&self, x: i32, y: i32, z: i32) -> Result<Biome, ()> {
        unsafe {
            let b = self.cache.unwrap().offset((
                (y - self.y) * self.sx * self.sz +
                (z - self.z) * self.sx +
                (x - self.x)
            ) as isize);
            let b = Biome::try_from(*b).ok();
            match b {
                Some(b) => Ok(b),
                None => Err(()),
            }
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

pub fn init_biome_colors() -> [[u8; 3]; 256] {
    let mut v = [[0u8; 3]; 256];
    unsafe {
        initBiomeColors(v.as_mut_ptr());
    }
    v
}