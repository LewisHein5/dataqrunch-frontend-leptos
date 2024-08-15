fn main() -> Result<(), Box<dyn std::error::Error>> {
    //https://github.com/tokio-rs/prost/issues/541
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(&["proto/dataqrunch.proto"], &["proto/"])?;
    Ok(())
}
