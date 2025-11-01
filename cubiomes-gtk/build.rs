fn main() {
    #[cfg(not(feature = "meson"))]
    glib_build_tools::compile_resources(
        &["src"],
        "src/cubiomesgtk.gresource.xml",
        "cubiomesgtk.gresource"
    );
}