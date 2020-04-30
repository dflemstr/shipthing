fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/shipthing/v1/player_api.proto")?;
    Ok(())
}
