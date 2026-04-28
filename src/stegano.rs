use anyhow::{Result, bail};
use image::Rgb;
use std::fs;

const START_MARKER: &str = "-----BEGIN PGP PUBLIC KEY BLOCK-----";
const END_MARKER: &str = "-----END PGP PUBLIC KEY BLOCK-----";

/// High-level orchestrator that tries different steganography detection methods.
pub fn extract(path: &str, print_garbage: bool) -> Result<String> {
    if let Ok(data) = extract_trailing_data(path) {
        println!("[+] Data found using EOF Appending analysis.");
        return Ok(data);
    }

    println!("[*] No trailing data found. Attempting deep LSB scan...");

    match extract_lsb(path, print_garbage) {
        Ok(data) => {
            println!("[+] Data found using LSB pixel analysis.");
            Ok(data)
        }
        Err(e) => bail!("Steganography detection failed: {}", e),
    }
}

/// Detects hidden data appended after the image's EOF (End of File) marker.
///
/// This is a common, simple technique where data is tacked onto the end of the file
/// byte-stream, which image viewers typically ignore.
pub fn extract_trailing_data(path: &str) -> Result<String> {
    let bytes = fs::read(path)?;
    let content = String::from_utf8_lossy(&bytes);

    if let Some(start_index) = content.find(START_MARKER) {
        if let Some(end_index) = content.find(END_MARKER) {
            return Ok(content[start_index..end_index + END_MARKER.len()].to_string());
        }
    }
    bail!("No trailing data markers found.")
}

/// Detects hidden data embedded within the Least Significant Bits of the image pixels.
///
/// This method iterates through every RGB pixel, extracts the lowest bit of each channel,
/// and reconstructs them into ASCII characters.
pub fn extract_lsb(path: &str, print_garbage: bool) -> Result<String> {
    let img = image::open(path)?;
    let rgb = img.to_rgb8();

    let mut msg = String::new();
    let mut bits: Vec<u8> = Vec::with_capacity(8);

    for Rgb([r, g, b]) in rgb.pixels() {
        for bit in [r & 1, g & 1, b & 1] {
            bits.push(bit);

            if bits.len() == 8 {
                let byte: u8 = bits.iter().fold(0u8, |acc, b| acc << 1 | b);
                bits.clear();

                if (32..=126).contains(&byte) || byte == 10 {
                    msg.push(byte as char);
                }

                // Optimization: Only check for markers once the message is large enough
                if msg.len() >= END_MARKER.len() {
                    let tail = &msg[msg.len() - END_MARKER.len()..];
                    if tail.contains(END_MARKER) {
                        if let Some(s_idx) = msg.find(START_MARKER) {
                            return Ok(msg[s_idx..].to_string());
                        }
                    }
                }
            }
        }
    }

    if print_garbage {
        println!(
            "[*] LSB analysis complete. No valid PGP key found, but extracted the following printable text:"
        );
        println!("--- BEGIN GARBAGE DATA ---");
        println!("{}", msg);
        println!("--- END GARBAGE DATA ---");
    }

    bail!("[!] No significant hidden data detected in LSB analysis.");
}
