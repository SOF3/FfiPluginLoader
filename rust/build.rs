use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header(
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
                .join("../resources/include/all.h")
                .display()
                .to_string(),
        )
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap();

    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bind.rs");
    bindings.write_to_file(out).unwrap();
}
