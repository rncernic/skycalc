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

use std::fmt;
use std::fmt::{Display, Formatter};
use crate::constraints::Constraint;
use crate::darkness::darkness_utc;
use crate::datetime::{format_hm, ymd_hms_from_jd, DateTime};
use crate::environment::Environment;
use crate::location::Location;
use crate::sun::{sun_hour_angle, sun_position_from_jd, sunrise_utc_grid, sunset_utc_grid, SunRS};
use crate::moon::{moon_position_low_precision, moon_position_high_precision, moonrise_utc_grid, MoonRS, moonset_utc_grid};

#[derive(Debug, Clone, Default)]
pub struct Observer {
    pub observatory_name: Option<String>,
    pub output_dir: String,
    pub target_date: DateTime,
    pub location: Location,
    pub environment: Environment,
    pub constraint: Constraint
}

impl Display for Observer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        write!(f, "")
    }
}

impl Observer {
    pub fn new(observatory_name: String, output_dir: String, target_date:DateTime,
               location: Location, environment: Environment,
               constraint: Constraint) -> Self {
        Self { observatory_name: Some(observatory_name), output_dir, target_date, location, environment, constraint }
    }

    pub fn get_sun_position(&self) -> (f64, f64) {
        let (ra, dec) = sun_position_from_jd(self.target_date.jd);
        (ra, dec)
    }

    pub fn get_sun_hour_angle(&self) -> f64 {
        sun_hour_angle(self.location.latitude, self.get_sun_position().1)
    }

    pub fn get_moon_position_low_precision(&self) -> (f64, f64) {
        moon_position_low_precision(self.target_date.jd2000_century)
    }

    pub fn get_moon_position_high_precision(&self) -> (f64, f64) {
        let (ra, dec, _) = moon_position_high_precision(self.target_date.jd2000_century);
        (ra, dec)
    }

    pub fn get_moon_distance(&self) -> f64 {
        let (_, _, d) = moon_position_high_precision(self.target_date.jd2000_century);
        d
    }

    pub fn get_moonrise_moonset_utc_grid(&self) -> (f64) {
        moonrise_utc_grid(
            self.target_date.jd,
            self.location.latitude,
            self.location.longitude,
            self.location.timezone
        ).unwrap()
    }

    pub fn get_moonrise_local_grid(&self) -> Result<f64, MoonRS> {
        let rise_utc = moonrise_utc_grid(
            self.target_date.jd,
            self.location.latitude,
            self.location.longitude,
            self.location.timezone
        );
        match rise_utc {
            Ok(rise) => Ok(rise + self.location.timezone / 24.0),
            Err(e) => Err(e),
        }

    }

    pub fn get_moonset_local_grid(&self) -> Result<f64, MoonRS> {
        let set_utc = moonset_utc_grid(
            self.target_date.jd,
            self.location.latitude,
            self.location.longitude,
            self.location.timezone
        );
        match set_utc {
            Ok(set) => Ok(set + self.location.timezone / 24.0),
            Err(e) => Err(e),
        }

    }

    pub fn get_sunrise_local_grid(&self, horizon: f64) -> Result<f64, SunRS> {
        let rise_utc = sunrise_utc_grid(
            self.target_date.jd,
            self.location.latitude,
            self.location.longitude,
            horizon,
            self.location.timezone
        );
        match rise_utc {
            Ok(rise) => Ok(rise + self.location.timezone / 24.0),
            Err(e) => Err(e),
        }
    }

    pub fn get_sunset_local_grid(&self, horizon: f64) -> Result<f64, SunRS> {
        let set_utc = sunset_utc_grid(
            self.target_date.jd,
            self.location.latitude,
            self.location.longitude,
            horizon,
            self.location.timezone
        );
        match set_utc {
            Ok(set) => Ok(set + self.location.timezone / 24.0),
            Err(e) => Err(e),
        }
    }

    pub fn get_darkness_astronomical_utc(&self) -> (f64, f64) {
        let (utc_rise, utc_set) = darkness_utc(self.location.latitude, self.location.longitude, self.target_date.jd, 3);
        (utc_rise, utc_set)
    }

    pub fn get_darkness_nautical_utc(&self) -> (f64, f64) {
        let (utc_rise, utc_set) = darkness_utc(self.location.latitude, self.location.longitude, self.target_date.jd, 2);
        (utc_rise, utc_set)
    }

    pub fn get_darkness_civil_utc(&self) -> (f64, f64) {
        let (utc_rise, utc_set) = darkness_utc(self.location.latitude, self.location.longitude, self.target_date.jd, 1);
        (utc_rise, utc_set)
    }

    pub fn get_darkness_rise_set_utc(&self) -> (f64, f64) {
        let (utc_rise, utc_set) = darkness_utc(self.location.latitude, self.location.longitude, self.target_date.jd, 0);
        (utc_rise, utc_set)
    }

    pub fn get_sunrise_sunset_str(&self) -> (String, String) {

        let horizon = -0.8333;
        let mut sunset = "".to_string();
        let mut sunrise = "".to_string();

        match self.get_sunset_local_grid(horizon) {
            Ok(set) => sunset = format_hm(ymd_hms_from_jd(set).0,
                                          ymd_hms_from_jd(set).1,
                                          ymd_hms_from_jd(set).2,
                                          ymd_hms_from_jd(set).3,
                                          ymd_hms_from_jd(set).4,
                                          ymd_hms_from_jd(set).5).to_string(),
            Err(e) => sunset = "Never set".to_string(),
        };

        match self.get_sunrise_local_grid(horizon) {
            Ok(rise) => sunrise = format_hm(ymd_hms_from_jd(rise).0,
                                          ymd_hms_from_jd(rise).1,
                                          ymd_hms_from_jd(rise).2,
                                          ymd_hms_from_jd(rise).3,
                                          ymd_hms_from_jd(rise).4,
                                          ymd_hms_from_jd(rise).5).to_string(),
            Err(e) => sunrise = "Never rise".to_string(),
        }

        if sunset == "Never set" {
            sunrise = sunset.to_string();
            (sunrise, sunset)
        } else {
            if sunrise == "Never rise" {
                sunset = sunrise.to_string();
                (sunrise, sunset)
            } else {
                (sunrise, sunset)
            }
        }
    }

    pub fn get_civil_rise_set_str(&self) -> (String, String) {

        let horizon = -6.0;
        let mut sunset = "".to_string();
        let mut sunrise = "".to_string();

        match self.get_sunset_local_grid(horizon) {
            Ok(set) => sunset = format_hm(ymd_hms_from_jd(set).0,
                                          ymd_hms_from_jd(set).1,
                                          ymd_hms_from_jd(set).2,
                                          ymd_hms_from_jd(set).3,
                                          ymd_hms_from_jd(set).4,
                                          ymd_hms_from_jd(set).5).to_string(),
            Err(e) => sunset = "Never set".to_string(),
        };

        match self.get_sunrise_local_grid(horizon) {
            Ok(rise) => sunrise = format_hm(ymd_hms_from_jd(rise).0,
                                            ymd_hms_from_jd(rise).1,
                                            ymd_hms_from_jd(rise).2,
                                            ymd_hms_from_jd(rise).3,
                                            ymd_hms_from_jd(rise).4,
                                            ymd_hms_from_jd(rise).5).to_string(),
            Err(e) => sunrise = "Never rise".to_string(),
        }

        if sunset == "Never set" {
            sunrise = sunset.to_string();
            (sunrise, sunset)
        } else {
            if sunrise == "Never rise" {
                sunset = sunrise.to_string();
                (sunrise, sunset)
            } else {
                (sunrise, sunset)
            }
        }
    }

    pub fn get_nautical_rise_set_str(&self) -> (String, String) {

        let horizon = -12.0;
        let mut sunset = "".to_string();
        let mut sunrise = "".to_string();

        match self.get_sunset_local_grid(horizon) {
            Ok(set) => sunset = format_hm(ymd_hms_from_jd(set).0,
                                          ymd_hms_from_jd(set).1,
                                          ymd_hms_from_jd(set).2,
                                          ymd_hms_from_jd(set).3,
                                          ymd_hms_from_jd(set).4,
                                          ymd_hms_from_jd(set).5).to_string(),
            Err(e) => sunset = "Never set".to_string(),
        };

        match self.get_sunrise_local_grid(horizon) {
            Ok(rise) => sunrise = format_hm(ymd_hms_from_jd(rise).0,
                                            ymd_hms_from_jd(rise).1,
                                            ymd_hms_from_jd(rise).2,
                                            ymd_hms_from_jd(rise).3,
                                            ymd_hms_from_jd(rise).4,
                                            ymd_hms_from_jd(rise).5).to_string(),
            Err(e) => sunrise = "Never rise".to_string(),
        }

        if sunset == "Never set" {
            sunrise = sunset.to_string();
            (sunrise, sunset)
        } else {
            if sunrise == "Never rise" {
                sunset = sunrise.to_string();
                (sunrise, sunset)
            } else {
                (sunrise, sunset)
            }
        }
    }

    pub fn get_astro_rise_set_str(&self) -> (String, String) {

        let horizon = -18.0;
        let mut sunset = "".to_string();
        let mut sunrise = "".to_string();

        match self.get_sunset_local_grid(horizon) {
            Ok(set) => sunset = format_hm(ymd_hms_from_jd(set).0,
                                          ymd_hms_from_jd(set).1,
                                          ymd_hms_from_jd(set).2,
                                          ymd_hms_from_jd(set).3,
                                          ymd_hms_from_jd(set).4,
                                          ymd_hms_from_jd(set).5).to_string(),
            Err(e) => sunset = "Never set".to_string(),
        };

        match self.get_sunrise_local_grid(horizon) {
            Ok(rise) => sunrise = format_hm(ymd_hms_from_jd(rise).0,
                                            ymd_hms_from_jd(rise).1,
                                            ymd_hms_from_jd(rise).2,
                                            ymd_hms_from_jd(rise).3,
                                            ymd_hms_from_jd(rise).4,
                                            ymd_hms_from_jd(rise).5).to_string(),
            Err(e) => sunrise = "Never rise".to_string(),
        }

        if sunset == "Never set" {
            sunrise = sunset.to_string();
            (sunrise, sunset)
        } else {
            if sunrise == "Never rise" {
                sunset = sunrise.to_string();
                (sunrise, sunset)
            } else {
                (sunrise, sunset)
            }
        }
    }

}