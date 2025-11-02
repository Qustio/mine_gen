# WIP! mine_gen
Test environment for learning and implementing minecraft biome generation algorithm

This repository contains:
- cubiomes - rust crate with safe bindings of https://github.com/Cubitect/cubiomes library
- cubiomes-gtk - interactive gui for cubiomes powered by gtk4 and libadwaita
- cubiomes-sys - rust crate with unsafe bindings of cubiomes
- yarn - minecraft mappings for development

# TODO 📝
- [ ] Make proper gtk widget `mapwidget`
- [ ] Add dependencies to meson project
- [ ] Add translations
- [ ] Fix of map moving only per pixel by changing generation width and height values
- [ ] Make generation work faster by multithreading it
- [ ] Sync meson project version and authors with provided by cubiomes-gtkl/Cargo.toml. Something like this [Pika Backup](https://gitlab.gnome.org/World/pika-backup/-/blob/main/meson.build?ref_type=heads) does
- [ ] Fix flatpak build
- [ ] Remove yarn and add script to copy sources of minecraft since minecraft build is not using obfuscation anymore
- [ ] Reimplement cubiomes into own generator layer by layer

# Build
## cubiomes-gtk
Project supports both Cargo and Meson build system. Meson is used for flatpak and releases when Cargo used for development.
- Cargo
```
cargo build -p cubiomes-gtk
```
- Meson
```
meson setup _build
meson compile -C _build
meson install -C _build
```
## cubiomes
Clone repo with submodules

```git clone https://github.com/Qustio/mine_gen --recursive```

Build and test the lib

```cargo test --lib -p cubiomes```

# Wip GUI
<img alt="image" src="assets/guipreview.png" />
