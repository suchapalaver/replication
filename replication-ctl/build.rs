fn main() {
    tonic_build::configure()
        .build_server(true)
        .compile(&["replicate.proto"], &["proto/"])
        .unwrap();
}
