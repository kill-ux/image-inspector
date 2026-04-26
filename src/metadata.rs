use std::{fs::File, io::BufReader};

use anyhow::Result;
use exif::{In, Reader, Tag, Value};

/// Extract metadata from the specified image file.
pub fn extract(image: &str) -> Result<String> {
    let fd = File::open(image)?;
    let mut bufreader = BufReader::new(fd);
    let exif = Reader::new().read_from_container(&mut bufreader)?;

    let mut output = String::new();

    // --- GPS ---
    let lat = parse_gps(exif.get_field(Tag::GPSLatitude, In::PRIMARY));
    let lon = parse_gps(exif.get_field(Tag::GPSLongitude, In::PRIMARY));
    let lat_ref = get_string(&exif, Tag::GPSLatitudeRef);
    let lon_ref = get_string(&exif, Tag::GPSLongitudeRef);

    if let Some(la) = lat
        && let Some(lo) = lon
    {
        let la = apply_ref(la, &lat_ref, "S");
        let lo = apply_ref(lo, &lon_ref, "W");
        output.push_str(&format!("Lat/Lon: ({:.4}) / ({:.4})\n", la, lo));
    } else {
        output.push_str("Lat/Lon: Not found\n");
    }

    // --- Device ---
    let make = get_string(&exif, Tag::Make).unwrap_or("Unknown".to_string());
    let model = get_string(&exif, Tag::Model).unwrap_or("Unknown".to_string());
    output.push_str(&format!("Device: {} {}\n", make, model));
    dbg!(make);
    dbg!(model);

    for ele in exif.fields() {
        println!(
            "{} {} {}",
            ele.ifd_num,
            ele.tag.to_string(),
            ele.display_value().with_unit(&exif)
        );
    }
    Ok(output)
}

fn get_string(exif: &exif::Exif, tag: Tag) -> Option<String> {
    exif.get_field(tag, In::PRIMARY)
        .map(|f| f.display_value().to_string())
}

// Apply N/S or E/W reference to make the coordinate negative if needed
fn apply_ref(val: f64, reference: &Option<String>, negative_ref: &str) -> f64 {
    match reference {
        Some(r) if r.trim() == negative_ref => -val,
        _ => val,
    }
}

fn parse_gps(field: Option<&exif::Field>) -> Option<f64> {
    if let Some(field) = field {
        if let Value::Rational(vals) = &field.value
            && vals.len() == 3
        {
            let degrees = vals[0].to_f64();
            let minutes = vals[1].to_f64();
            let seconds = vals[2].to_f64();
            return Some(degrees + minutes / 60. + seconds / 3600.);
        }
    }
    None
}
