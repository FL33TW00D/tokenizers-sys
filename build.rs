use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    std::fs::remove_dir_all("./bindings").ok();
    std::fs::create_dir_all("./bindings").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .unwrap()
        .write_to_file("bindings/ctokenizers.h");
}
