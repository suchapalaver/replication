fn main() {
    tonic_build::configure()
        .build_server(true)
        .compile(&["image.proto"], &["proto/"])
        .unwrap();
}
