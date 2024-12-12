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

use libm::atan2;
use crate::time::Time;
use crate::utils::{constrain_360, sind, cosd};


// in degrees
pub fn hour_angle(lon: f64, ra: f64, y: i64, m: u64, d: u64, h: u64, min: u64, s: u64) -> f64 {
    let date = Time::new(y, m, d, h, min, s);
    constrain_360(date.to_gst() + lon - ra)
}

// azimuth reckoned from north
pub fn equatorial_to_altaz(lat: f64, lon: f64, ra: f64, dec: f64, y: i64, m:u64, d: u64, h: u64, min: u64, s: u64) -> (f64, f64) {
    let ha = hour_angle(lon, ra, y, m, d, h, min, s);
    let x = -cosd(ha) * cosd(dec) * sind(lat) + sind(dec) * cosd(lat);
    let y = -sind(ha) * cosd(dec);
    let z = cosd(ha) * cosd(dec) * cosd(lat) + sind(dec) * sind(lat);
    let r = (x * x + y * y).sqrt();

    let alt = atan2(z, r).to_degrees();
    let az = constrain_360(atan2(y, x).to_degrees());

    (alt, az)
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use crate::transformations::{equatorial_to_altaz,
                                 hour_angle};

    #[test]
    fn test_eq_to_altaz() {
        let (alt, az) = equatorial_to_altaz(38.921388889, -77.065555556,
                                            347.3193375, -6.719891667,
                                            1987, 4, 10.80625);
        assert_approx_eq!(alt, 15.124_9, 1e-3);
    }

    #[test]
    fn test_hour_angle() {
        assert_approx_eq!(hour_angle(-77.065555556, 347.3193375, 1987, 4, 10.80625), 64.352_133, 1e-3);
    }
}