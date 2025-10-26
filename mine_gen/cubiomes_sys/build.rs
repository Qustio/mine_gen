use std::env;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .files([
            "cubiomes/finders.c",
           	"cubiomes/generator.c",
           	"cubiomes/layers.c",
           	"cubiomes/biomenoise.c",
           	"cubiomes/biomes.c",
           	"cubiomes/noise.c",
           	"cubiomes/util.c",
           	"cubiomes/quadbase.c",
            "cubiomes/tests.c",
        ])
        .flag_if_supported("-Dmain=disabled_main")
        .compile("cubiomes");
    println!("cargo:rustc-link-search=native={}", "cubiomes");
    println!("cargo:rustc-link-lib=static=cubiomes");
    let bindings = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .headers([
                "cubiomes/finders.h",
               	"cubiomes/generator.h",
               	"cubiomes/layers.h",
               	"cubiomes/biomenoise.h",
               	"cubiomes/biomes.h",
               	"cubiomes/noise.h",
               	"cubiomes/util.h",
               	"cubiomes/quadbase.h",
                "cubiomes/tests.c"
            ])
            .allowlist_file("./cubiomes/.*")
            .allowlist_file("cubiomes/tests.c")
            .translate_enum_integer_types(true)
            .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: true })
            .allowlist_recursively(true)
            .layout_tests(false)
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
        bindings
            .write_to_file(out_path)
            .expect("Couldn't write bindings!");
}
