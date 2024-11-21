use version_check::is_feature_flaggable;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(nightly)");

    if is_feature_flaggable() == Some(true) {
        println!("cargo:rustc-cfg=nightly");
    }
}
