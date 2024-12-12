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

use crate::utils::constrain_360;

// D, M, Mprime, F, omega
const NUTATION_ARGS: &[&[f64]] = &[
    &[0.0, 0.0, 0.0, 0.0, 1.0],
    &[-2.0, 0.0, 0.0, 2.0, 2.0],
    &[0.0, 0.0, 0.0, 2.0, 2.0],
    &[0.0, 0.0, 0.0, 0.0, 2.0],
    &[0.0, 1.0, 0.0, 0.0, 0.0],
    &[0.0, 0.0, 1.0, 0.0, 0.0],
    &[-2.0, 1.0, 0.0, 2.0, 2.0],
    &[0.0, 0.0, 0.0, 2.0, 1.0],
    &[0.0, 0.0, 1.0, 2.0, 2.0],
    &[-2.0, -1.0, 0.0, 2.0, 2.0],
    &[-2.0, 0.0, 1.0, 0.0, 0.0],
    &[-2.0, 0.0, 0.0, 2.0, 1.0],
    &[0.0, 0.0, -1.0, 2.0, 2.0],
    &[2.0, 0.0, 0.0, 0.0, 0.0],
    &[0.0, 0.0, 1.0, 0.0, 1.0],
    &[2.0, 0.0, -1.0, 2.0, 2.0],
    &[0.0, 0.0, -1.0, 0.0, 1.0],
    &[0.0, 0.0, 1.0, 2.0, 1.0],
    &[-2.0, 0.0, 2.0, 0.0, 0.0],
    &[0.0, 0.0, -2.0, 2.0, 1.0],
    &[2.0, 0.0, 0.0, 2.0, 2.0],
    &[0.0, 0.0, 2.0, 2.0, 2.0],
    &[0.0, 0.0, 2.0, 0.0, 0.0],
    &[-2.0, 0.0, 1.0, 2.0, 2.0],
    &[0.0, 0.0, 0.0, 2.0, 0.0],
    &[-2.0, 0.0, 0.0, 2.0, 0.0],
    &[0.0, 0.0, -1.0, 2.0, 1.0],
    &[0.0, 2.0, 0.0, 0.0, 0.0],
    &[2.0, 0.0, -1.0, 0.0, 1.0],
    &[-2.0, 2.0, 0.0, 2.0, 2.0],
    &[0.0, 1.0, 0.0, 0.0, 1.0],
    &[-2.0, 0.0, 1.0, 0.0, 1.0],
    &[0.0, -1.0, 0.0, 0.0, 1.0],
    &[0.0, 0.0, 2.0, -2.0, 0.0],
    &[2.0, 0.0, -1.0, 2.0, 1.0],
    &[2.0, 0.0, 1.0, 2.0, 2.0],
    &[0.0, 1.0, 0.0, 2.0, 2.0],
    &[-2.0, 1.0, 1.0, 0.0, 0.0],
    &[0.0, -1.0, 0.0, 2.0, 2.0],
    &[2.0, 0.0, 0.0, 2.0, 1.0],
    &[2.0, 0.0, 1.0, 0.0, 0.0],
    &[-2.0, 0.0, 2.0, 2.0, 2.0],
    &[-2.0, 0.0, 1.0, 2.0, 1.0],
    &[2.0, 0.0, -2.0, 0.0, 1.0],
    &[2.0, 0.0, 0.0, 0.0, 1.0],
    &[0.0, -1.0, 1.0, 0.0, 0.0],
    &[-2.0, -1.0, 0.0, 2.0, 1.0],
    &[-2.0, 0.0, 0.0, 0.0, 1.0],
    &[0.0, 0.0, 2.0, 2.0, 1.0],
    &[-2.0, 0.0, 2.0, 0.0, 1.0],
    &[-2.0, 1.0, 0.0, 2.0, 1.0],
    &[0.0, 0.0, 1.0, -2.0, 0.0],
    &[-1.0, 0.0, 1.0, 0.0, 0.0],
    &[-2.0, 1.0, 0.0, 0.0, 0.0],
    &[1.0, 0.0, 0.0, 0.0, 0.0],
    &[0.0, 0.0, 1.0, 2.0, 0.0],
    &[0.0, 0.0, -2.0, 2.0, 2.0],
    &[-1.0, -1.0, 1.0, 0.0, 0.0],
    &[0.0, 1.0, 1.0, 0.0, 0.0],
    &[0.0, -1.0, 1.0, 2.0, 2.0],
    &[2.0, -1.0, -1.0, 2.0, 2.0],
    &[0.0, 0.0, 3.0, 2.0, 2.0],
    &[2.0, -1.0, 0.0, 2.0, 2.0],
];

const NUTATION_SIN_COEFF: &[&[f64]] = &[
    &[-171996.0, -174.2],
    &[-13187.0, -1.6],
    &[-2274.0, -0.2],
    &[2062.0, 0.2],
    &[1426.0, -3.4],
    &[712.0, 0.1],
    &[-517.0, 1.2],
    &[-386.0, -0.4],
    &[-301.0, 0.0],
    &[217.0, -0.5],
    &[-158.0, 0.0],
    &[129.0, 0.1],
    &[123.0, 0.0],
    &[63.0, 0.0],
    &[63.0, 0.1],
    &[-59.0, 0.0],
    &[-58.0, -0.1],
    &[-51.0, 0.0],
    &[48.0, 0.0],
    &[46.0, 0.0],
    &[-38.0, 0.0],
    &[-31.0, 0.0],
    &[29.0, 0.0],
    &[29.0, 0.0],
    &[26.0, 0.0],
    &[-22.0, 0.0],
    &[21.0, 0.0],
    &[17.0, -0.1],
    &[16.0, 0.0],
    &[-16.0, 0.1],
    &[-15.0, 0.0],
    &[-13.0, 0.0],
    &[-12.0, 0.0],
    &[11.0, 0.0],
    &[-10.0, 0.0],
    &[-8.0, 0.0],
    &[7.0, 0.0],
    &[-7.0, 0.0],
    &[-7.0, 0.0],
    &[-7.0, 0.0],
    &[6.0, 0.0],
    &[6.0, 0.0],
    &[6.0, 0.0],
    &[-6.0, 0.0],
    &[-6.0, 0.0],
    &[5.0, 0.0],
    &[-5.0, 0.0],
    &[-5.0, 0.0],
    &[-5.0, 0.0],
    &[4.0, 0.0],
    &[4.0, 0.0],
    &[4.0, 0.0],
    &[-4.0, 0.0],
    &[-4.0, 0.0],
    &[-4.0, 0.0],
    &[3.0, 0.0],
    &[-3.0, 0.0],
    &[-3.0, 0.0],
    &[-3.0, 0.0],
    &[-3.0, 0.0],
    &[-3.0, 0.0],
    &[-3.0, 0.0],
    &[-3.0, 0.0],
];

const NUTATION_COS_COEFF: &[&[f64]] = &[
    &[92025.0, 8.9],
    &[5736.0, -3.1],
    &[977.0, -0.5],
    &[-895.0, 0.5],
    &[54.0, -0.1],
    &[-7.0, 0.0],
    &[224.0, -0.6],
    &[200.0, 0.0],
    &[129.0, -0.1],
    &[-95.0, 0.3],
    &[0.0, 0.0],
    &[-70.0, 0.0],
    &[-53.0, 0.0],
    &[0.0, 0.0],
    &[-33.0, 0.0],
    &[26.0, 0.0],
    &[32.0, 0.0],
    &[27.0, 0.0],
    &[0.0, 0.0],
    &[-24.0, 0.0],
    &[16.0, 0.0],
    &[13.0, 0.0],
    &[0.0, 0.0],
    &[-12.0, 0.0],
    &[0.0, 0.0],
    &[0.0, 0.0],
    &[-10.0, 0.0],
    &[0.0, 0.0],
    &[-8.0, 0.0],
    &[7.0, 0.0],
    &[9.0, 0.0],
    &[7.0, 0.0],
    &[6.0, 0.0],
    &[0.0, 0.0],
    &[5.0, 0.0],
    &[3.0, 0.0],
    &[-3.0, 0.0],
    &[0.0, 0.0],
    &[3.0, 0.0],
    &[3.0, 0.0],
    &[0.0, 0.0],
    &[-3.0, 0.0],
    &[-3.0, 0.0],
    &[3.0, 0.0],
    &[3.0, 0.0],
    &[0.0, 0.0],
    &[3.0, 0.0],
    &[3.0, 0.0],
    &[3.0, 0.0],
    &[0.0, 0.0],
    &[0.0, 0.0],
    &[0.0, 0.0],
    &[0.0, 0.0],
    &[0.0, 0.0],
    &[0.0, 0.0],
    &[0.0, 0.0],
    &[0.0, 0.0],
    &[0.0, 0.0],
    &[0.0, 0.0],
    &[0.0, 0.0],
    &[0.0, 0.0],
    &[0.0, 0.0],
    &[0.0, 0.0],
];


// deltas in hours and eps0 in degrees
pub fn nutation(t: f64) -> (f64, f64, f64) {
    //let t = jd2000_century_from_date(y, m, d);
    //mean elongation of Moon from Sun
    let d = constrain_360(297.850_36 + 445_267.111_480 * t -
        0.001_914_2 * t * t +
        t * t * t / 189_474.0).to_radians();

    //mean anomaly of the Sun
    let m = constrain_360(357.527_72 + 35_999.050_340 * t -
        0.000_160_3 * t * t + t * t * t / 300_000.0).to_radians();

    //mean anomaly of the Moon
    let mprime = constrain_360(134.962_98 + 477_198.867_398 * t +
        0.008_697_2 * t * t +
        t * t * t / 56_250.0).to_radians();

    //Moon's argument of latitude
    let f = constrain_360(93.271_91 + 483_202.017_538 * t -
        0.003_682_5 * t * t +
        t * t * t / 327_270.0).to_radians();

    //longitude of the ascending node of the Moon's mean orbit on the
    //elliptic, measured from the mean equinox of the date.
    let omega = constrain_360(125.044_52 - 1_934.136_261 * t +
        0.002_070_8 * t * t +
        t * t * t / 450_000.0).to_radians();

    let mut delta_phi = 0.0;
    let mut delta_eps = 0.0;

    for (i, v) in NUTATION_ARGS.iter().enumerate() {
        let x = v[0] * d + v[1] * m + v[2] * mprime + v[3] * f + v[4] * omega;
        delta_phi += (NUTATION_SIN_COEFF[i][0] + NUTATION_SIN_COEFF[i][1] * t) * x.sin();
        delta_eps += (NUTATION_COS_COEFF[i][0] + NUTATION_COS_COEFF[i][1] * t) * x.cos();
    }

    //convert results from 0.0001 seconds to hours
    delta_phi /= 1e4 * 3_600.0;
    delta_eps /= 1e4 * 3_600.0;

    //mean obliquity of the ecliptic
    let eps0 = 23.0 + 26.0 / 60.0 + (21.448 - 46.815_0 * t - 0.000_59 * t * t +
        0.001_813 * t * t * t) / 3_600.0;

    (delta_phi, delta_eps, eps0)
}

#[cfg(test)]
mod test{
    use assert_approx_eq::assert_approx_eq;
    use crate::earth::{nutation};

    #[test]
    fn test_nutation() {
        let (dphi, deps, eps0) = nutation(1987.0);
        assert_approx_eq!(dphi, -0.001_052_203, 1e-6);
        assert_approx_eq!(deps, 0.002_623_056, 1e-6);
        assert_approx_eq!(eps0, 23.440946389, 1e-6);
    }

}