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

use std::fs::File;
use std::io::Write;
use SunRS::NeverSet;
use crate::observer::Observer;
use crate::angle::format_dms;
use crate::datetime::{format_hm, ymd_hms_from_jd};
use crate::sun::SunRS;

const VERSION: &str = "0.0.1 alpha";

pub fn darkness_report(observer: Observer) {

    let mut line: Vec<String> = Vec::new();
    line.push("\n------------------------------------------------------------------------------------------------------------".to_string());
    line.push(format!("\nSkyCalc v.{}", VERSION));
    line.push("\n------------------------------------------------------------------------------------------------------------".to_string());
    line.push(format!("\n\nObservatory: {}", observer.observatory_name.as_deref().unwrap_or("")));
    line.push(format!("\n\n  - Location    ->  lat: {}, lon: {}, alt: {}m, tz: {:.1}",
                      format_dms(observer.location.latitude, true),
                      format_dms(observer.location.longitude, false),
                      observer.location.elevation,
                      observer.location.timezone));
    line.push(format!("\n  - Environment ->  pressure: {:.0} hPa, temperature: {:.0} \u{00B0}C, humidity: {:.0}%",
                      observer.environment.pressure, observer.environment.temperature, observer.environment.relative_humidity));
    line.push(format!("\n\nDarkness info for night: (Wed) yyyy-mm-dd to (Thu) yyyy-mm-dd"));
    line.push(format!("\n\n  - Sunset                  : {:11}  Sunrise                : {:11}  Length: hh:mm",
                      observer.get_sunrise_sunset_str().1, observer.get_sunrise_sunset_str().0));
    line.push(format!("\n  - Civil Tw start          : {:11}  Civil Tw end           : {:11}  Length: hh:mm",
                      observer.get_civil_rise_set_str().1, observer.get_civil_rise_set_str().0));
    line.push(format!("\n  - Nautical Tw start       : {:11}  Nautical Tw end        : {:11}  Length: hh:mm",
                      observer.get_nautical_rise_set_str().1, observer.get_nautical_rise_set_str().0));
    line.push(format!("\n  - Astronomical Tw start   : {:11}  Astronomical Tw end    : {:11}  Length: hh:mm",
                      observer.get_astro_rise_set_str().1, observer.get_astro_rise_set_str().0));
    line.push("\n".to_string());
    line.push("\n  - Moon rise               : hh:mm        Moon set               : hh:mm        Length: hh:mm".to_string());
    line.push("\n  - Moon illumination       : xxx%         Moon age               : xx days      Phase : zzzzzzzzzz".to_string());
    line.push("\n".to_string());
    line.push("\n  - Deep Sky Darkness start : hh:mm        Deep Sky Darkness end  : hh:mm        Length: hh:mm".to_string());
    line.push("\n  - Narrow Band start       : hh:mm        Narrow Band end        : hh:mm        Length: hh:mm".to_string());
    line.push("\n\n------------------------------------------------------------------------------------------------------------".to_string());
    line.push("\n\nConstraints:".to_string());
    line.push("\n\n------------------------------------------------------------------------------------------------------------".to_string());

    let mut f = File::create("skycalc.txt").expect("Unable to create file");
    f.write_all(line.concat().as_ref()).expect("Unable to write data");

}