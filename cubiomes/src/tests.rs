use std::{env, path::PathBuf};

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
        Scale::S256,
        -60,
        256,
        -60,
        120,
        1,
        120,
    );
    g.alloc_cache(&mut range);
    range.x = 1;
    range.z = 1;
    range.sx = 2;
    range.sz = 2;
    g.gen_biomes(&mut range).unwrap();    
    let biome = range.get_biome_at(1, 256,1).unwrap();
    //assert_eq!(biome, Biome::TallBirchForest);
}

#[test]
fn test_save() {
    let mut g = Generator::new(MCVersion::MC_1_21_WD);
    g.set_seed(Dimension::Overworld, 728201557363502228);
    let mut range = Range::new(
        Scale::S1,
        -512,
        64,
        -512,
        1024,
        1,
        1024,
    );
    g.alloc_cache(&mut range);
    g.gen_biomes(&mut range).unwrap();
    let mut colors = init_biome_colors();
    let image = range.biomes_to_image(&mut colors);
    assert!(image.is_ok());
    let path = PathBuf::from("./img.png");
    let image = range.save_image(&path, image.unwrap());
    assert!(image.is_ok());
    image.map(|_| std::fs::remove_file(&path).unwrap()).unwrap();
}