use std::{fs::File, io::BufReader};
use anyhow::Result;
use exif::{In, Reader, Tag, Value, Rational};

pub fn extract(image: &str) -> Result<String> {
    let fd = File::open(image)?;
    let mut bufreader = BufReader::new(fd);
    let exif = Reader::new().read_from_container(&mut bufreader)?;

    let mut output = String::new();
    output.push_str(&format!("\n--- Report for {} ---\n", image));

    // --- 1. Device Info (Cleaned) ---
    let make = get_clean_string(&exif, Tag::Make).unwrap_or_else(|| "Generic".to_string());
    let model = get_clean_string(&exif, Tag::Model).unwrap_or_else(|| "Device".to_string());
    let software = get_clean_string(&exif, Tag::Software).unwrap_or_else(|| "Unknown".to_string());
    
    output.push_str(&format!("Device:   {} {}\n", make, model));
    output.push_str(&format!("Software: {}\n", software));

    // --- 2. Date & Time ---
    let date = get_clean_string(&exif, Tag::DateTime).unwrap_or_else(|| "Unknown".to_string());
    output.push_str(&format!("Captured: {}\n", date));

    // --- 3. GPS Coordination (Decimal & DMS) ---
    let lat_field = exif.get_field(Tag::GPSLatitude, In::PRIMARY);
    let lon_field = exif.get_field(Tag::GPSLongitude, In::PRIMARY);
    let lat_ref = get_clean_string(&exif, Tag::GPSLatitudeRef).unwrap_or_else(|| "N".to_string());
    let lon_ref = get_clean_string(&exif, Tag::GPSLongitudeRef).unwrap_or_else(|| "E".to_string());

    if let (Some(la_f), Some(lo_f)) = (lat_field, lon_field) {
        if let (Value::Rational(lat_data), Value::Rational(lon_data)) = (&la_f.value, &lo_f.value) {
            
            // Calculate Decimals
            let lat_dec = to_decimal(lat_data, &lat_ref);
            let lon_dec = to_decimal(lon_data, &lon_ref);
            
            // Format DMS strings: 32°05'11.8"N
            let lat_dms = format_dms(lat_data, &lat_ref);
            let lon_dms = format_dms(lon_data, &lon_ref);

            output.push_str(&format!("Location: {} {}\n", lat_dms, lon_dms));
            output.push_str(&format!("Decimal:  {:.6}, {:.6}\n", lat_dec, lon_dec));
            output.push_str(&format!("Maps Link: https://www.google.com/maps?q={},{}\n", lat_dec, lon_dec));
        }
    } else {
        output.push_str("Location: No GPS data found\n");
    }

    Ok(output)
}

// --- Helper Functions ---

/// Cleans strings to remove quotes added by display_value()
fn get_clean_string(exif: &exif::Exif, tag: Tag) -> Option<String> {
    exif.get_field(tag, In::PRIMARY).map(|f| {
        let s = f.display_value().to_string();
        s.trim_matches('"').trim().to_string()
    })
}

fn to_decimal(rationals: &[Rational], ref_str: &str) -> f64 {
    let d = rationals[0].to_f64();
    let m = rationals[1].to_f64();
    let s = rationals[2].to_f64();
    let res = d + m / 60.0 + s / 3600.0;
    if ref_str == "S" || ref_str == "W" { -res } else { res }
}

fn format_dms(rationals: &[Rational], ref_str: &str) -> String {
    let d = rationals[0].to_f64() as u32;
    let m = rationals[1].to_f64() as u32;
    let s = rationals[2].to_f64();
    format!("{:02}°{:02}'{:04.1}\"{}", d, m, s, ref_str)
}