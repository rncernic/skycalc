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

use std::f64::consts::PI;
use libm::{atan2};
use crate::datetime::{jd2000_from_date,
                      ymd_from_jd,
                      gmst};
use crate::moon::MoonRS;
use crate::utils::{sind, cosd, constrain, cross_horizon, two_point_interpolation};
use crate::transformations::{equatorial_to_altaz};

//https://en.wikipedia.org/wiki/Sunrise_equation#Complete_calculation_on_Earth
//https://astrogreg.com/

#[derive(Debug)]
pub enum SunRS {
    NeverRise,
    NeverSet,
    Rise,
    Set,
}

//TODO Create test
pub fn sun_position_from_jd(jd: f64) -> (f64, f64) {
    let n = jd - 2_451_545.0;
    let mut l = (280.460 + 0.985_647_4 * n) % 360.0;
    let mut g = ((357.528 + 0.985_600_3 * n) % 360.0).to_radians();
    if l < 0.0 {l += 360.0};
    if g < 0.0 {g += 2. * PI};
    let lambda = (l + 1.915 * g.sin() + 0.020 * (2. * g).sin()).to_radians();
    let bet = 0.0;
    let eps = (23.439 - 0.000_000_4 * n).to_radians();
    let mut ra = atan2(eps.cos() * lambda.sin(), lambda.cos());
    let dec = (eps.sin() * lambda.sin()).asin();
    if ra < 0.0 {ra += 2. * PI};
    (ra.to_degrees(), dec.to_degrees())
}

//TODO Create test
pub fn sun_position_from_ymd(y: i64, m: i64, d: f64) -> (f64, f64) {
    let j = jd2000_from_date(y, m, d);
    sun_position_from_jd(j)
}

//TODO Unify with datetime
//lat and dec in degrees
//hour angle in degrees
pub fn sun_hour_angle(lat: f64, dec: f64) -> f64 {
    let numerator = sind(-0.833_33_f64) - sind(lat) * sind(dec);
    let denominator = cosd(lat) * cosd(dec);
    if denominator > 1. {999.0} // never sets - always above horizon
    else if denominator < -1. {-999.0} // never rises - always below horizon
    else { (numerator / denominator).acos().to_degrees() }
}

//TODO Create test
pub fn sun_alt_az_from_jd(lat: f64, lon: f64, ra: f64, dec: f64, jd: f64) -> (f64, f64) {
    let(y, m, d) = ymd_from_jd(jd);
    equatorial_to_altaz(lat, lon, ra, dec, y, m, d)
}

pub fn sun_alt_az_grid_utc(lat: f64, lon:f64, jd_start: f64, jd_end: f64,
                           num_points: usize) -> Vec<(f64, f64, f64)> {
    // create a null grid vector with 3 columns and num_points+1 rows
    let mut grid: Vec<(f64, f64, f64)> = Vec::new();
    let inc = (jd_end - jd_start) / num_points as f64;
    for i in 0..=num_points {
        let jd = jd_start + inc * i as f64;
        let (ra, dec) = sun_position_from_jd(jd);
        let (y, m, d) = ymd_from_jd(jd);
        let (alt, az) = equatorial_to_altaz(lat, lon, ra, dec, y, m, d);
        grid.push((jd, alt, az));
    }
    grid
}

//Todo implement previous, next and nearest
pub fn sunrise_utc_grid(jd: f64, lat: f64, lon: f64, horizon: f64, tz: f64) -> Result<f64, SunRS> {
    let num_points = 288;
    let target_night_start = (jd + 0.5).floor() + tz / 24.0;
    let target_night_end = target_night_start + 1.0;
    let sun = sun_alt_az_grid_utc(lat, lon, target_night_start, target_night_end, num_points);
    let v = cross_horizon(sun, horizon, true);
    if v.is_empty() {
        Err(SunRS::NeverRise)
    } else {
        Ok(two_point_interpolation(v[0].0, v[0].2, v[0].1, v[0].3, horizon))
    }
}

//Todo implement previous, next and nearest
pub fn sunset_utc_grid(jd: f64, lat: f64, lon: f64, horizon: f64, tz: f64) -> Result<f64, SunRS> {
    let num_points = 288;
    let target_night_start = (jd + 0.5).floor() + tz / 24.0;
    let target_night_end = target_night_start + 1.0;
    let sun = sun_alt_az_grid_utc(lat, lon, target_night_start, target_night_end, num_points);
    let v = cross_horizon(sun, horizon, false);
    if v.is_empty() {
        Err(SunRS::NeverSet)
    } else {
        Ok(two_point_interpolation(v[0].0, v[0].2, v[0].1, v[0].3, horizon))
    }
}