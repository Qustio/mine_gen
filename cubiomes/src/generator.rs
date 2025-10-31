#![warn(missing_docs)]

use super::*;
use cubiomes_sys::{
    applySeed, biomesToImage, genBiomes, getBiomeAt, getMinCacheSize, initBiomeColors,
    setupGenerator,
};
use std::{
    alloc::Layout,
    alloc::{alloc_zeroed, dealloc},
};

/// Generator struct that hold all noise layers required for biome generation
#[derive(Debug)]
pub struct Generator {
    generator: *mut cubiomes_sys::Generator,
}

impl Drop for Generator {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.generator as *mut u8,
                Layout::new::<cubiomes_sys::Generator>(),
            );
        }
    }
}

impl Generator {
    /// Allocates new generator
    pub fn new(version: MCVersion) -> Self {
        unsafe {
            let generator = alloc_zeroed(Layout::new::<cubiomes_sys::Generator>())
                as *mut cubiomes_sys::Generator;
            setupGenerator(generator, version as i32, 0);
            Self { generator }
        }
    }

    /// Sets seed
    pub fn set_seed(&mut self, dimention: Dimension, seed: u64) {
        unsafe {
            applySeed(self.generator, dimention.into(), seed);
        }
    }

    /// Returns seed value
    pub fn get_seed(&self) -> u64 {
        unsafe { (*self.generator).seed }
    }

    /// Returns dimention value
    pub fn get_dim(&self) -> Dimension {
        unsafe { Dimension::try_from((*self.generator).dim).unwrap() }
    }

    /// Calculates biome at `x` `y` `z` coordinates with `scale`
    /// 
    /// This function internally allocates cache with sx, sy, sz = 1,
    /// generates biomes, returns value and dealloc cache each call.
    /// This can result in unnecessary allocations and stutters.
    /// 
    /// Allocate cache with [Generator::alloc_cache] and use
    /// [Range::get_biome_at] instead.
    pub fn get_biome_at(&self, scale: i32, x: i32, y: i32, z: i32) -> Biome {
        unsafe { Biome::try_from(getBiomeAt(self.generator, scale, x, y, z)).unwrap() }
    }

    /// Allocates vec for storing all biomes data of specified range
    pub fn alloc_cache(&self, range: &mut Range) {
        unsafe {
            let size = getMinCacheSize(self.generator, range.scale, range.sx, range.sy, range.sz);
            range.cache = vec![0_i32; size]
        }
    }

    /// Generates biomes value of specified range
    /// 
    /// If range is missing cache  for biomes it panics
    pub fn gen_biomes(&self, range: &mut Range) -> Result<(), i32> {
        unsafe {
            let size = getMinCacheSize(self.generator, range.scale, range.sx, range.sy, range.sz);
            if range.cache.len() < size {
                panic!("Invalid cache");
            }
            match genBiomes(
                self.generator,
                range.cache.as_mut_ptr(),
                range.get_range()
            ) {
                0 => Ok(()),
                e => Err(e),
            }
        }
    }
}

/// Range for biome generation in range
/// 
/// This struct also hold cache for generated biomes
/// 
/// Before we can generate the biomes for an area or volume, we need
/// to define the bounds with a `Range` structure and allocate the
/// necessary buffer using `allocCache()`. The `Range` is described by a scale,
/// position, and size, where each cell inside the `Range` represents an amount
/// of `scale` blocks in the horizontal axes. The vertical direction is treated
/// separately and always follows the biome coordinate scaling of 1:4, except
/// for when `scale == 1`, in which case the vertical scaling is also 1:1.
#[derive(Debug)]
pub struct Range {
    /// The only supported values for `scale` are 1, 4, 16, 64, and
    /// (for the Overworld) 256. For versions up to 1.17, the scale
    /// is matched to an appropriate biome layer and will influence
    /// the biomes that can generate.
    pub scale: i32,
    /// Position of x coordinate
    pub x: i32,
    /// Position of y coordinate
    pub y: i32,
    /// Position of z coordinate
    pub z: i32,
    /// Scale of x axis
    pub sx: i32,
    /// Scale of y axis
    pub sy: i32,
    /// Scale of z axis
    pub sz: i32,
    cache: Vec<i32>,
}

impl Range {
    /// Creates new range with specified parameters and empty cache
    pub fn new(scale: i32, x: i32, y: i32, z: i32, sx: i32, sy: i32, sz: i32) -> Self {
        Self {
            scale,
            x,
            z,
            sx,
            sz,
            y,
            sy,
            cache: vec![],
        }
    }

    /// Saves biomes in cache to Vec<u8> RGB array
    pub fn biomes_to_image(&mut self, colors: &mut [[u8; 3]; 256]) -> Result<Vec<u8>, ()> {
        let p4c = 1;
        let (img_height, img_width) = (self.sx * p4c, self.sz * p4c);
        let mut rgb = vec![0u8; (img_height * img_width * 3) as usize];
        unsafe {
            biomesToImage(
                rgb.as_mut_ptr(),
                colors.as_mut_ptr(),
                self.cache.as_mut_ptr(),
                self.sx as u32,
                self.sz as u32,
                p4c as u32,
                2,
            );
        }
        Ok(rgb)
    }

    /// Get biome at specified coordinates
    pub fn get_biome_at(&self, x: i32, y: i32, z: i32) -> Result<Biome, ()> {
        let b = self
            .cache
            .get(
                ((y - self.y) * self.sx * self.sz + (z - self.z) * self.sx + (x - self.x))
                    as usize,
            )
            .unwrap();
        let b = Biome::try_from(*b).ok();
        match b {
            Some(b) => Ok(b),
            None => Err(()),
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

/// Initialize biome colors with default values
pub fn init_biome_colors() -> [[u8; 3]; 256] {
    let mut v = [[0u8; 3]; 256];
    unsafe {
        initBiomeColors(v.as_mut_ptr());
    }
    v
}
