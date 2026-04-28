

# Image Inspector 🕵️‍♂️

A high-performance digital forensics tool written in **Rust**, designed to analyze images for hidden metadata and steganographic data. 

This tool assists cybersecurity professionals in identifying concealed information within digital media.

<img src="https://media.wired.com/photos/594db1736b76bb7625b89e48/16:9/w_4991,h_2807,c_limit/hidden_data-01.png" width="40%">

## Features

- **Metadata Extraction**: Retrieves EXIF data including device make/model, software versions, and capture timestamps.
- **GPS Forensics**: Extracts geolocation coordinates with DMS (Degrees, Minutes, Seconds) formatting.
- **Steganography Detection**:
  - **LSB Pixel Analysis**: Performs bit-plane analysis on RGB channels to find data hidden within the least significant bits.
  - **EOF Appending Analysis**: Detects data injected after the End-of-File marker (e.g., JPEG EOI `FF D9`).

## Prerequisites

To build and run this tool, you need the **Rust toolchain** (v1.70+) installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Installation & Setup

1. Clone the repository:

```bash
git clone https://github.com/kill-ux/image-inspector.git
cd image-inspector
```

2. Build the project:

```bash
cargo build --release
```

3. Run the binary located at `./target/release/image-inspector`.

## Usage

### Command Line Interface

```bash
➜  debug git:(main) ✗  ./image-inspector --help
Welcome to Image Inspector

Usage: image-inspector [OPTIONS] <IMAGE>

Arguments:
  <IMAGE>  The image file to analyze

Options:
  -m, --metadata         Extract metadata from the image (geolocation, device info, date)
  -s, --steganography    Detect and extract hidden data using steganography techniques
  -o, --output <OUTPUT>  Specify the file name to save output (e.g., report.txt)
      --garbage          print the garbage data in the LSB analysis
  -h, --help             Print help
  -V, --version          Print version
(base) ➜  debug git:(main) ✗ 
```


### Examples

**Metadata Extraction:**

```bash
$> ./image-inspector -m -o metadata.txt image.jpeg
Lat/Lon: (13.731) / (-1.1373)
Device: Canon EOS 5D Mark III
Date: 2023-07-20 14:32:10
Data saved in metadata.txt
```

**Steganography Detection:**

```bash
$> ./image-inspector -s -o hidden_data.txt image.jpeg
-----BEGIN PGP PUBLIC KEY BLOCK-----
Version: 01
...
-----END PGP PUBLIC KEY BLOCK-----
Data saved in hidden_data.txt
```

**Full Forensic Report (Metadata + Steganography):**

```bash
$> ./image-inspector -m -s -o full_report.txt image.jpeg
```

## Ethical & Legal Considerations

> **IMPORTANT: READ CAREFULLY BEFORE USE.**

1. **Authorization**: NEVER analyze an image that you do not own or have explicit written permission to audit. Unauthorized access to data can violate privacy laws such as GDPR, CCPA, or the Computer Misuse Act.
2. **Privacy**: Metadata often contains sensitive information including exact locations. Handle all extracted data responsibly.
3. **Legal Implications**: Analyzing digital media without consent can be considered a criminal offense in many jurisdictions. This tool is intended for educational and authorized forensic purposes only.
4. **No Malice**: Do not use this tool to bypass security controls or facilitate unauthorized data exfiltration.

## Project Structure

```
image-inspector/
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src
    ├── bin
    │   └── image-inspector.rs
    ├── cli.rs
    ├── lib.rs
    ├── metadata.rs
    └── stegano.rs
```

## Technical Notes

- **LSB Logic**: Extracts the least significant bit from R, G, and B channels sequentially across all pixels (Big-Endian bit packing).
- **EOF Logic**: Scans for the JPEG End-of-File marker `FF D9` and reads any data appended after it.

###  res
- https://gchq.github.io/CyberChef/#recipe=Extract_LSB('R','G','B','','Row',0)
- https://stylesuxx.github.io/steganography/

> **Disclaimer**: The developers of Image Inspector are not responsible for any misuse of this tool. Use it at your own risk and within the legal boundaries of your jurisdiction.


