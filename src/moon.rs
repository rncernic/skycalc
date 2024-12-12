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

use crate::datetime::{jd2000_century_from_jd, ymd_from_jd};
use crate::utils::{cosd, sind, tand, constrain_360, cross_horizon, two_point_interpolation};
use crate::transformations::{equatorial_to_altaz};
use crate::earth::nutation;
use libm::{atan2};
use std::f64::consts::PI;

#[derive(Debug)]
pub enum MoonRS {
    NeverRise,
    NeverSet,
    Rise,
    Set,
}

// D, M, Mprime, F
const LUNAR_LON_ARGS: &[&[f64]] = &[
    &[0.0, 0.0, 1.0, 0.0],
    &[2.0, 0.0, -1.0, 0.0],
    &[2.0, 0.0, 0.0, 0.0],
    &[0.0, 0.0, 2.0, 0.0],
    &[0.0, 1.0, 0.0, 0.0],
    &[0.0, 0.0, 0.0, 2.0],
    &[2.0, 0.0, -2.0, 0.0],
    &[2.0, -1.0, -1.0, 0.0],
    &[2.0, 0.0, 1.0, 0.0],
    &[2.0, -1.0, 0.0, 0.0],
    &[0.0, 1.0, -1.0, 0.0],
    &[1.0, 0.0, 0.0, 0.0],
    &[0.0, 1.0, 1.0, 0.0],
    &[2.0, 0.0, 0.0, -2.0],
    &[0.0, 0.0, 1.0, 2.0],
    &[0.0, 0.0, 1.0, -2.0],
    &[4.0, 0.0, -1.0, 0.0],
    &[0.0, 0.0, 3.0, 0.0],
    &[4.0, 0.0, -2.0, 0.0],
    &[2.0, 1.0, -1.0, 0.0],
    &[2.0, 1.0, 0.0, 0.0],
    &[1.0, 0.0, -1.0, 0.0],
    &[1.0, 1.0, 0.0, 0.0],
    &[2.0, -1.0, 1.0, 0.0],
    &[2.0, 0.0, 2.0, 0.0],
    &[4.0, 0.0, 0.0, 0.0],
    &[2.0, 0.0, -3.0, 0.0],
    &[0.0, 1.0, -2.0, 0.0],
    &[2.0, 0.0, -1.0, 2.0],
    &[2.0, -1.0, -2.0, 0.0],
    &[1.0, 0.0, 1.0, 0.0],
    &[2.0, -2.0, 0.0, 0.0],
    &[0.0, 1.0, 2.0, 0.0],
    &[0.0, 2.0, 0.0, 0.0],
    &[2.0, -2.0, -1.0, 0.0],
    &[2.0, 0.0, 1.0, -2.0],
    &[2.0, 0.0, 0.0, 2.0],
    &[4.0, -1.0, -1.0, 0.0],
    &[0.0, 0.0, 2.0, 2.0],
    &[3.0, 0.0, -1.0, 0.0],
    &[2.0, 1.0, 1.0, 0.0],
    &[4.0, -1.0, -2.0, 0.0],
    &[0.0, 2.0, -1.0, 0.0],
    &[2.0, 2.0, -1.0, 0.0],
    &[2.0, 1.0, -2.0, 0.0],
    &[2.0, -1.0, 0.0, -2.0],
    &[4.0, 0.0, 1.0, 0.0],
    &[0.0, 0.0, 4.0, 0.0],
    &[4.0, -1.0, 0.0, 0.0],
    &[1.0, 0.0, -2.0, 0.0],
    &[2.0, 1.0, 0.0, -2.0],
    &[0.0, 0.0, 2.0, -2.0],
    &[1.0, 1.0, 1.0, 0.0],
    &[3.0, 0.0, -2.0, 0.0],
    &[4.0, 0.0, -3.0, 0.0],
    &[2.0, -1.0, 2.0, 0.0],
    &[0.0, 2.0, 1.0, 0.0],
    &[1.0, 1.0, -1.0, 0.0],
    &[2.0, 0.0, 3.0, 0.0],
    &[2.0, 0.0, -1.0, -2.0],
];

// D, M, Mprime, F
const LUNAR_LAT_ARGS: &[&[f64]] = &[
    &[0.0, 0.0, 0.0, 1.0],
    &[0.0, 0.0, 1.0, 1.0],
    &[0.0, 0.0, 1.0, -1.0],
    &[2.0, 0.0, 0.0, -1.0],
    &[2.0, 0.0, -1.0, 1.0],
    &[2.0, 0.0, -1.0, -1.0],
    &[2.0, 0.0, 0.0, 1.0],
    &[0.0, 0.0, 2.0, 1.0],
    &[2.0, 0.0, 1.0, -1.0],
    &[0.0, 0.0, 2.0, -1.0],
    &[2.0, -1.0, 0.0, -1.0],
    &[2.0, 0.0, -2.0, -1.0],
    &[2.0, 0.0, 1.0, 1.0],
    &[2.0, 1.0, 0.0, -1.0],
    &[2.0, -1.0, -1.0, 1.0],
    &[2.0, -1.0, 0.0, 1.0],
    &[2.0, -1.0, -1.0, -1.0],
    &[0.0, 1.0, -1.0, -1.0],
    &[4.0, 0.0, -1.0, -1.0],
    &[0.0, 1.0, 0.0, 1.0],
    &[0.0, 0.0, 0.0, 3.0],
    &[0.0, 1.0, -1.0, 1.0],
    &[1.0, 0.0, 0.0, 1.0],
    &[0.0, 1.0, 1.0, 1.0],
    &[0.0, 1.0, 1.0, -1.0],
    &[0.0, 1.0, 0.0, -1.0],
    &[1.0, 0.0, 0.0, -1.0],
    &[0.0, 0.0, 3.0, 1.0],
    &[4.0, 0.0, 0.0, -1.0],
    &[4.0, 0.0, -1.0, 1.0],
    &[0.0, 0.0, 1.0, -3.0],
    &[4.0, 0.0, -2.0, 1.0],
    &[2.0, 0.0, 0.0, -3.0],
    &[2.0, 0.0, 2.0, -1.0],
    &[2.0, -1.0, 1.0, -1.0],
    &[2.0, 0.0, -2.0, 1.0],
    &[0.0, 0.0, 3.0, -1.0],
    &[2.0, 0.0, -2.0, 1.0],
    &[0.0, 0.0, 3.0, -1.0],
    &[2.0, 0.0, 2.0, 1.0],
    &[2.0, 0.0, -3.0, -1.0],
    &[2.0, 1.0, -1.0, 1.0],
    &[2.0, 1.0, 0.0, 1.0],
    &[4.0, 0.0, 0.0, 1.0],
    &[2.0, -1.0, 1.0, 1.0],
    &[2.0, -2.0, 0.0, -1.0],
    &[0.0, 0.0, 1.0, 3.0],
    &[2.0, 1.0, 1.0, -1.0],
    &[1.0, 1.0, 0.0, -1.0],
    &[1.0, 1.0, 0.0, 1.0],
    &[0.0, 1.0, -2.0, -1.0],
    &[2.0, 1.0, -1.0, -1.0],
    &[1.0, 0.0, 1.0, 1.0],
    &[2.0, -1.0, -2.0, -1.0],
    &[0.0, 1.0, 2.0, 1.0],
    &[4.0, 0.0, -2.0, -1.0],
    &[4.0, -1.0, -1.0, -1.0],
    &[1.0, 0.0, 1.0, -1.0],
    &[4.0, 0.0, 1.0, -1.0],
    &[1.0, 0.0, -1.0, -1.0],
    &[4.0, -1.0, 0.0, -1.0],
    &[2.0, -2.0, 0.0, 1.0],
];

// lon, dist, lat
const LUNAR_COEFF: &[&[f64]] = &[
    &[6288774.0, -20905355.0, 5128122.0],
    &[1274027.0, -3699111.0, 280602.0],
    &[658314.0, -2955968.0, 277693.0],
    &[213618.0, -569925.0, 173237.0],
    &[-185116.0, 48888.0, 55413.0],
    &[-114332.0, -3149.0, 46271.0],
    &[58793.0, 246158.0, 32573.0],
    &[57066.0, -152138.0, 17198.0],
    &[53322.0, -170733.0, 9266.0],
    &[45758.0, -204586.0, 8822.0],
    &[-40923.0, -129620.0, 8216.0],
    &[-34720.0, 108743.0, 4324.0],
    &[-30383.0, 104755.0, 4200.0],
    &[15327.0, 10321.0, -3359.0],
    &[-12528.0, 0.0, 2463.0],
    &[10980.0, 79661.0, 2211.0],
    &[10675.0, -34782.0, 2065.0],
    &[10034.0, -23210.0, -1870.0],
    &[8548.0, -21636.0, 1828.0],
    &[-7888.0, 24208.0,  -1794.0],
    &[-6766.0, 30824.0, -1749.0],
    &[-5163.0, -8379.0, -1565.0],
    &[4987.0, -16675.0, -1491.0],
    &[4036.0, -12831.0, -1475.0],
    &[3994.0, -10445.0, -1410.0],
    &[3861.0, -11650.0, -1344.0],
    &[3665.0, 14403.0, -1335.0],
    &[-2689.0, -7003.0, 1107.0],
    &[-2602.0, 0.0, 1021.0],
    &[2390.0, 10056.0, 833.0],
    &[-2348.0, 6322.0, 777.0],
    &[2236.0, -9884.0, 671.0],
    &[-2120.0, 5751.0, 607.0],
    &[-2069.0, 0.0, 596.0],
    &[2048.0, -4950.0, 491.0],
    &[-1773.0, 4130.0, -451.0],
    &[-1595.0, 0.0, 439.0],
    &[1215.0, -3958.0, 422.0],
    &[-1110.0, 0.0, 421.0],
    &[-892.0, 3258.0, -366.0],
    &[-810.0, 2616.0, -351.0],
    &[759.0, -1897.0, 331.0],
    &[-713.0, -2117.0, 315.0],
    &[-700.0, 2354.0, 302.0],
    &[691.0, 0.0, -283.0],
    &[596.0, 0.0, -229.0],
    &[549.0, -1423.0, 223.0],
    &[537.0, -1117.0, 223.0],
    &[520.0, -1571.0, -220.0],
    &[-487.0, -1739.0, -220.0],
    &[-399.0, 0.0, -185.0],
    &[-381.0, -4421.0, 181.0],
    &[351.0, 0.0, -177.0],
    &[-340.0, 0.0, 176.0],
    &[330.0, 0.0, 166.0],
    &[327.0, 0.0, -164.0],
    &[-323.0, 1165.0, 132.0],
    &[299.0, 0.0, -119.0],
    &[294.0, 0.0, 115.0],
    &[0.0, 8752.0, 107.0],
];

pub fn moon_position_low_precision(t: f64) -> (f64, f64) {
    let l = 218.32 + 481_267.881 * t + 6.29 * sind(135.0 + 477_198.87 * t) -
        1.27 * sind(259.3 - 413_335.36 * t) + 0.66 * sind(235.7 + 890_534.22 * t) +
        0.21 * sind(269.9 + 954_397.74 * t) - 0.19 * sind(357.5 + 35_999.05 * t) -
        0.11 * sind(186.5 + 966_404.03 * t);
    let b = 5.13 * sind( 93.3 + 483_202.02 * t) +
        0.28 * sind(228.2 + 960_400.89 * t) -
        0.28 * sind(318.3 + 6_003.15 * t) -
        0.17 * sind(217.6 - 407_332.21 * t);
    let p = 0.950_8 + 0.051_8 * cosd(135.0 + 477_198.87 * t) +
        0.009_5 * cosd(259.3 - 413_335.36 * t) +
        0.007_8 * cosd(235.7 + 890_534.22 * t) +
        0.002_8 * cosd(269.9 + 954_397.74 * t);

    // let sd = 0.272_4 * p;
    // let r = 1.0 / sind(p);

    let ll = cosd(b) * cosd(l);
    let m = 0.917_5 * cosd(b) * sind(l) - 0.397_8 * sind(b);
    let n = 0.397_8 * cosd(b) * sind(l) + 0.917_5 * sind(b);

    let mut ra = atan2(m,ll);
    if ra < 0.0 { ra += 2.0 * PI;}
    let dec = n.asin();
    (ra.to_degrees(), dec.to_degrees())
}

pub fn moon_position_high_precision(t: f64) -> (f64, f64, f64) {

    //let t = jd2000_century_from_date(y, month, d);

    // mean longitude of the Moon
    let lprime = constrain_360(218.316_447_7 + 481_267.881_234_21 * t -
        0.001_578_6 * t * t +
        t * t * t / 538_841.0 -
        t * t * t * t / 65_194_000.0).to_radians();

    // mean elongation of the Moon
    let d = constrain_360(297.850_192_1 + 445_267.111_403_4 * t -
        0.001_881_9 * t * t +
        t * t * t / 545_868.0 -
        t * t * t * t / 113_065_000.0).to_radians();

    // mean anomaly of the Sun
    // note: this doesn't quite match the calculation used for solar position
    let m = constrain_360(357.529_11 + 35_999.050_290_9 * t -
        0.000_153_6 * t * t +
        t * t * t / 24_490_000.0).to_radians();

    // mean anomaly of the Moon
    let mprime = constrain_360(134.963_396_4 + 477_198.867_505_5 * t +
        0.008_741_4 * t * t +
        t * t * t  / 69_699.0 -
        t * t * t * t / 14_712_000.0).to_radians();

    // argument of latitude of the Moon
    let f = constrain_360(93.272_095_0 + 483_202.017_523_3 * t -
        0.003_653_9 * t * t -
        t * t * t / 3_526_000.0 +
        t * t * t *t / 863_310_000.0).to_radians();

    // three further arguments, a1 is due to Venus, a2 is due to Jupiter
    let a1 = constrain_360(119.75 + 131.849 * t).to_radians();
    let a2 = constrain_360(53.09 + 479_264.290 * t).to_radians();
    let a3 = constrain_360(313.45 + 481_266.484 * t).to_radians();

    // "correction" for eccentricity of Earth's orbit
    let e = 1.0 - 0.002_516 * t - 0.000_007_4 * t * t;
    let e2 = e * e;

    let mut sigmal = 0.0;
    let mut sigmab = 0.0;
    let mut sigmar = 0.0;
    for (i, args) in LUNAR_LON_ARGS.iter().enumerate() {
        let coeff = LUNAR_COEFF[i];
        let x = args[0] * d + args[1] * m + args[2] * mprime + args[3] * f;
        if args[1] == 1.0 || args[1] == -1.0 {
            sigmal += e * coeff[0] * x.sin();
            sigmar += e * coeff[1] * x.cos();
        }
        else if args[1] == 2.0 || args[1] == -2.0 {
            sigmal += e2 * coeff[0] * x.sin();
            sigmar += e2 * coeff[1] * x.cos();
        }
        else {
            sigmal += coeff[0] * x.sin();
            sigmar += coeff[1] * x.cos();
        }

        let v = LUNAR_LAT_ARGS[i];
        let x = v[0] * d + v[1] * m + v[2] * mprime + v[3] * f;

        if v[1] == 1.0 || v[1] == -1.0 { sigmab += e * coeff[2] * x.sin(); }
        else if v[1] == 2.0 ||  v[1] == -2.0 { sigmab += e2 * coeff[2] * x.sin(); }
        else { sigmab += coeff[2] * x.sin(); }
    }

    sigmal += 3_958.0 * a1.sin() + 1_962.0 * (lprime - f).sin() + 318.0 * a2.sin();
    sigmab += -2_235.0 * lprime.sin() + 382.0 * a3.sin() + 175.0 * (a1 - f).sin() +
        175.0 * (a1 + f).sin() + 127.0 * (lprime - mprime).sin() -
        115.0 * (lprime + mprime).sin();

    let true_lon = lprime.to_degrees() + sigmal / 1e6;
    let true_lat = sigmab / 1e6;
    let radius = 385_000.56 + sigmar / 1e3;

    // apparent longitude
    let (delta_phi, _, mut eps) = nutation(t);
    let apparent_lon = true_lon + delta_phi;

    eps = eps.to_radians();

    let right_ascension = constrain_360(atan2(eps.cos() * sind(apparent_lon) -
                                                  eps.sin() * tand(true_lat), cosd(apparent_lon)).to_degrees());

    let declination = (sind(true_lat) * eps.cos() +
        eps.sin() * sind(apparent_lon) * cosd(true_lat)).asin().to_degrees();

    (right_ascension, declination, radius)
}

pub fn moon_alt_az_grid_utc(lat: f64, lon:f64, jd_start: f64, jd_end: f64,
                           num_points: usize) -> Vec<(f64, f64, f64)> {
    let mut grid: Vec<(f64, f64, f64)> = Vec::new();
    let inc = (jd_end - jd_start) / num_points as f64;
    for i in 0..=num_points {
        let jd = jd_start + inc * i as f64;
        let t = jd2000_century_from_jd(jd);
        let (ra, dec, _) = moon_position_high_precision(t);
        let (y, m, d) = ymd_from_jd(jd);
        let (alt, az) = equatorial_to_altaz(lat, lon, ra, dec, y, m, d);
        grid.push((jd, alt, az));
    }
    grid
}

//Todo implement previous, next and nearest
pub fn moonrise_utc_grid(jd: f64, lat: f64, lon: f64, tz: f64) -> Result<f64, MoonRS> {
    let num_points = 288;
    let target_night_start = (jd + 0.5).floor() + tz / 24.0; // Noon @ local time
    let target_night_end = target_night_start + 1.0;
    let moon = moon_alt_az_grid_utc(lat, lon, target_night_start, target_night_end, num_points);
    let v = cross_horizon(moon,0.125, true);
    if v.is_empty() {
        Err(MoonRS::NeverRise)
    } else {
        Ok(two_point_interpolation(v[0].0, v[0].2, v[0].1, v[0].3, 0.125))
    }
}

//Todo implement previous, next and nearest
pub fn moonset_utc_grid(jd: f64, lat: f64, lon: f64, tz: f64) -> Result<f64, MoonRS> {
    let num_points = 288;
    let target_night_start = (jd + 0.5).floor() + tz / 24.0; // Noon @ local time
    let target_night_end = target_night_start + 1.0;
    let moon = moon_alt_az_grid_utc(lat, lon, target_night_start, target_night_end, num_points);
    let v = cross_horizon(moon,0.125, false);
    if v.is_empty() {
        Err(MoonRS::NeverSet)
    } else {
        Ok(two_point_interpolation(v[0].0, v[0].2, v[0].1, v[0].3, 0.125))
    }
}
