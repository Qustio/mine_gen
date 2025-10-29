use std::cell::RefCell;

use super::*;

#[test]
fn test_seed() {
    let mut g = Generator::new(MCVersion::MC_1_21_WD);
    g.set_seed(Dimension::Nether, 728201557363502228);
    let seed = g.get_seed();
    assert_eq!(seed, 728201557363502228);
}

#[test]
fn test_dim() {
    let mut g = Generator::new(MCVersion::MC_1_21_WD);
    g.set_seed(Dimension::Nether, 728201557363502228);
    let seed = g.get_dim();
    assert_eq!(seed, Dimension::Nether);
}

#[test]
fn test_get_biome_at() {
    let mut g = Generator::new(MCVersion::MC_1_21_WD);
    g.set_seed(Dimension::Nether, 728201557363502228);
    let biome = g.get_biome_at(1, 0, 64, 0);
    assert_eq!(biome, Biome::CrimsonForest);
}


#[test]
fn test_range() {
    let mut g = Generator::new(MCVersion::MC_1_21_WD);
    g.set_seed(Dimension::Overworld, 728201557363502228);
    let mut range = Range::new(
        1,
        -60,
        256,
        -60,
        120,
        1,
        120,
    );
    let c = g.alloc_cache(&mut range);
    g.gen_biomes(&mut range, c).unwrap();
    unsafe {
        // let c = range.cache.unwrap();
        // let cc = c.offset(100 as isize);
        // println!("cc: {}", *cc);
        cubiomes_sys::free(c as *mut std::os::raw::c_void);
    }
    
    // let biome = range.get_biome_at(120, 256,0).unwrap();
    // println!("biome:{:?}", biome);
}


fn test_save() {
    let mut g = RefCell::new(Generator::new(MCVersion::MC_1_21_WD));
    g.borrow_mut().set_seed(Dimension::Overworld, 728201557363502228);
    let mut range = RefCell::new(Range::new(
        1,
        -512,
        64,
        -512,
        1024,
        1,
        1024,
    ));
    g.borrow_mut().alloc_cache(&mut range.borrow_mut());
    //g.borrow_mut().gen_biomes(&mut range.borrow_mut()).unwrap();
    let mut colors = init_biome_colors();
    let image = range.borrow_mut().biomes_to_image(&mut colors);
    assert!(image.is_ok());
    // let handle = std::thread::spawn(move || {
    //     g.borrow_mut().alloc_cache(&mut range.borrow_mut());
    //     g.borrow_mut().alloc_cache(&mut range.borrow_mut());
    //     g.borrow_mut().gen_biomes(&mut range.borrow_mut()).unwrap();
    //     let mut colors = init_biome_colors();
    //     let image = range.borrow_mut().biomes_to_image(&mut colors);
    //     assert!(image.is_ok());
    //     println!("123")
    // });
    // handle.join();
    
    
}