// The MIT License (MIT)
//
// Copyright (c) 2024 Ricardo Cernic
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

use std::str::FromStr;

pub fn format_dms(angle: f64, is_latitude: bool) -> String {
    let mut direction = "";
    if is_latitude {
        if angle >= 0.0 { direction = "N" } else { direction = "S" }
    } else {
        if angle >= 0.0 { direction = "E" } else { direction = "W" }
    }
    let d = angle.trunc().abs();
    let remainder = angle.abs() - d;
    let m = (remainder * 60.0).trunc();
    let s = ((remainder * 60.0) - m) * 60.0;
    format!("{}° {}' {:.1}\" {}", d, m, s, direction.to_string())
}

#[derive(Debug)]
pub struct Degrees {
    pub value: f64,
}

impl Degrees {
    /// Parses a string to a degree value within the specified range.
    /// The `min` and `max` parameters define the allowed range for degrees (e.g., -90 to 90 for latitude).
    pub fn from_str(input: String, min: f64, max: f64) -> Self {
        let input = input.trim();

        // First, try parsing as decimal degrees
        if let Ok(deg) = input.parse::<f64>() {
            if deg < min || deg > max {
                return Self { value: 0.0 };
            }
            return Self { value: deg };
        }

        // If not decimal, try parsing as DMS (Degrees, Minutes, Seconds)
        Self::parse_dms(input, min, max)
    }

    /// Parses a DMS (degrees, minutes, seconds) string into decimal degrees within the specified range.
    pub fn parse_dms(dms: &str, min: f64, max: f64) -> Self {
        let dms = dms.to_lowercase();
        let parts: Vec<&str> = dms.split(&['d','m','s','°', '\'','\"'][..]).collect();

        if parts.is_empty() {
            return Self { value: 0.0 };
        }

        let mut deg = 0.0;
        let mut min_val = 0.0;
        let mut sec = 0.0;
        let mut direction = 1.0;

        // Determine direction (N/S/E/W)
        if let Some(last_char) = dms.chars().last() {
            match last_char {
                'n' | 'e' => direction = 1.0,
                's' | 'w' => direction = -1.0,
                _ => {}
            }
        }

        // Parse degrees
        deg = parts[0].parse::<f64>().unwrap_or(0.0);

        // Parse minutes if available
        if parts.len() > 1 {
            min_val = parts[1].trim().parse::<f64>().unwrap_or(0.0);
        }

        // Parse seconds if available
        if parts.len() > 2 {
            sec = parts[2].trim().parse::<f64>().unwrap_or(0.0);
        }

        // Convert DMS to decimal degrees
        let decimal_deg = direction * (deg + min_val / 60.0 + sec / 3600.0);

        // Ensure the value is within the specified range
        if decimal_deg < min || decimal_deg > max {
            return Self { value: 0.0 };
        }

        Self { value: decimal_deg }
    }
}