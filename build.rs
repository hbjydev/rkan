fn main() {
    println!("cargo:rerun-if-changed=src/validation/data/ckan.schema.json");
}
