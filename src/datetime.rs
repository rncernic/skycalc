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

// TODO remove before release
#![allow(dead_code, unused_variables)]

use chrono::{Datelike, NaiveDate, Utc};
use crate::earth::nutation;
use crate::utils::{constrain_360, cosd};

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Date {
    pub year: i64,
    pub month: i64,
    pub day: i64
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Time {
    pub hour: i64,
    pub min: i64,
    pub sec: f64
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
    pub frac_day: f64,
    pub day_decimal: f64,
    pub jd: f64,
    pub jd2000: f64,
    pub jd2000_century: f64
}

pub fn format_hm(y: i64, m: i64, d: i64, h: i64, min: i64, s: i64) -> String {
    format!("{:02}-{:02} {:02}:{:02}",
            d,
            m,
            h,
            min)
}

fn parse_date(input: &str) -> Result<NaiveDate, String> {
    // Define the possible date formats
    let formats = ["%Y-%m-%d", "%d/%m/%Y", "%d-%m-%Y", "%Y%m%d"];

    // Try parsing the input with each format
    for format in formats.iter() {
        if let Ok(date) = NaiveDate::parse_from_str(input, format) {
            return Ok(date);
        }
    }

    // If none of the formats matched, return an error
    Err(format!("Failed to parse date: {}", input))
}

pub fn naive_to_date(date_str: String) -> (i64, i64, i64) {
    match parse_date(&date_str) {
        Ok(date) => (date.year() as i64, date.month() as i64, date.day() as i64),
        Err(err) => (Utc::now().naive_utc().date().year() as i64,
                     Utc::now().naive_utc().date().month() as i64,
                     Utc::now().naive_utc().date().day() as i64)
    }
}

impl Date {
    pub fn new(year: i64, month: i64, day: i64) -> Self {
        Self { year, month, day }
    }

    pub fn date(&self) {
        println!("{}-{}-{}", self.year, self.month, self.day)
    }
}

impl Time {
    pub fn new(hour: i64, min: i64, sec: f64) -> Self {
        Self { hour, min, sec }
    }

    pub fn time(&self) {
        println!("{}-{}-{}", self.hour, self.min, self.sec)
    }
}

impl DateTime {
    pub fn new() -> Self{ Self::default() }
    pub fn ymd_hms(self, year: i64, month: i64, day: i64, hour: i64, min: i64, sec: f64) -> Self {
        let frac_day = (hour as f64 + min as f64 / 60.0 + sec as f64 / 3600.0) / 24.0;
        let day_decimal = day as f64 + frac_day;
        let jd = jd_from_date(year, month, day_decimal);
        let jd2000= jd2000_from_date(year, month, day_decimal);
        let jd2000_century = jd2000_century_from_jd(jd);
        Self {
            date: Date::new(year, month, day),
            time: Time::new(hour, min, sec),
            frac_day,
            day_decimal,
            jd,
            jd2000,
            jd2000_century,
            ..self
        }
    }

    pub fn julian(self, jd: f64) -> Self {
        let (year, month, day_decimal) = ymd_from_jd(jd);
        let jd2000= jd2000_from_date(year, month, day_decimal);
        let jd2000_century = jd2000_century_from_jd(jd);
        let frac_day = day_decimal.fract();
        let day = (day_decimal - frac_day) as i64;
        let (hour, min, sec) = h_to_hms(frac_day);
        Self {
            date: Date::new(year, month, day),
            time: Time::new(hour, min, sec),
            frac_day,
            day_decimal,
            jd,
            jd2000,
            jd2000_century,
            ..self
        }
    }
}

/// Julian Day from a given date. Takes into account Julian and Gregorian calendars.
///
/// Based on Meeus, J. - Astronomical Algorithms, 1991 - Chapter 7 (ISBN 0-943396-35-2)
///
/// # Arguments
///
///     year (i64): year
///     month (i64): month
///     day (f64): fractional day (0.5 = noon)
///
/// # Return
///
///     (f64): julian day
///
/// # Examples
///
/// ```
/// let answer = julian::jd_from_date(2000, 1, 1.5);
///
/// assert_eq!(answer, 2_451_545.0_f64);
/// ```
pub fn jd_from_date(year: i64, month: i64, day: f64) -> f64 {
    let (y, m) = if month > 2 {(year, month)} else {(year - 1, month + 12)};
    let a: f64 = (y as f64 / 100.0).floor();
    // if date in julian calendar, i.e, prior to 1582.10.15 assign zero to b
    let b: f64 = if y < 1582 || (m < 10 && y == 1582) || (day < 15.0 && m == 10 && y == 1582 )  {
        // julian calendar
        0_f64
    } else {
        // gregorian calendar
        2.0 - a + (a / 4.0).floor()
    };
    let jd: f64 = (365.25 * (y + 4716) as f64).floor() +
        (30.600_1 * (m + 1) as f64).floor() + day + b - 1524.5;
    jd
}

pub fn mjd_from_date(year: i64, month: i64, day: f64) -> f64 {
    jd_from_date(year, month, day) - 2_400_000.5
}

pub fn jd2000_from_date(year: i64, month: i64, day: f64) -> f64 {
    jd_from_date(year, month, day) - 2_451_545.0
}

pub fn jd2000_century_from_date(year: i64, month: i64, day: f64) -> f64 {
    jd2000_from_date(year, month, day) / 36_525.0
}

pub fn jd2000_century_from_jd(jd: f64) -> f64 {
    (jd - 2_451_545.0) / 36525.0
}

pub fn ymd_from_jd(jd: f64) -> (i64, i64, f64) {
    let j = jd + 0.5;
    let z = j as i64;
    let f = j.fract();
    let mut a: i64 = 0;
    if z < 2_229_161 {
        a = z;
    } else {
        let alpha = (( z as f64 - 1_867_216.25) / 36_524.25 ) as i64;
        a = z + 1 + alpha - (alpha / 4);
    }
    let b = a + 1_524;
    let c = (( b as f64 - 122.1) / 365.25) as i64;
    let d = (365.25 * c as f64) as i64;
    let e = (( b - d) as f64 / 30.600_1) as i64;
    let day: f64 = (b - d - (30.600_1 * e as f64) as i64) as f64 + f;
    let mut month: i64 = 0;
    if e < 14 {
        month = e - 1;
    } else {
        month = e - 13;
    }
    let mut year: i64 = 0;
    if month > 2 {
        year = c - 4_716;
    } else {
        year = c - 4_715;
    }
    (year, month, day)
}

pub fn ymd_hms_from_jd(jd: f64) -> (i64, i64, i64, i64, i64, i64) {
    let (y, m, dd) = ymd_from_jd(jd);
    let total_seconds = dd * 86_400.0;
    let d = total_seconds as i64 / 86400;
    let remainder = total_seconds % 86400.0;

    let h = remainder as i64 / 3600;
    let remainder = remainder % 3600.0;

    let min = remainder as i64 / 60;
    let s = remainder as i64 % 60;

    (y, m, d, h, min, s)
}

// Result in degrees
pub fn mean_sidereal_time_greenwich(y: i64, m:i64, d: f64) -> f64 {
    let t = jd2000_century_from_date(y, m, d);
    let j = jd2000_from_date(y, m, d);
    constrain_360(280.460_618_37 + 360.985_647_366_29 * j + 0.000_387_933 * t * t -
        t * t * t / 38_710_000.0)
}

// Calculate the apparent sidereal time at Greenwich for the specified date.
//
// Result in degrees.
pub fn apparent_sidereal_time_greenwich(y: i64, m:i64, d: f64) -> f64 {
    let mm = mean_sidereal_time_greenwich(y, m, d);
    let t = jd2000_century_from_date(y, m, d);
    let (mut delta_phi, delta_eps, eps) = nutation(t);
    delta_phi *= 15.0; // convert to degrees
    //delta_eps *= 15.0; // convert to degrees
    //cosd(mm + delta_phi * (eps + delta_eps))
    constrain_360(mm + (delta_phi * cosd(eps)))
}

// GMST in degress
pub fn gmst(jd: f64) -> f64 {
    let t = (jd - 2_451_545.0) / 36_525.0;
    let st = (280.460_618_37 + 360.985_647_366_29 * (jd - 2_451_545.0)
        + 0.000_387_933 * t * t -
        t * t * t / 38_710_000.0) % 360.0;
    //if st < 0.0 { st += 360.0 }
    constrain_360(st)
}

// LST in degrees
pub fn local_sidereal_time(jd: f64, longitude: f64) -> f64 {
    constrain_360(gmst(jd) - longitude)
}

pub fn h_to_hms(v: f64) -> (i64, i64, f64) {
    let h = (v * 24.0) as i64;
    let m = ((v * 24.0 - h as f64) * 60.0) as i64;
    let s= (v * 24.0 - h as f64 - m as f64 / 60.0) * 60.0;
    (h, m, s)
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use crate::datetime::{gmst,
                          mean_sidereal_time_greenwich,
                          apparent_sidereal_time_greenwich,
                          jd_from_date,
                          mjd_from_date,
                          jd2000_century_from_date,
                          ymd_from_jd,
                          h_to_hms,
                          local_sidereal_time,
                          Date,
                          Time,
                          DateTime};

    #[test]
    fn test_date() {
        let d = Date::new(2024, 8, 19);
        assert_eq!(d, Date{year: 2024, month: 8, day: 19});
    }

    #[test]
    fn test_time() {
        let t = Time::new(13, 55, 45.8);
        assert_eq!(t, Time{hour: 13, min: 55, sec: 45.8});
    }

    #[test]
    fn test_datetime_from_yms_hms() {
        let dt = DateTime::new()
            .ymd_hms(2024, 9, 13, 12, 0, 0.0);
        assert_eq!((dt.date.year, dt.date.month, dt.date.day,
                    dt.time.hour, dt.time.min, dt.time.sec), (2024, 9, 13, 12, 0, 0.0));
    }

    #[test]
    fn test_datetime_from_jd() {
        let dt = DateTime::new()
            .julian( 2460566.5);
        let dt1 = DateTime::new()
            .julian(0.0);
        assert_eq!((dt.date.year, dt.date.month, dt.date.day,
                    dt.time.hour, dt.time.min, dt.time.sec), (2024, 9, 13, 0, 0, 0.0));
        assert_eq!((dt1.date.year, dt1.date.month, dt1.date.day,
                    dt1.time.hour, dt1.time.min, dt1.time.sec), (-4712, 1, 1, 12, 0, 0.0));
    }

    #[test]
    fn test_jd_from_date() {
        // test jd_from_date
        assert_eq!(jd_from_date(2000, 1, 1.5), 2_451_545.0_f64);
        assert_eq!(jd_from_date(1987, 1, 27.0), 2_446_822.5_f64);
        assert_eq!(jd_from_date(1987, 6, 19.5), 2_446_966.0_f64);
        assert_eq!(jd_from_date(1957, 10, 4.81), 2_436_116.31_f64);
        assert_eq!(jd_from_date(1900, 1, 1.0), 2_415_020.5_f64);
        assert_eq!(jd_from_date(1600, 1, 1.0), 2_305_447.5_f64);
        assert_eq!(jd_from_date(1600, 12, 31.0), 2_305_812.5_f64);
        assert_eq!(jd_from_date(837, 4, 10.3), 2_026_871.8_f64);
        assert_eq!(jd_from_date(333, 1, 27.5), 1842713.0);
        assert_eq!(jd_from_date(-1000, 7, 12.5), 1_356_001.0_f64);
        assert_eq!(jd_from_date(-1000, 2, 29.0), 1_355_866.5_f64);
        assert_eq!(jd_from_date(-1001, 8, 17.9), 1_355_671.4_f64);
        assert_eq!(jd_from_date(-4712, 1, 1.5), 0.0_f64);
    }

    #[test]
    fn test_mjd_from_date() {
        // test mjd_from_date
        assert_eq!(mjd_from_date(1858, 11, 17.0), 0.0_f64 )
    }

    #[test]
    fn test_jd2000_century_from_date() {
        assert_approx_eq!(jd2000_century_from_date(1987, 4, 10.0), -0.127_296_372_348, 0.000_000_000_001);
    }

    #[test]
    fn test_ymd_from_jd() {
        assert_eq!(ymd_from_jd(0.0), (-4712, 1, 1.5));
        assert_eq!(ymd_from_jd(1_356_001.0_f64), (-1000, 7, 12.5));
        assert_eq!(ymd_from_jd(2_415_020.5_f64), (1900, 1, 1.0));
    }

    #[test]
    fn test_h_to_hms() {
        assert_eq!(h_to_hms(0.5), (12, 0, 0.0));
    }

    #[test]
    fn test_gmst() {
        assert_approx_eq!(gmst(2_443_825.5) / 15.0, 3.450386, 0.000_001);
        assert_approx_eq!(gmst(2_446_895.5), mean_sidereal_time_greenwich(1987, 4, 10.0), 1e-6);
    }

    #[test]
    fn test_mean_sidereal_time() {
        assert_approx_eq!(mean_sidereal_time_greenwich(1978, 11, 13.0) / 15.0, 3.450386, 0.000_001);
    }

    #[test]
    fn test_mean_sidereal_time_greenwich() {
        assert_approx_eq!(mean_sidereal_time_greenwich(1987, 4, 10.80625), 128.737_873_4, 1e-6);
    }

    #[test]
    fn test_apparent_sidereal_time_greenwich() {
        assert_approx_eq!(apparent_sidereal_time_greenwich(1987, 4, 10.0), 197.678_714, 1e-6);
    }

    #[test]
    fn test_lst() {
        assert_approx_eq!(local_sidereal_time(jd_from_date(1987,4,10.80625), 77.065555556), 51.672_317_688, 1e-6);
    }
}

