//! Color parsing and formatting shared across commands.

use anyhow::{bail, Result};
use image::Rgba;

/// Parse a CSS-ish color string into an RGBA pixel.
///
/// Accepts `#rgb`, `#rrggbb`, `#rrggbbaa` (with or without the leading `#`),
/// the keyword `transparent`, and the common names `white`/`black`.
pub fn parse_rgba(input: &str) -> Result<Rgba<u8>> {
    let s = input.trim();
    match s.to_ascii_lowercase().as_str() {
        "transparent" | "none" => return Ok(Rgba([0, 0, 0, 0])),
        "white" => return Ok(Rgba([255, 255, 255, 255])),
        "black" => return Ok(Rgba([0, 0, 0, 255])),
        _ => {}
    }
    let hex = s.strip_prefix('#').unwrap_or(s);
    let parse = |slice: &str| -> Result<u8> {
        u8::from_str_radix(slice, 16).map_err(|_| anyhow::anyhow!("invalid hex in color '{input}'"))
    };
    let rgba = match hex.len() {
        3 => {
            let r = parse(&hex[0..1].repeat(2))?;
            let g = parse(&hex[1..2].repeat(2))?;
            let b = parse(&hex[2..3].repeat(2))?;
            Rgba([r, g, b, 255])
        }
        6 => Rgba([
            parse(&hex[0..2])?,
            parse(&hex[2..4])?,
            parse(&hex[4..6])?,
            255,
        ]),
        8 => Rgba([
            parse(&hex[0..2])?,
            parse(&hex[2..4])?,
            parse(&hex[4..6])?,
            parse(&hex[6..8])?,
        ]),
        _ => bail!("unrecognized color '{input}' (use #rgb, #rrggbb, #rrggbbaa, or a name)"),
    };
    Ok(rgba)
}

/// Format an RGB(A) pixel as a `#rrggbb` hex string (alpha dropped).
pub fn to_hex(px: Rgba<u8>) -> String {
    format!("#{:02x}{:02x}{:02x}", px[0], px[1], px[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_named_and_hex() {
        assert_eq!(parse_rgba("white").unwrap(), Rgba([255, 255, 255, 255]));
        assert_eq!(parse_rgba("#000").unwrap(), Rgba([0, 0, 0, 255]));
        assert_eq!(parse_rgba("ff8800").unwrap(), Rgba([255, 136, 0, 255]));
        assert_eq!(parse_rgba("#112233aa").unwrap(), Rgba([17, 34, 51, 170]));
        assert_eq!(parse_rgba("transparent").unwrap(), Rgba([0, 0, 0, 0]));
    }

    #[test]
    fn rejects_garbage() {
        assert!(parse_rgba("#zzzzzz").is_err());
        assert!(parse_rgba("blurple").is_err());
    }

    #[test]
    fn hex_roundtrip() {
        assert_eq!(to_hex(Rgba([255, 136, 0, 255])), "#ff8800");
    }
}
