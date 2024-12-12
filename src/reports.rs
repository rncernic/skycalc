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

use std::fs::File;
use std::io::Write;
use crate::constraint::Constraints;
use crate::darkness::{Darkness};
use crate::environment::Environment;
use crate::moon::Moon;
use crate::observer::Observer;
use crate::sun::RiseSetType::{Nearest, Next, Previous};
use crate::sun::Sun;
use crate::sun::TwilightType::{AstronomicalTwilight, CivilTwilight, NauticalTwilight, RiseSet};
use crate::time::Time;
use crate::version::VERSION;

pub(crate) fn header_section() -> Vec<String> {
    let mut header: Vec<String> = Vec::new();
    header.push("\n------------------------------------------------------------------------------------------".to_string());
    header.push(format!("\nSkyCalc v.{}", VERSION));
    header.push("\n------------------------------------------------------------------------------------------".to_string());
    header.push("\n\n".to_string());
    header
}

pub(crate) fn observer_section(observer: &Observer) -> Vec<String> {
    let mut obs: Vec<String> = Vec::new();
    obs.push("Observatory:".to_string());
    obs.push("\n   - ".to_string());
    obs.push(observer.to_string_decimal());
    obs
}

pub(crate) fn environment_section(environment: &Environment) -> Vec<String> {
    let mut env: Vec<String> = Vec::new();
    env.push("\n   - ".to_string());
    env.push(environment.to_string());
    env.push("\n\n".to_string());
    env
}

pub(crate) fn night_section(time: &Time) -> Vec<String> {
    let start = time;
    let end = Time::from_jd(start.to_jd() + 1.0);
    let mut night: Vec<String> = Vec::new();
    night.push(format!("Info for night:  {:10} to {:10} in local time", start.to_string(Some("yyyymmdd")), end.to_string(Some("yyyymmdd"))));
    night.push("\n\n".to_string());
    night
}

pub(crate) fn moon_section(observer: &Observer, time: &Time, environment: &Environment) -> Vec<String> {
    let moon = Moon::new(&observer, &time, &environment);
    let moonrise = moon.get_moonrise_local_str(Next, Some("short"));
    let moonset = moon.get_moonset_local_str(Next, Some("short"));
    let mut moon_vec: Vec<String> = Vec::new();
    moon_vec.push("Moon:".to_string());
    moon_vec.push(format!("\n   - Rise                    : {:11}   Set   : {:11}   ", moonrise, moonset));
    moon_vec.push("\n\n".to_string());
    moon_vec
}

pub(crate) fn sun_section(observer: &Observer, time: &Time, environment: &Environment) -> Vec<String> {
    let sun = Sun::new(&observer, &time, &environment);
    let sunrise = sun.get_sunrise_local_str(Next, RiseSet, Some("short"));
    let sunset = sun.get_sunset_local_str(Next, RiseSet, Some("short"));
    let civil_tw_start = sun.get_sunrise_local_str(Next, CivilTwilight, Some("short"));
    let civil_tw_end = sun.get_sunset_local_str(Next, CivilTwilight, Some("short"));
    let nautical_tw_start = sun.get_sunrise_local_str(Next, NauticalTwilight, Some("short"));
    let nautical_tw_end = sun.get_sunset_local_str(Next, NauticalTwilight, Some("short"));
    let astronomical_tw_start = sun.get_sunrise_local_str(Next, AstronomicalTwilight, Some("short"));
    let astronomical_tw_end = sun.get_sunset_local_str(Next, AstronomicalTwilight, Some("short"));
    let mut sun_vec: Vec<String> = Vec::new();
    sun_vec.push("Sun:".to_string());
    sun_vec.push(format!("\n   - Set                     : {:11}   Rise  : {:11}   ", sunset, sunrise));
    sun_vec.push(format!("\n   - Civil Tw end            : {:11}   start : {:11}   ", civil_tw_end, civil_tw_start));
    sun_vec.push(format!("\n   - Nautical Tw end         : {:11}   start : {:11}   ", nautical_tw_end, nautical_tw_start));
    sun_vec.push(format!("\n   - Astronomical Tw end     : {:11}   start : {:11}   ", astronomical_tw_end, astronomical_tw_start));
    sun_vec.push("\n\n".to_string());
    sun_vec
}

pub(crate) fn darkness_section(observer: &Observer, time: &Time, environment: &Environment) -> Vec<String> {
    let darkness = Darkness::new(&observer, &time, &environment);
    let sun = Sun::new(&observer, &time, &environment);
    let astronomical_dso_start = darkness.get_darkness_local_astronomical_start_str(Some("short"));
    let astronomical_dso_end = darkness.get_darkness_local_astronomical_end_str(Some("short"));
    let nautical_dso_start = darkness.get_darkness_local_nautical_start_str(Some("short"));
    let nautical_dso_end = darkness.get_darkness_local_nautical_end_str(Some("short"));
    let astronomical_nb_start = sun.get_sunset_local_str(Next, AstronomicalTwilight, Some("short"));
    let astronomical_nb_end = sun.get_sunrise_local_str(Next, AstronomicalTwilight, Some("short"));
    let nautical_nb_start = sun.get_sunset_local_str(Next, NauticalTwilight, Some("short"));
    let nautical_nb_end = sun.get_sunrise_local_str(Next, NauticalTwilight, Some("short"));
    let mut dark: Vec<String> = Vec::new();
    dark.push("Darkness:".to_string());
    dark.push(format!("\n   - DSO Astronomical   start: {:11}   end   : {:11}", astronomical_dso_start, astronomical_dso_end));
    dark.push(format!("\n   - DSO Nautical       start: {:11}   end   : {:11}", nautical_dso_start, nautical_dso_end));
    // TODO Ignore moon in calculations for narrow band
    dark.push(format!("\n"));
    dark.push(format!("\n   - NB  Astronomical   start: {:11}   end   : {:11}", astronomical_nb_start, astronomical_nb_end));
    dark.push(format!("\n   - NB  Nautical       start: {:11}   end   : {:11}", nautical_nb_start, nautical_nb_end));
    dark
}

pub fn darkness_report(observer: &Observer, time: &Time, environment: &Environment) {
    // Header
    let header_lines = header_section();
    let mut lines = header_lines.join("");

    // Observer
    let observer_lines = observer_section(&observer);
    lines = lines + &*observer_lines.join("");

    // Environment
    let environment_lines = environment_section(&environment);
    lines = lines + &*environment_lines.join("");

    // Night
    let night_lines = night_section(&time);
    lines = lines + &*night_lines.join("");

    // Sun
    let sun_lines = sun_section(&observer, &time, &environment);
    lines = lines + &*sun_lines.join("");

    // Moon
    let moon_lines = moon_section(&observer, &time, &environment);
    lines = lines + &*moon_lines.join("");

    // Darkness
    let darkness_lines = darkness_section(&observer, &time, &environment);
    lines = lines + &*darkness_lines.join("");

    let mut f = File::create("skycalc.txt").expect("Unable to create file");
    f.write_all(lines.as_bytes()).expect("Unable to write data");
}

// TODO Implement up tonight report based on constraints
// TODO Add targets
pub fn up_tonight_report(observer: Observer, time: Time, environment: Environment,
                         constraints: Constraints) {

}