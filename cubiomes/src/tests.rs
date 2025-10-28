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
    g.alloc_cache(&mut range);
    g.gen_biomes(&mut range).unwrap();
    let biome = range.get_biome_at(120, 256,0).unwrap();
    println!("biome:{:?}", biome);
    
}