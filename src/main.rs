use std::fs;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use sha2::{Sha256, Digest};

#[derive(Parser)]
#[command(name = "payload-gen")]
#[command(about = "Generate Android A/B update payloads")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate full update payload
    Full {
        #[arg(short, long)]
        system: PathBuf,
        #[arg(short, long)]
        boot: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Generate delta payload (old → new)
    Delta {
        #[arg(long)]
        old_system: PathBuf,
        #[arg(long)]
        new_system: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Calculate SHA256 of image
    Hash {
        #[arg(short, long)]
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Full { system, boot, output } => {
            println!("Generating full payload...");
            println!("  System: {}", system.display());
            println!("  Boot:   {}", boot.display());
            
            let sys_hash = calculate_hash(&system);
            let boot_hash = calculate_hash(&boot);
            
            println!("  System SHA256: {}", sys_hash);
            println!("  Boot SHA256:   {}", boot_hash);
            println!("✓ Would generate: {}", output.display());
        },
        Commands::Delta { old_system, new_system, output } => {
            println!("Generating delta payload...");
            println!("  Old:    {}", old_system.display());
            println!("  New:    {}", new_system.display());
            
            let old_hash = calculate_hash(&old_system);
            let new_hash = calculate_hash(&new_system);
            
            println!("  Old SHA256: {}", old_hash);
            println!("  New SHA256: {}", new_hash);
            println!("✓ Would generate delta: {}", output.display());
        },
        Commands::Hash { file } => {
            let hash = calculate_hash(&file);
            println!("{}", hash);
        },
    }
}

fn calculate_hash(path: &PathBuf) -> String {
    let data = fs::read(path).expect("Failed to read file");
    let mut hasher = Sha256::new();
    hasher.update(&data);
    format!("{:x}", hasher.finalize())
}
