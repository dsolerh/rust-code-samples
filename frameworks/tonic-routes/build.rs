fn main() -> anyhow::Result<()> {
    tonic_build::compile_protos("proto/route_guide.proto")?;
    Ok(())
}
