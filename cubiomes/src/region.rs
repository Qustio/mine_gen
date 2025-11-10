use ndarray::s;

use crate::*;

#[derive(Debug, Default)]
pub struct Region {
    data256: Option<Biome>,
    data64: Option<[Biome; 16]>,
    data16: Option<[Biome; 256]>,
    data4: Option<[Biome; 4096]>,
    data1: Option<[Biome; 65536]>,
    img256: Option<[u8; 3]>,
    pub img4: Option<[u8; 3*4096]>,
    x: i32,
    y: i32,
}

impl Region {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            ..Default::default()
        }
    }
    pub fn col(&mut self, colors: [[u8; 3]; 256])
    {
        if let Some(data4) = self.data4 {
            let mut img4 = [0; 3*4096];
            for (b, i) in data4.iter().zip(img4.chunks_exact_mut(3)) {
                i.copy_from_slice(&colors[b.clone() as usize]);
            }
            self.img4 = Some(img4);
        }
    }

    pub fn fill_from_range(&mut self, range: &Range) {
        // check if a region in range bounds
        let region_x = 256 * self.x;
        let region_y = 256 * self.y;
        let scale = range.scale.clone() as i32;
        let range_x = range.x * scale;
        let range_y = range.z * scale;
        let range_sx = range.sx * scale;
        let range_sy = range.sz * scale;

        if  region_x >= range_x &&
            region_y >= range_y &&
            range_x + range_sx >= region_x + 256 &&
            range_y + range_sy >= region_y + 256 {
            let arr = ndarray::ArrayView3::from_shape(
                (range.sx as usize, range.sy as usize, range.sz as usize), 
                &range.cache
            ).unwrap();
            
            // need a slice for (region_x..region_x+256, 0, region_y..region_y+256)
            // but its in blocks so we convert it to range.scale
            // (region_x/range.scale..region_x+256/range.scale, 0, region_y/range.scale..region_y+256/range.scale)

            let s = arr
                .slice(s![(
                    region_x/scale)-range.x..(region_x+256)/scale-range.x,
                    0,
                    region_y/scale-range.z..(region_y+256)/scale-range.z
                ])
                .map(|b| Biome::try_from(*b).unwrap());

			println!("biomes: {:?}", s);
            let s = s.as_slice()
                .unwrap();

            
            match range.scale {
                Scale::S256 => self.data256 = Some(s[0]),
                Scale::S64 => self.data64 = Some(s.try_into().unwrap()),
                Scale::S16 => self.data16 = Some(s.try_into().unwrap()),
                Scale::S4 => self.data4 = Some(s.try_into().unwrap()),
                Scale::S1 => self.data1 = Some(s.try_into().unwrap()),
            }
            //println!("array length: {:?}", &s);
            // let biome = Biome::try_from(bb).unwrap();
            
            //range.cache.as_chunks()
            // let b = self
            // .cache
            // .get(
            //     ((y - self.y) * self.sx * self.sz + (z - self.z) * self.sx + (x - self.x))
            //         as usize,
            // )
            // .unwrap();
        }
    }
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_region_fill(){
        let mut g = Generator::new(MCVersion::MC_1_21_WD);
        g.set_seed(Dimension::Overworld, 728201557363502228);
        let mult = 4;
        let mut range = Range::new(
            Scale::S64,
            -4/mult,
            256,
            -4/mult,
            270/mult,
            1,
            270/mult
        );
        g.alloc_cache(&mut range);
        g.gen_biomes(&mut range).unwrap();
        let mut region = Region::new(0, 0);
        region.fill_from_range(&range);
        let biome = range.get_biome_at(0, 256,0).unwrap();
        println!("Biome: {biome:?}")
        //assert_eq!(biome, Biome::TallBirchForest);
    }
}
