use clap::{
    Parser,
    builder::{
        Styles,
        styling::{AnsiColor, Effects},
    },
};


/// Welcome to Image Inspector
#[derive(Parser, Debug)]
#[command(
    name = "image-inspector",
    arg_required_else_help = true,
    color = clap::ColorChoice::Always,
    styles = MY_STYLES,
    version = "0.1.0",
)]
pub struct Cli {
    /// Extract metadata from the image (geolocation, device info, date)
    #[arg(short, long)]
    pub metadata: bool,

    /// Detect and extract hidden data using steganography techniques
    #[arg(short, long)]
    pub steganography: bool,

    /// Specify the file name to save output (e.g., report.txt)
    #[arg(short, long)]
    pub output: Option<String>,

    /// print the garbage data in the LSB analysis
    #[arg(long)]
    pub garbage: bool,

    /// The image file to analyze
    pub image: String,
}

const MY_STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());
