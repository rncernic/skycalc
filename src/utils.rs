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

pub fn sind(v: f64) -> f64 {
    v.to_radians().sin()
}

pub fn cosd(v: f64) -> f64 {
    v.to_radians().cos()
}

pub fn tand(v: f64) -> f64 { v.to_radians().tan() }

pub fn constrain_360(angle: f64) -> f64 {
    ((angle % 360.0) + 360.0) % 360.0
}

pub fn constrain_180(v: f64) -> f64 {
    ((v % 180.0) + 180.0) % 180.0
}

pub fn constrain(v: f64) -> f64 {
    if v < 0.0 { v + 1.0 }
    else if v > 1.0 { v - 1.0 }
    else { v }
}

pub fn round_float(value: f64, num_digits: f64) -> f64 {
    (value * 10_f64.powf(num_digits)).round() / 10_f64.powf(num_digits)
}

// Do linear interpolation between two ``altitudes`` at two times to determine the time when the
// altitude goes through zero.
//
// Parameters
// ----------
// jd_before : JD(UTC) before crossing event
//
// jd_after : JD(UTC) after crossing event
//
// alt_before : altitude before crossing event (degrees)
//
// alt_after : altitude after crossing event (degrees)
//
// horizon : Solve for the time when the altitude is equal to a reference altitude (degrees)
//
// Returns
// -------
// t : JD(UTC) Time when target crosses the horizon
//
// Observation
// -----------
//
// Interpolation will work only if alt_before is below horizon and alt_after is above or vice-vers.
// This function does not handle never rises and never sets situations.
//
pub fn two_point_interpolation(jd_before: f64, jd_after: f64,
                               alt_before: f64, alt_after: f64, horizon: f64) -> f64 {


    // Approximate the horizon-crossing time:
    let slope = (alt_after - alt_before) / (jd_after - jd_before);
    let crossing_jd = jd_after - (alt_after - horizon) / slope;

    crossing_jd
}

pub fn float_loop(start: f64, threshold: f64, step_size: f64) -> impl Iterator<Item = f64> {
    std::iter::successors(Some(start), move |&prev| {
        let next = prev + step_size;
        (next < threshold).then_some(next)
    })
}

pub fn cross_horizon(grid: Vec<(f64, f64, f64)>,
                     horizon: f64, is_rising: bool) -> Vec<(f64, f64, f64, f64)> {
    let mut cross_points: Vec<(f64, f64, f64, f64)> = Vec::new();
    let mut previous_altitude = None;
    // let mut never_rise = true;
    // let mut never_set = true;
    for i in 0..grid.len() {
        if let Some(prev_alt) = previous_altitude {
            if is_rising {
                if prev_alt < horizon && grid[i].1 >= horizon {
                    cross_points.push((grid[i - 1].0, grid[i - 1].1, grid[i].0, grid[i].1));
                    // never_rise = false;
                }
            } else {
                if prev_alt > horizon && grid[i].1 <= horizon {
                    cross_points.push((grid[i-1].0, grid[i-1].1, grid[i].0, grid[i].1));
                    // never_set = false;
                }
            }
        }
        previous_altitude = Some(grid[i].1);
    }

    cross_points
}

// Anderson Peligrini
// fn float_loop<F>(start: f64, end: f64, increment: f64, mut func: F)
// where
//     F: FnMut(f64),
// {
//     let mut i = start;
//
//     while i < end {
//         func(i);
//         i += increment;
//     }
// }
//
// fn main() {
//     float_loop(0.0, 10.0, 0.5, |i| {
//         println!("Value of i: {}", i);
//     });
// }
