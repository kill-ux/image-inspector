use anyhow::Result;
use clap::Parser;
use image_inspector::{Cli, metadata, stegano};

fn main() -> Result<()> {
    let cli = Cli::parse();

    if !cli.metadata && !cli.steganography {
        eprintln!("Please specify at least one analysis option: --metadata or --steganography");
        std::process::exit(1);
    }
    let mut results = String::new();

    // --- Metadata mode ---
    if cli.metadata {
        println!("[*] Extracting metadata from: {}", cli.image);
        match metadata::extract(&cli.image) {
            Ok(data) => {
                println!("{}", data);
                results.push_str(&data);
                results.push_str("\n");
            }
            Err(e) => eprintln!("[-] Metadata extraction failed: {}", e),
        }
    }

    // --- Steganography mode ---
    if cli.steganography {
        println!("[*] Extracting hidden data from: {}", cli.image);
        match stegano::extract(&cli.image) {
            Ok(data) => {
                println!("{}", data);
                results.push_str(&data);
                results.push_str("\n");
            }
            Err(e) => eprintln!("[-] Steganography extraction failed: {}", e),
        }
    }

    // --- Save output ---
    if let Some(output_file) = &cli.output {
        match std::fs::write(output_file, &results) {
            Ok(_) => println!("[+] Data saved in {}", output_file),
            Err(e) => eprintln!("[-] Failed to save output: {}", e),
        }
    }

    Ok(())
}
