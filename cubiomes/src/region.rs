use crate::*;

pub struct Region {
    data256: Option<[u8; 3]>,
    data128: Option<[u8; 3*4]>,
    data64: Option<[u8; 3*16]>,
    data32: Option<[u8; 3*64]>,
    data16: Option<[u8; 3*256]>,
    data8: Option<[u8; 3*1024]>,
    data4: Option<[u8; 3*4096]>,
    data2: Option<[u8; 3*16384]>,
    data1: Option<[u8; 3*65536]>,
    x: i32,
    y: i32,
}

impl Region {
    pub fn fill_from_range(&self, range: &Range) {
        // check if a region in range bounds
        let region_x = 256 * self.x;
        let region_y = 256 * self.y;
        let range_x = range.x * range.scale;
        let range_y = range.y * range.scale;
        let range_sx = range.sx * range.scale;
        let range_sy = range.sy * range.scale;

        if 
            region_x >= range_x && 
            region_y >= range_y && 
            range_x + range_sx >= region_x + 256 &&
            range_y + range_sy >= region_y + 256{

        }
    }
}
