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
// TODO Create tests
#![allow(dead_code, unused_variables)]

use crate::application::{
    environment::Environment,
    observer::Observer,
    time::Time,
    transformations::equatorial_to_altaz,
};
use crate::utils::utils::{
    cosd,
    cross_horizon,
    sind,
    two_point_interpolation
};
use libm::atan2;
use std::cmp::PartialEq;
use std::f64::consts::PI;
//https://en.wikipedia.org/wiki/Sunrise_equation#Complete_calculation_on_Earth
//https://astrogreg.com/

#[derive(Debug)]
pub enum SunRS {
    NeverRise,
    NeverSet,
}

pub fn sun_position_from_jd(jd: f64) -> (f64, f64) {
    let n = jd - 2_451_545.0;
    let mut l = (280.460 + 0.985_647_4 * n) % 360.0;
    let mut g = ((357.528 + 0.985_600_3 * n) % 360.0).to_radians();
    if l < 0.0 {
        l += 360.0
    };
    if g < 0.0 {
        g += 2. * PI
    };
    let lambda = (l + 1.915 * g.sin() + 0.020 * (2. * g).sin()).to_radians();
    let bet = 0.0;
    let eps = (23.439 - 0.000_000_4 * n).to_radians();
    let mut ra = atan2(eps.cos() * lambda.sin(), lambda.cos());
    let dec = (eps.sin() * lambda.sin()).asin();
    if ra < 0.0 {
        ra += 2. * PI
    };
    (ra.to_degrees(), dec.to_degrees())
}

pub fn sun_position_from_ymd(y: i64, m: u64, d: u64, h: u64, min: u64, s: u64) -> (f64, f64) {
    let date = Time::new(y, m, d, h, min, s);
    sun_position_from_jd(date.to_jd())
}

//lat and dec in degrees
//hour angle in degrees
pub fn sun_hour_angle(lat: f64, dec: f64) -> f64 {
    let numerator = sind(-0.833_33_f64) - sind(lat) * sind(dec);
    let denominator = cosd(lat) * cosd(dec);
    if denominator > 1. {
        999.0
    }
    // never sets - always above horizon
    else if denominator < -1. {
        -999.0
    }
    // never rises - always below horizon
    else {
        (numerator / denominator).acos().to_degrees()
    }
}

pub fn sun_alt_az_from_jd(lat: f64, lon: f64, ra: f64, dec: f64, jd: f64) -> (f64, f64) {
    let date = Time::from_jd(jd);
    equatorial_to_altaz(
        lat,
        lon,
        ra,
        dec,
        date.year,
        date.month,
        date.day,
        date.hour,
        date.minute,
        date.second,
    )
}

pub fn sun_alt_az_grid_utc(
    lat: f64,
    lon: f64,
    jd_start: f64,
    jd_end: f64,
    num_points: usize,
) -> Vec<(f64, f64, f64)> {
    // create a null grid vector with 3 columns and num_points+1 rows
    let mut grid: Vec<(f64, f64, f64)> = Vec::new();
    let inc = (jd_end - jd_start) / num_points as f64;
    for i in 0..=num_points {
        let jd = jd_start + inc * i as f64;
        let (ra, dec) = sun_position_from_jd(jd);
        let mut date = Time::from_jd(jd);
        let (alt, az) = equatorial_to_altaz(
            lat,
            lon,
            ra,
            dec,
            date.year,
            date.month,
            date.day,
            date.hour,
            date.minute,
            date.second,
        );
        grid.push((jd, alt, az));
    }
    grid
}

pub fn sunrise_utc_grid(lat: f64, lon: f64, jd: f64, horizon: f64, tz: f64) -> Result<f64, SunRS> {
    let num_points = 288;
    let target_night_start = (jd + 0.5).floor() + tz / 24.0;
    let target_night_end = target_night_start + 1.0;
    let sun = sun_alt_az_grid_utc(lat, lon, target_night_start, target_night_end, num_points);
    let v = cross_horizon(sun, horizon, true);
    if v.is_empty() {
        Err(SunRS::NeverRise)
    } else {
        Ok(two_point_interpolation(
            v[0].0, v[0].2, v[0].1, v[0].3, horizon,
        ))
    }
}

pub fn next_sunrise_utc(
    lat: f64,
    lon: f64,
    jd: f64,
    horizon: f64,
    tz: f64,
    max_days: u32,
) -> Result<f64, SunRS> {
    let mut current_jd = jd;
    for _ in 0..max_days {
        // Limit to 2 days of iterations
        match sunrise_utc_grid(lat, lon, current_jd, horizon, tz) {
            Ok(sunrise) => return Ok(sunrise),
            Err(SunRS::NeverRise) => current_jd += 1.0, // Skip to the next day
            Err(e) => return Err(e),
        }
    }
    Err(SunRS::NeverRise) // Return error if no sunrise is found within the range
}

pub fn previous_sunrise_utc(
    lat: f64,
    lon: f64,
    jd: f64,
    horizon: f64,
    tz: f64,
    max_days: u32,
) -> Result<f64, SunRS> {
    let mut current_jd = jd - 1.0;
    for _ in 0..max_days {
        // Limit to 2 days of iterations
        match sunrise_utc_grid(lat, lon, current_jd, horizon, tz) {
            Ok(sunrise) => return Ok(sunrise),
            Err(SunRS::NeverRise) => current_jd -= 1.0, // Skip to the next day
            Err(e) => return Err(e),
        }
    }
    Err(SunRS::NeverRise) // Return error if no sunrise is found within the range
}

pub fn nearest_sunrise_utc(
    lat: f64,
    lon: f64,
    jd: f64,
    horizon: f64,
    tz: f64,
    max_days: u32,
) -> Result<f64, SunRS> {
    let next = next_sunrise_utc(lat, lon, jd, horizon, tz, max_days); // max_days window
    let previous = previous_sunrise_utc(lat, lon, jd, horizon, tz, max_days); // max_days window

    match (next, previous) {
        (Ok(next_sunrise), Ok(previous_sunrise)) => {
            // Compare which sunrise is closer to `jd`
            if (next_sunrise - jd).abs() < (jd - previous_sunrise).abs() {
                Ok(next_sunrise)
            } else {
                Ok(previous_sunrise)
            }
        }
        (Ok(next_sunrise), Err(_)) => Ok(next_sunrise), // Only next is valid
        (Err(_), Ok(previous_sunrise)) => Ok(previous_sunrise), // Only previous is valid
        (Err(next_err), Err(prev_err)) => Err(next_err), // Neither is valid, return an error
    }
}

pub fn sunset_utc_grid(lat: f64, lon: f64, jd: f64, horizon: f64, tz: f64) -> Result<f64, SunRS> {
    let num_points = 288;
    let target_night_start = (jd + 0.5).floor() + tz / 24.0;
    let target_night_end = target_night_start + 1.0;
    let sun = sun_alt_az_grid_utc(lat, lon, target_night_start, target_night_end, num_points);
    let v = cross_horizon(sun, horizon, false);
    if v.is_empty() {
        Err(SunRS::NeverSet)
    } else {
        Ok(two_point_interpolation(
            v[0].0, v[0].2, v[0].1, v[0].3, horizon,
        ))
    }
}

pub fn next_sunset_utc(
    lat: f64,
    lon: f64,
    jd: f64,
    horizon: f64,
    tz: f64,
    max_days: u32,
) -> Result<f64, SunRS> {
    let mut current_jd = jd;
    for _ in 0..max_days {
        // Limit to 2 days of iterations
        match sunset_utc_grid(lat, lon, current_jd, horizon, tz) {
            Ok(sunset) => return Ok(sunset),
            Err(SunRS::NeverSet) => current_jd += 1.0, // Skip to the next day
            Err(e) => return Err(e),
        }
    }
    Err(SunRS::NeverSet) // Return error if no sunset is found within the range
}

pub fn previous_sunset_utc(
    lat: f64,
    lon: f64,
    jd: f64,
    horizon: f64,
    tz: f64,
    max_days: u32,
) -> Result<f64, SunRS> {
    let mut current_jd = jd - 1.0;
    for _ in 0..max_days {
        // Limit to 2 days of iterations
        match sunset_utc_grid(lat, lon, current_jd, horizon, tz) {
            Ok(sunset) => return Ok(sunset),
            Err(SunRS::NeverSet) => current_jd -= 1.0, // Skip to the next day
            Err(e) => return Err(e),
        }
    }
    Err(SunRS::NeverSet) // Return error if no sunset is found within the range
}

pub fn nearest_sunset_utc(
    lat: f64,
    lon: f64,
    jd: f64,
    horizon: f64,
    tz: f64,
    max_days: u32,
) -> Result<f64, SunRS> {
    let next = next_sunset_utc(lat, lon, jd, horizon, tz, max_days);
    let previous = previous_sunset_utc(lat, lon, jd, horizon, tz, max_days);

    match (next, previous) {
        (Ok(next_sunset), Ok(previous_sunset)) => {
            if (next_sunset - jd).abs() < (jd - previous_sunset).abs() {
                Ok(next_sunset)
            } else {
                Ok(previous_sunset)
            }
        }
        (Ok(next_sunset), Err(_)) => Ok(next_sunset), // Only next is valid
        (Err(_), Ok(previous_sunset)) => Ok(previous_sunset), // Only previous is valid
        (Err(next_err), Err(prev_err)) => Err(next_err), // Neither is valid, return an error
    }
}

#[derive(Debug, Clone)]
pub struct Sun<'a> {
    pub observer: &'a Observer,
    pub time: &'a Time,
    pub environment: &'a Environment,
}

#[derive(Debug)]
pub enum TwilightType {
    RiseSet,
    CivilTwilight,
    NauticalTwilight,
    AstronomicalTwilight,
}

impl TwilightType {
    pub(crate) fn angle(&self) -> f64 {
        match self {
            TwilightType::RiseSet => -0.8333,
            TwilightType::CivilTwilight => -6.0,
            TwilightType::NauticalTwilight => -12.0,
            TwilightType::AstronomicalTwilight => -18.0,
        }
    }

    fn description(&self) -> &str {
        match self {
            TwilightType::RiseSet => "Sunrise/Sunset",
            TwilightType::CivilTwilight => "Civil Twilight",
            TwilightType::NauticalTwilight => "Nautical Twilight",
            TwilightType::AstronomicalTwilight => "Astronomical Twilight",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RiseSetType {
    Nearest,
    Next,
    Previous,
}

impl RiseSetType {
    pub fn to_string(&self) -> &str {
        match self {
            RiseSetType::Nearest => "Nearest",
            RiseSetType::Next => "Next",
            RiseSetType::Previous => "Previous",
        }
    }
}

impl<'a> Sun<'a> {
    pub fn new(observer: &'a Observer, time: &'a Time, environment: &'a Environment) -> Sun<'a> {
        Sun {
            observer,
            time,
            environment,
        }
    }

    fn get_sun_event_utc<F>(
        &self,
        rise_set_type: RiseSetType,
        twilight: TwilightType,
        nearest_fn: F,
        next_fn: F,
        previous_fn: F,
    ) -> f64
    where
        F: Fn(f64, f64, f64, f64, f64, u32) -> Result<f64, SunRS>,
    {
        const MAX_DAYS: u32 = 2; // number of days to look forward or backward
        let latitude = self.observer.latitude;
        let longitude = self.observer.longitude;
        let jd = self.time.to_jd();
        let angle = twilight.angle();
        let timezone = self.observer.timezone;

        match rise_set_type {
            RiseSetType::Nearest => {
                nearest_fn(latitude, longitude, jd, angle, timezone, MAX_DAYS).unwrap_or(0.0)
            }
            RiseSetType::Next => {
                next_fn(latitude, longitude, jd, angle, timezone, MAX_DAYS).unwrap_or(0.0)
            }
            RiseSetType::Previous => {
                previous_fn(latitude, longitude, jd, angle, timezone, MAX_DAYS).unwrap_or(0.0)
            }
        }
    }

    pub fn get_sunrise_utc(&self, rise_set_type: RiseSetType, twilight: TwilightType) -> f64 {
        self.get_sun_event_utc(
            rise_set_type,
            twilight,
            nearest_sunrise_utc as fn(f64, f64, f64, f64, f64, u32) -> Result<f64, SunRS>,
            next_sunrise_utc as fn(f64, f64, f64, f64, f64, u32) -> Result<f64, SunRS>,
            previous_sunrise_utc as fn(f64, f64, f64, f64, f64, u32) -> Result<f64, SunRS>,
        )
    }

    pub fn get_sunset_utc(&self, rise_set_type: RiseSetType, twilight: TwilightType) -> f64 {
        self.get_sun_event_utc(
            rise_set_type,
            twilight,
            nearest_sunset_utc as fn(f64, f64, f64, f64, f64, u32) -> Result<f64, SunRS>,
            next_sunset_utc as fn(f64, f64, f64, f64, f64, u32) -> Result<f64, SunRS>,
            previous_sunset_utc as fn(f64, f64, f64, f64, f64, u32) -> Result<f64, SunRS>,
        )
    }

    pub fn get_sunrise_local(&self, rise_set_type: RiseSetType, twilight: TwilightType) -> f64 {
        let utc = self.get_sunrise_utc(rise_set_type, twilight);
        if utc == 0.0 {
            0.0
        } else {
            utc + self.observer.timezone / 24.0
        }
    }

    pub fn get_sunset_local(&self, rise_set_type: RiseSetType, twilight: TwilightType) -> f64 {
        let utc = self.get_sunset_utc(rise_set_type, twilight);
        if utc == 0.0 {
            0.0
        } else {
            utc + self.observer.timezone / 24.0
        }
    }

    fn get_sun_event_str<F>(
        &self,
        rise_set_type: RiseSetType,
        twilight: TwilightType,
        format: Option<&str>,
        event_fn: F,
        never_message: &str,
    ) -> String
    where
        F: Fn(&Self, RiseSetType, TwilightType) -> f64,
    {
        let event_time = event_fn(self, rise_set_type, twilight);
        if event_time == 0.0 {
            never_message.to_string()
        } else {
            Time::from_jd(event_time).to_string(format)
        }
    }

    pub fn get_sunrise_utc_str(
        &self,
        rise_set_type: RiseSetType,
        twilight: TwilightType,
        format: Option<&str>,
    ) -> String {
        self.get_sun_event_str(
            rise_set_type,
            twilight,
            format,
            Sun::get_sunrise_utc,
            "Never Rises",
        )
    }

    pub fn get_sunrise_local_str(
        &self,
        rise_set_type: RiseSetType,
        twilight: TwilightType,
        format: Option<&str>,
    ) -> String {
        self.get_sun_event_str(
            rise_set_type,
            twilight,
            format,
            Sun::get_sunrise_local,
            "Never Rises",
        )
    }

    pub fn get_sunset_utc_str(
        &self,
        rise_set_type: RiseSetType,
        twilight: TwilightType,
        format: Option<&str>,
    ) -> String {
        self.get_sun_event_str(
            rise_set_type,
            twilight,
            format,
            Sun::get_sunset_utc,
            "Never Sets",
        )
    }

    pub fn get_sunset_local_str(
        &self,
        rise_set_type: RiseSetType,
        twilight: TwilightType,
        format: Option<&str>,
    ) -> String {
        self.get_sun_event_str(
            rise_set_type,
            twilight,
            format,
            Sun::get_sunset_local,
            "Never Sets",
        )
    }
}
