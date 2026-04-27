// use anyhow::{Result, bail};
// use image::Rgb;

// /// Steganography analysis module for extracting hidden data from images.
// pub fn extract(path: &str) -> Result<String> {
//     let img = image::open(path)?;

//     // collect LSBs from RGB channels
//     let rgb = img.to_rgb8();
//     let mut bits = Vec::new();
//     let mut len = 0;
//     for Rgb([r, g, b]) in rgb.pixels() {
//         bits.push(r & 1);
//         bits.push(g & 1);
//         bits.push(b & 1);
//         len += 3;
//     }
//     println!("[*] Extracted {} bits from the image", len); // 14747856

//     // group evry 8 bits into a byte
//     let mut msg = String::new();
//     for (i, chunk) in bits.chunks(8).enumerate() {
//         if i % 100000 == 0 {
//             println!("[*] Processed {} bits...", i);
//         }
//         if chunk.len() < 8 {
//             break;
//         }

//         let byte = chunk.iter().fold(0, |acc, bit| acc << 1 | bit);

//         if byte.is_ascii() {
//             msg.push(byte as char);
//         }
//         if msg.contains("-----END PGP PUBLIC KEY BLOCK-----") {
//             break;
//         }
//     }

//     if msg.contains("-----BEGIN PGP PUBLIC KEY BLOCK-----") {
//         Ok(msg)
//     } else {
//         bail!("No hidden data found using LSB steganography")
//     }
// }


use anyhow::{bail, Result};
use image::Rgb;

pub fn extract(path: &str) -> Result<String> {
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

                if byte.is_ascii() {
                    msg.push(byte as char);
                }

                if msg.len() >= 40 {
                    let tail = &msg[msg.len() - 40..];
                    if tail.contains("-----END PGP PUBLIC KEY BLOCK-----") {
                        break;
                    }
                }
            }
        }
    }
    println!("msg: {}", msg);


    if msg.contains("-----BEGIN PGP PUBLIC KEY BLOCK-----") {
        Ok(msg)
    } else {
        bail!("No hidden data found using LSB steganography")
    }
}