fn main() -> Result<(), Box<dyn std::error::Error>> {
    //https://github.com/tokio-rs/prost/issues/541
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        .compile(&["proto/dataqrunch.proto"], &["proto/"])?;
    Ok(())
}
