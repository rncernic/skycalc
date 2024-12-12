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

// TODO Implement test
#![allow(dead_code, unused_variables)]

use core::option::Option;
use chrono::{DateTime, Datelike, Timelike,
             Utc, TimeZone, NaiveDateTime,
             NaiveDate, NaiveTime, FixedOffset, Local};
use serde::{Deserialize, Deserializer};

/// Time struct
///
/// Struct representing a date and time.
///
/// # Attributes
///
/// * `year` - Year
/// * `month` - Month
/// * `day` - Day
/// * `hour` - Hour
/// * `minute` - Minute
/// * `second` - Second
///
/// # Methods
///
/// * `new` - Create a new Time
/// * `now` - Get the current time
/// * `from_utc` - Create a new Time from a `DateTime<Utc>`
/// * `from_isot_str` - Create a new Time from an ISO 8601 string
/// * `from_jd` - Create a new Time from a Julian Date
/// * `from_mjd` - Create a new Time from a Modified Julian Date
/// * `to_jd` - Convert the Time to a Julian Date
/// * `to_mjd` - Convert the Time to a Modified Julian Date
/// * `to_gst` - Convert the Time to a Greenwich Sidereal Time
/// * `to_utc` - Convert the Time to a `DateTime<Utc>`
/// * `to_string` - Convert the Time to a string
///
/// # Examples
///
/// ```no_run
/// use time::Time;
///
/// let date = Time::new(2024, 11, 1, 0, 0, 0);
/// assert_eq!(date.year, 2024);
/// assert_eq!(date.month, 11);
/// assert_eq!(date.day, 1);
/// assert_eq!(date.hour, 0);
/// assert_eq!(date.minute, 0);
/// assert_eq!(date.second, 0);
/// ```
#[derive(Debug, Clone)]
pub struct Time {
    pub year: i64,
    pub month: u64,
    pub day: u64,
    pub hour: u64,
    pub minute: u64,
    pub second: u64
}

// Parse from a date-time string, defaulting to current time if empty
pub fn from_str_or_now(timestamp_str: &str) -> Time {
    // Define the possible date and time formats
    let date_formats = ["%Y-%m-%d", "%d/%m/%Y", "%d-%m-%Y", "%Y%m%d"];
    let time_formats = ["%H:%M:%S", "%H:%M"];

    let datetime_formats: Vec<String> = date_formats
        .iter()
        .flat_map(|&date_fmt| {
            time_formats.iter().map(move |&time_fmt| format!("{} {}", date_fmt, time_fmt))
        })
        .collect();

    if timestamp_str.is_empty() {
        return Time::default();
    }

    // Try parsing as a full date-time
    for format in &datetime_formats {
        if let Ok(datetime) = NaiveDateTime::parse_from_str(timestamp_str, format) {
            return Time {
                year: datetime.year() as i64,
                month: datetime.month() as u64,
                day: datetime.day() as u64,
                hour: datetime.hour() as u64,
                minute: datetime.minute() as u64,
                second: datetime.second() as u64,
            };
        }
    }

    // Try parsing just the date
    for format in &date_formats {
        if let Ok(date) = NaiveDate::parse_from_str(timestamp_str, format) {
            return Time {
                year: date.year() as i64,
                month: date.month() as u64,
                day: date.day() as u64,
                hour: 0,
                minute: 0,
                second: 0,
            };
        }
    }

    // Try parsing just the time
    for format in &time_formats {
        if let Ok(time) = NaiveTime::parse_from_str(timestamp_str, format) {
            let now = Utc::now().naive_utc(); // Get the current date
            return Time {
                year: now.year() as i64,
                month: now.month() as u64,
                day: now.day() as u64,
                hour: time.hour() as u64,
                minute: time.minute() as u64,
                second: time.second() as u64,
            };
        }
    }

    // If no format matched, default to current time
    Time::default()
}

// Custom deserialization for Time struct
impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Time, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(from_str_or_now(&value))
    }
}

impl Default for Time {
    fn default() -> Self {
        let now = Utc::now();
        Time {
            year: now.year() as i64,
            month: now.month() as u64,
            day: now.day() as u64,
            hour: now.hour() as u64,
            minute: now.minute() as u64,
            second: now.second() as u64,
        }
    }
}

impl Time {
    /// Create a new Time
    ///
    /// # Arguments
    ///
    /// * `year` - Year
    /// * `month` - Month
    /// * `day` - Day
    /// * `hour` - Hour
    /// * `minute` - Minute
    /// * `second` - Second
    ///
    /// # Returns
    ///
    /// * `Time` - A new Time object
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use time::Time;
    ///
    /// let date = Time::new(2024, 11, 1, 0, 0, 0);
    /// assert_eq!(date.year, 2024);
    /// assert_eq!(date.month, 11);
    /// assert_eq!(date.day, 1);
    /// assert_eq!(date.hour, 0);
    /// assert_eq!(date.minute, 0);
    /// assert_eq!(date.second, 0);
    /// ```
    pub fn new(year: i64, month: u64, day: u64, hour: u64, minute: u64, second: u64) -> Time {
        Time {
            year,
            month,
            day,
            hour,
            minute,
            second
        }
    }

    /// Create a new time using the current date and time
    ///
    /// # Returns
    ///
    /// * `Time` - A new Time object representing the current time
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use time::Time;
    ///
    /// let date = Time::now();
    /// assert_eq!(date.year, 2024);
    /// ```
    pub fn now() -> Time {
        let utc = Utc::now();
        Time {
            year: utc.year() as i64,
            month: utc.month() as u64,
            day: utc.day() as u64,
            hour: utc.hour() as u64,
            minute: utc.minute() as u64,
            second: utc.second() as u64,
        }
    }

    /// Create a new Time from a `DateTime<Utc>`
    ///
    /// # Arguments
    ///
    /// * `utc` - `DateTime<Utc>` object
    ///
    /// # Returns
    ///
    /// * `Time` - A new Time object
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use chrono::{DateTime, Datelike, Timelike, Utc, TimeZone};
    /// use time::Time;
    ///
    /// let utc = Utc.with_ymd_and_hms(2024, 11, 1, 0, 0, 0).unwrap();
    /// let date = Time::from_utc(utc);
    /// assert_eq!(date.year, utc.year());
    /// assert_eq!(date.month, utc.month());
    /// assert_eq!(date.day, utc.day());
    /// assert_eq!(date.hour, utc.hour());
    /// assert_eq!(date.minute, utc.minute());
    /// assert_eq!(date.second, utc.second());
    /// ```
    pub fn from_utc(utc: DateTime<Utc>) -> Time {
        Time {
            year: utc.year() as i64,
            month: utc.month() as u64,
            day: utc.day() as u64,
            hour: utc.hour() as u64,
            minute: utc.minute() as u64,
            second: utc.second() as u64,
        }
    }

    /// Create a new Time from an ISO 8601 string
    ///
    /// # Arguments
    ///
    /// * `isot` - ISO 8601 string
    ///
    /// # Returns
    ///
    /// * `Time` - A new Time object
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use time::Time;
    ///
    /// let isot = "2021-11-01T00:00:00Z";
    /// let date = Time::from_isot_str(isot);
    /// assert_eq!(date.year, 2024);
    /// assert_eq!(date.month, 11);
    /// assert_eq!(date.day, 1);
    /// assert_eq!(date.hour, 0);
    /// assert_eq!(date.minute, 0);
    /// assert_eq!(date.second, 0);
    /// ```
    pub fn from_isot_str(isot: &str) -> Time {
        let utc = DateTime::parse_from_rfc3339(isot).unwrap();
        Time {
            year: utc.year() as i64,
            month: utc.month() as u64,
            day: utc.day() as u64,
            hour: utc.hour() as u64,
            minute: utc.minute() as u64,
            second: utc.second() as u64
        }
    }

    /// Create a new Time from a Julian Date
    ///
    /// # Arguments
    ///
    /// * `jd` - Julian Date
    ///
    /// # Returns
    ///
    /// * `Time` - A new Time object
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use time::Time;
    ///
    /// let jd = 2460564.0569609753;
    /// let date = Time::from_jd(jd);
    /// println!("{:?}", date);
    /// assert_eq!(date.year, 2024);
    /// assert_eq!(date.month, 9);
    /// assert_eq!(date.day, 10);
    /// assert_eq!(date.hour, 13);
    /// assert_eq!(date.minute, 22);
    /// assert_eq!(date.second, 1);
    /// ```
    pub fn from_jd(jd: f64) -> Time {
        let temp = jd + 0.5;
        let z = temp.floor() as i32;
        let mut f = temp - z as f64;
        let mut a = z;
        if z > 2299161 {
            let alpha = ((z as f64 - 1867216.25) / 36524.25).floor() as i32;
            a = z + 1 + alpha - (alpha / 4);
        }
        let b = a + 1524;
        let c = ((b as f64 - 122.1) / 365.25).floor();
        let d = (365.25 * c) as i32;
        let e = ((b as f64 - d as f64) / 30.6001).floor() as i32;

        let day = b - d - ((30.6001 * e as f64) as i32) + f as i32;
        let month = if e < 14 { e - 1 } else { e - 13 };
        let year = if month > 2 { c - 4716.0 } else { c - 4715.0 };

        let hour = ((f * 24.0) as i32).abs();
        f = f - (hour as f64 / 24.0);
        let minute = ((f * 1440.0) as i32).abs();
        f = f - (minute as f64 / 1440.0);
        let second = ((f * 86400.0) as i32).abs();

        Time {
            year: year as i64,
            month: month  as u64,
            day: day  as u64,
            hour: hour  as u64,
            minute: minute  as u64,
            second: second  as u64
        }
    }

    /// Create a new Time from a Modified Julian Date
    ///
    /// # Arguments
    ///
    /// * `mjd` - Modified Julian Date
    ///
    /// # Returns
    ///
    /// * `Time` - A new Time object
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use time::Time;
    ///
    /// let mjd = 60636.5;
    /// let date = Time::from_mjd(mjd);
    /// assert_eq!(date.year == 2024);
    /// assert_eq!(date.month == 11);
    /// assert_eq!(date.day == 22);
    /// assert_eq!(date.hour == 12);
    /// assert_eq!(date.minute == 0);
    /// assert_eq!(date.second == 0);
    /// ```
    pub fn from_mjd(mjd: f64) -> Time {
        Time::from_jd(mjd + 2400000.5)
    }

    /// Convert the Time to a Julian Date
    ///
    /// # Returns
    ///
    /// * `f64` - Julian Date
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use time::Time;
    ///
    /// let date = Time::new(2024, 11, 22, 12, 0, 0);
    /// let jd = date.to_jd();
    /// assert_eq!(jd,  2460637.0);
    /// ```
    pub fn to_jd(&self) -> f64 {
        let year = self.year as f64;
        let month = self.month as f64;
        let day = self.day as f64;
        let hour = self.hour as f64;
        let minute = self.minute as f64;
        let second = self.second as f64;

        let jd = 367.0 * year - ((year + ((month + 9.0) / 12.0)).floor() * 7.0 / 4.0).floor()
            + ((275.0 * month) / 9.0).floor() + day + 1721013.5
            + ((hour + (minute / 60.0) + (second / 3600.0)) / 24.0);
        jd
    }

    /// Convert the Time to a Modified Julian Date
    ///
    /// # Returns
    ///
    /// * `f64` - Modified Julian Date
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use time::Time;
    ///
    /// let date = Time::new(2024, 11, 22, 12, 0, 0);
    /// let mjd = date.to_mjd();
    /// assert_eq!(mjd, 60636.5);
    /// ```
    pub fn to_mjd(&self) -> f64 {
        self.to_jd() - 2400000.5
    }

    /// Convert the Time to a Greenwich Sidereal Time
    ///
    /// # Returns
    ///
    /// * `f64` - Greenwich Sidereal Time in degrees
    ///
    /// # Examples
    ///
    /// ```no_run
    // TODO Change example
    /// use time::Time;
    ///
    /// let date = Time::new(2024, 8, 24, 6, 35, 34);
    /// let gst = date.to_gst();
    /// assert_eq!(gst, 71.92783272871748);
    /// ```
    pub fn to_gst(&self) -> f64 {
        let jd = self.to_jd();
        let t = (jd - 2451545.0) / 36525.0;
        let gst = 280.46061837 + 360.98564736629 * (jd - 2451545.0)
            + 0.000387933 * t * t
            - (t * t * t) / 38710000.0;
        gst % 360.0
    }

    /// Convert the Time to a `DateTime<Utc>`
    ///
    /// # Returns
    ///
    /// * `DateTime<Utc>` - `DateTime<Utc>` object
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use time::Time;
    /// use chrono::{DateTime, Utc};
    ///
    /// let date = Time::new(2024, 11, 22, 12, 30, 0);
    /// let utc = date.to_utc();
    /// let utc_str = utc.to_string();
    /// assert_eq!(utc_str, "2024-11-22 12:30:00 UTC");
    /// ```
    pub fn to_utc(&self) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(
            self.year as i32,
            self.month as u32,
            self.day as u32,
            self.hour as u32,
            self.minute as u32,
            self.second as u32,
        ).unwrap()
    }

    pub fn to_hhmm(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }

    pub fn to_yyyymmdd(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    pub fn to_short(&self) -> String {
        format!("{:02}-{:02} {:02}:{:02}", self.day, self.month, self.hour, self.minute)
    }

    // TODO Add local time
    /// Convert the Time to a string
    ///
    /// # Arguments
    ///
    /// * `format` - Format of the string
    ///
    /// # Returns
    ///
    /// * `String` - String representation of the Time
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use time::Time;
    ///
    /// let date = Time::new(2024, 11, 22, 12, 0, 0);
    /// let jd_str = date.to_string(Some("jd"));
    /// assert_eq!(jd_str, "2460637.0");
    ///
    /// let mjd_str = date.to_string(Some("mjd"));
    /// assert_eq!(mjd_str, "60636.5");
    ///
    /// let utc_str = date.to_string(Some("utc"));
    /// assert_eq!(utc_str, "2024-11-22 12:00:00 UTC");
    ///
    /// let isot_str = date.to_string(Some("isot"));
    /// assert_eq!(isot_str, "2024-11-22T12:00:00+00:00");
    /// ```
    pub fn to_string(&self, format: Option<&str>) -> String {
        if let Some(format) = format {
            if format == "jd" {
                // format!("Time JD: {}",self.to_jd().to_string())
                self.to_jd().to_string()
            } else if format == "mjd" {
                // format!("Time MJD: {}",self.to_mjd().to_string())
                self.to_mjd().to_string()
            } else if format == "utc" {
                // format!("Time UTC: {}",self.to_utc().to_string())
                self.to_utc().to_string()
            } else if format == "isot" {
                // format!("Time: {}",self.to_utc().to_rfc3339())
                self.to_utc().to_rfc3339()
            } else if format == "hhmm" {
                self.to_hhmm()
            } else if format == "yyyymmdd" {
                self.to_yyyymmdd()
            } else if format == "short"{
                self.to_short()
            } else {
                return "Invalid format".to_string();
            }
        } else {
            self.to_utc().to_string()
        }
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}-{:02}-{:02} {:02}:{:02}:{:02}",
               self.year, self.month, self.day, self.hour, self.minute, self.second)
    }
}