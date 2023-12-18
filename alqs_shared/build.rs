fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/status.proto")?;
    tonic_build::compile_protos("proto/tables.proto")?;
    Ok(())
}
