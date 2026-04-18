use clap::Parser;
use sha2::{Sha256, Digest};
use std::fs;
use std::io::Write;

#[derive(Parser)]
#[command(name = "payload-gen", about = "Generate Android A/B OTA update payloads")]
struct Args {
    /// Path to source system.img
    #[arg(short)]
    source: String,

    /// Path to target system.img
    #[arg(short)]
    target: String,

    /// Output payload.bin file
    #[arg(short)]
    output: Option<String>,

    /// Also generate properties file
    #[arg(long)]
    with_properties: bool,
}

fn hash_file(path: &str) -> anyhow::Result<String> {
    let data = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    Ok(hex::encode(hasher.finalize()))
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let source_data = fs::read(&args.source)?;
    let target_data = fs::read(&args.target)?;

    let source_hash = hash_file(&args.source)?;
    let target_hash = hash_file(&args.target)?;

    let output_path = args.output.unwrap_or_else(|| "payload.bin".to_string());

    // Build minimal OTA payload (simplified)
    let mut payload = Vec::new();
    
    // Magic header
    payload.extend_from_slice(b"PAYLOAD");
    
    // Version
    payload.extend_from_slice(&[0, 0, 0, 4u8]); // version 4
    
    // Manifest size + offset
    let manifest = format!(
        "version=4\nold_sha1={}\nnew_sha1={}\nold_size={}\nnew_size={}",
        source_hash,
        target_hash,
        source_data.len(),
        target_data.len()
    );
    let manifest_bytes = manifest.as_bytes();
    payload.extend_from_slice(&(manifest_bytes.len() as u64).to_le_bytes());
    payload.extend_from_slice(manifest_bytes);

    // Delta (simplified: just store target)
    payload.extend_from_slice(&target_data);

    // Write payload
    let mut file = fs::File::create(&output_path)?;
    file.write_all(&payload)?;

    println!("✓ Payload generated: {}", output_path);
    println!("  Source: {} ({})", args.source, source_hash[..8].to_string());
    println!("  Target: {} ({})", args.target, target_hash[..8].to_string());
    println!("  Size:   {} bytes", payload.len());

    if args.with_properties {
        let props_path = format!("{}.properties", output_path);
        let props = format!(
            "version=4\nold_sha1={}\nnew_sha1={}\nold_size={}\nnew_size={}",
            source_hash, target_hash, source_data.len(), target_data.len()
        );
        fs::write(&props_path, props)?;
        println!("  Properties: {}", props_path);
    }

    Ok(())
}
