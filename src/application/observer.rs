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

use chrono::{NaiveTime, Timelike};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use serde::ser::SerializeStruct;

pub fn degrees_from_str(input: &str, min: f64, max: f64) -> f64 {
    let input_trimmed = input.trim();

    // First, try parsing as decimal degrees
    if let Ok(deg) = input_trimmed.parse::<f64>() {
        if deg < min || deg > max {
            return 0.0;
        }

        return deg;
    }

    // If not decimal, try parsing as DMS (Degrees, Minutes, Seconds)
    parse_dms(input_trimmed, min, max)
}

// Parses a DMS (degrees, minutes, seconds) string into decimal degrees within the specified range.
pub fn parse_dms(dms: &str, min: f64, max: f64) -> f64 {
    let dms = dms.to_lowercase();
    let parts: Vec<&str> = dms.split(&['d', 'm', 's', '°', '\'', '\"', ' ', 'n', 'w', 'e'][..]).collect();

    if parts.is_empty() {
        return 0.0;
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
        return 0.0;
    }
    decimal_deg
}

// Parse timezone from string, e.g., "+05:30" or "-02:00" or "3.5"
pub fn timezone_from_str(input: &str) -> f64 {
    let input_trimmed = input.trim();

    // First, try parsing as decimal degrees
    if let Ok(deg) = input_trimmed.parse::<f64>() {
        return deg;
    }

    // If not decimal, try parsing as HM (Hours, Minutes)
    parse_hm(input_trimmed)
}

// Parses a HM (hour, minutes) string into decimal hours.
pub fn parse_hm(hm: &str) -> f64 {
    // Check for a leading '-' to handle negative times
    let is_negative = hm.starts_with('-');
    let time_part = if is_negative {
        &hm[1..] // Remove the '-' for parsing
    } else {
        hm
    };

    if let Ok(time) = NaiveTime::parse_from_str(time_part, "%H:%M") {
        let decimal_hours = time.hour() as f64 + time.minute() as f64 / 60.0;
        return if is_negative {
            -decimal_hours
        } else {
            decimal_hours
        };
    }

    0.0 // Return 0.0 if parsing fails
}

/// Observer struct
///
/// This struct represents an observer.
///
/// # Attributes
///
/// * `name` - Optional name of the observer or observatory
/// * `lat` - Latitude of the observer in degrees
/// * `lon` - Longitude of the observer in degrees
/// * `elevation` - Elevation of the observer in meters
///
/// # Methods
///
/// * `new` - Create a new Observer
/// * `local_sidereal_time` - Calculate the local sidereal time at a given time
/// * `to_string_decimal` - Convert the Observer to a string
///
/// # Examples
///
/// ```no_run

/// use observer::{Observer, Time};
///
/// let observer = Observer::location(-23.1, -46.5, 780, Some("Piracaia".to_string()));
/// let time = Time::new(2024, 11, 14, 12, 0, 0);
/// let lst = observer.local_sidereal_time(&time);
/// println!("Local sidereal time: {}", lst);
/// assert_eq!(lst, 315.09169822871746);
/// ```

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Observer {
    #[serde(default = "default_name")]
    pub name: Option<String>,
    #[serde(
        default = "default_lat",
        deserialize_with = "deserialize_latitude"
    )]
    pub latitude: f64,
    #[serde(
        default = "default_lon",
        deserialize_with = "deserialize_longitude"
    )]
    pub longitude: f64,
    #[serde(
        default = "default_elevation",
        deserialize_with = "deserialize_elevation"
    )]
    pub elevation: i64,
    #[serde(
        default = "default_timezone",
        deserialize_with = "deserialize_timezone"
    )]
    pub timezone: f64,
}

// Default value functions for Observer fields
pub fn default_name() -> Option<String> {
    Some("My observatory".to_string())
}

pub fn default_lat() -> f64 {
    0.0
}

pub fn default_lon() -> f64 {
    0.0
}

pub fn default_elevation() -> i64 {
    0
}

pub fn default_timezone() -> f64 {
    0.0 // Default timezone is UTC
}

// Custom deserializer for latitude
fn deserialize_latitude<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    Ok(degrees_from_str(&value, -180.0, 180.0))
}

impl Serialize for Observer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Observer", 5)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("latitude", &self.latitude)?;
        s.serialize_field("longitude", &self.longitude)?;
        s.serialize_field("elevation", &self.elevation)?;
        s.serialize_field("timezone", &self.timezone)?;
        s.end()
    }
}

// Custom deserializer for longitude
fn deserialize_longitude<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    Ok(degrees_from_str(&value, -180.0, 180.0))
}

fn deserialize_elevation<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<i64> = Option::deserialize(deserializer)?;
    // If the value is None (either missing or null), use the default value
    match value {
        Some(value) => Ok(value),
        None => Ok(default_elevation()), // Use the default value
    }
}

// Custom deserializer for timezone
fn deserialize_timezone<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    Ok(timezone_from_str(&value))
}

impl Observer {
    /// Create a new Observer with default values
    ///
    ///
    ///  # Returns
    ///
    /// * `Observer` - A new Observer object
    ///
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new Observer for a given location
    ///
    ///
    /// # Arguments
    ///
    /// * `lat` - Latitude of the observer in degrees
    /// * `lon` - Longitude of the observer in degrees
    /// * `elevation` - Elevation of the observer in meters
    /// * `name` - Optional name of the observer
    ///
    /// # Returns
    ///
    /// * `Observer` - A new Observer object based on the given location
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use observer::Observer;
    ///
    /// let observer = Observer::new(-23.1, -46.5, 780, Some("Piracaia".to_string()));
    /// assert_eq!(observer.lat, -23.1);
    /// assert_eq!(observer.lon, -46.5);
    /// assert_eq!(observer.elevation, 780);
    /// assert_eq!(observer.name, Some("Piracaia".to_string()));
    /// println!("{}", observer.to_string());
    /// ```
    ///
    /// ```no_run
    /// use observer::Observer;
    ///
    /// let observer = Observer::location("23d 06m S", "46d 30m W", 780, None);
    /// assert_eq!(observer.lat, -23.1);
    /// assert_eq!(observer.lon, -46.5);
    /// assert_eq!(observer.elevation, 780);
    /// assert_eq!(observer.name, None);
    /// println!("{}", observer.to_string());
    /// ```
    pub fn location(
        name: Option<String>,
        lat: &str,
        lon: &str,
        elevation: i64,
        tz: &str,
    ) -> Observer {
        //(i64, u64)) -> Observer {
        let latitude = degrees_from_str(lat, -90.0, 90.0);
        let longitude = degrees_from_str(lon, -180.0, 180.0);
        let timezone = timezone_from_str(tz);
        Observer {
            name,
            latitude,
            longitude,
            elevation,
            timezone,
        }
    }

    /// Convert the Observer to a string
    ///
    /// # Returns
    ///
    /// * `String` - A string representing the observer with latitude and longitude im decimal degrees
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use observer::Observer;
    ///
    /// let observer = Observer::location("-23.1", "-46.5", 780, Some("Piracaia".to_string()));
    /// assert_eq!(observer.to_string(), "Name: Piracaia, Lat: -23.1, Lon: -46.5, Elevation: 780");
    /// println!("{}", observer.to_string_decimal());
    /// ```
    ///
    /// ```no_run
    /// use observer::Observer;
    ///
    /// let observer = Observer::location("-23.1", "-46.5", 780, None);
    /// assert_eq!(observer.to_string(), "Name: My observatory, Lat: -23.1, Lon: -46.5, Elevation: 780");
    /// println!("{}", observer.to_string());
    /// ```
    pub fn to_string_decimal(&self) -> String {
        if let Some(name) = &self.name {
            return format!(
                "{}, lat: {}, lon: {}, elevation: {} m, tz: {:3.2} h",
                name, self.latitude, self.longitude, self.elevation, self.timezone
            );
        }
        format!(
            "My observatory, lat: {}, lon: {}, elevation: {} m, tz: {:3.2} h",
            self.latitude, self.longitude, self.elevation, self.timezone
        )
    }

    // TODO Create to_string_dms
    pub fn to_string_dms(&self) -> String {
        // if let Some(name) = &self.name {
        //     return format!("{}, lat: {}, lon: {}, elevation: {} m, tz: {:03}:{:02} h",
        //                    name, self.latitude, self.longitude, self.elevation, self.timezone.0, self.timezone.1)
        // }
        // format!("My observatory, lat: {}, lon: {}, elevation: {} m, tz: {:03}:{:02} h",
        //         self.latitude, self.longitude, self.elevation, self.timezone.0, self.timezone.1)
        "".to_string()
    }
}

impl fmt::Display for Observer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(
                f,
                "{}, lat: {}, lon: {}, elevation: {} m, tz: {:3.2} h",
                name, self.latitude, self.longitude, self.elevation, self.timezone
            )
        } else {
            write!(
                f,
                "My observatory, lat: {}, lon: {}, elevation: {} m, tz: {:3.2} h",
                self.latitude, self.longitude, self.elevation, self.timezone
            )
        }
    }
}

