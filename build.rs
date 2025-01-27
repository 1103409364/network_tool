fn main() {
    if let Err(e) =
        embed_resource::compile("src/assets/icon.rc", embed_resource::NONE).manifest_optional()
    {
        panic!("Failed to compile Windows resource: {}", e);
    }

    built::write_built_file().expect("Failed to acquire build-time information");
}
