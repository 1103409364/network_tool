fn main() {
    embed_resource::compile("src/assets/icon.rc", embed_resource::NONE)
        .manifest_optional()
        .unwrap();
}
