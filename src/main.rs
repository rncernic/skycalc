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

// https://github.com/boom-astro/flare/tree/main

// TODO Create txt report
// TODO Check for extreme latitudes
// TODO Implement targets

#![allow(dead_code, unused_variables)]

mod observer;
mod time;
mod environment;
mod constraint;
mod config;
mod version;
mod reports;
mod darkness;
mod sun;
mod utils;
mod transformations;
mod moon;
mod earth;

use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{TimeZone, Utc};
use simple_logger as sl;
use observer::Observer;
use time::{Time};
use crate::config::{load_from_yaml};
use crate::darkness::{Darkness};
use crate::environment::{Environment};
use crate::moon::{Moon};
use crate::reports::{darkness_report};
use crate::sun::{Sun};
use crate::sun::RiseSetType::{Nearest, Next, Previous};
use crate::sun::TwilightType::{AstronomicalTwilight, CivilTwilight, NauticalTwilight, RiseSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init log
    sl::init().expect("Unable to init logger");

    // Start counting time to evaluate performance
    let p_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let (observer, time, environment, constraints) = load_from_yaml("config.yaml")?;
    let sun = Sun::new(&observer, &time, &environment);
    let moon = Moon::new(&observer, &time, &environment);

    log::info!("Observer        {:?}", observer.to_string_decimal());
    log::info!("Time            {:?}", time.to_string(Some("utc")));
    log::info!("Environment     {:?}", environment.to_string());
    log::info!("Constraints     {:?}", constraints.to_string());

    log::info!("*** Sunrise / Sunset ***");
    log::info!("--- Next ---");
    log::info!("RiseSet         {:?}  {:?}", sun.get_sunrise_utc_str(Next, RiseSet, Some("utc")), sun.get_sunset_utc_str(Next, RiseSet, Some("utc")));
    log::info!("Civil Tw        {:?}  {:?}", sun.get_sunrise_utc_str(Next, CivilTwilight, Some("utc")), sun.get_sunset_utc_str(Next, CivilTwilight, Some("utc")));
    log::info!("Nautical Tw     {:?}  {:?}", sun.get_sunrise_utc_str(Next, NauticalTwilight, Some("utc")), sun.get_sunset_utc_str(Next, NauticalTwilight, Some("utc")));
    log::info!("Astronomical Tw {:?}  {:?}", sun.get_sunrise_utc_str(Next, AstronomicalTwilight, Some("utc")), sun.get_sunset_utc_str(Next, AstronomicalTwilight, Some("utc")));
    log::info!("--- Previous ---");
    log::info!("RiseSet         {:?}  {:?}", sun.get_sunrise_utc_str(Previous, RiseSet, Some("utc")), sun.get_sunset_utc_str(Previous, RiseSet, Some("utc")));
    log::info!("Civil Tw        {:?}  {:?}", sun.get_sunrise_utc_str(Previous, CivilTwilight, Some("utc")), sun.get_sunset_utc_str(Previous, CivilTwilight, Some("utc")));
    log::info!("Nautical Tw     {:?}  {:?}", sun.get_sunrise_utc_str(Previous, NauticalTwilight, Some("utc")), sun.get_sunset_utc_str(Previous, NauticalTwilight, Some("utc")));
    log::info!("Astronomical Tw {:?}  {:?}", sun.get_sunrise_utc_str(Previous, AstronomicalTwilight, Some("utc")), sun.get_sunset_utc_str(Previous, AstronomicalTwilight, Some("utc")));
    log::info!("--- Nearest ---");
    log::info!("RiseSet         {:?}  {:?}", sun.get_sunrise_utc_str(Nearest, RiseSet, Some("utc")), sun.get_sunset_utc_str(Nearest, RiseSet, Some("utc")));
    log::info!("Civil Tw        {:?}  {:?}", sun.get_sunrise_utc_str(Nearest, CivilTwilight, Some("utc")), sun.get_sunset_utc_str(Nearest, CivilTwilight, Some("utc")));
    log::info!("Nautical Tw     {:?}  {:?}", sun.get_sunrise_utc_str(Nearest, NauticalTwilight, Some("utc")), sun.get_sunset_utc_str(Nearest, NauticalTwilight, Some("utc")));
    log::info!("Astronomical Tw {:?}  {:?}", sun.get_sunrise_utc_str(Nearest, AstronomicalTwilight, Some("utc")), sun.get_sunset_utc_str(Nearest, AstronomicalTwilight, Some("utc")));

    log::info!("*** Moon rise / set ***");
    log::info!("Next            {:?}  {:?}", moon.get_moonrise_utc_str(Next, Some("utc")), moon.get_moonset_utc_str(Next, Some("utc")));
    log::info!("Previous        {:?}  {:?}", moon.get_moonrise_utc_str(Previous, Some("utc")), moon.get_moonset_utc_str(Previous, Some("utc")));
    log::info!("Nearest         {:?}  {:?}", moon.get_moonrise_utc_str(Nearest, Some("utc")), moon.get_moonset_utc_str(Nearest, Some("utc")));

    darkness_report(&observer, &time, &environment);

    let p_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    log::debug!("Execution time: {:?}", p_end - p_start);

    Ok(())

}