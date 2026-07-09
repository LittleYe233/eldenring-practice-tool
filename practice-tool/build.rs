fn main() {
    embed_resource::compile("./src/resources.rc", embed_resource::NONE)
        .manifest_optional()
        .unwrap();
    println!("cargo:rustc-cdylib-link-arg=/DEF:lib/no-logo/exports.def");
}
