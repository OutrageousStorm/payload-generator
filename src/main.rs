use anyhow::Result;
use clap::Parser;
use std::fs;
use std::io::Write;
use sha2::{Sha256, Digest};

#[derive(Parser)]
#[command(name = "payload-gen")]
#[command(about = "Generate Android A/B update payloads")]
struct Args {
    #[arg(short, long)]
    system: String,

    #[arg(short, long)]
    boot: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(long)]
    version: Option<u32>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let output_path = args.output.unwrap_or_else(|| "payload.bin".to_string());

    println!("[*] Android Payload Generator");
    println!("  System: {}", args.system);
    println!("  Boot:   {}", args.boot);

    // Read images
    let system_data = fs::read(&args.system)?;
    let boot_data = fs::read(&args.boot)?;

    // Compute checksums
    let mut system_hasher = Sha256::new();
    system_hasher.update(&system_data);
    let system_hash = system_hasher.finalize();

    let mut boot_hasher = Sha256::new();
    boot_hasher.update(&boot_data);
    let boot_hash = boot_hasher.finalize();

    // Build minimal payload structure (simplified)
    let mut payload = Vec::new();
    payload.extend_from_slice(b"BRILLO");  // Magic

    // Metadata
    payload.extend_from_slice(&system_data.len().to_le_bytes()[..4]);
    payload.extend_from_slice(&boot_data.len().to_le_bytes()[..4]);
    payload.extend_from_slice(&system_hash[..]);
    payload.extend_from_slice(&boot_hash[..]);

    // Image data
    payload.extend_from_slice(&system_data);
    payload.extend_from_slice(&boot_data);

    // Write
    let mut file = fs::File::create(&output_path)?;
    file.write_all(&payload)?;

    println!("[+] Payload generated: {} ({} bytes)", output_path, payload.len());
    println!("  System hash:  {}", hex::encode(&system_hash[..]));
    println!("  Boot hash:    {}", hex::encode(&boot_hash[..]));

    Ok(())
}
