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

use std::fs;
use serde_yaml;
use serde::Deserialize;
use crate::angle::Degrees;
use crate::datetime::{jd2000_century_from_jd, jd2000_from_date, jd_from_date, naive_to_date};
use crate::observer::Observer;

pub const DEFAULT_TARGET_LIST: &str = "OpenNGC";
pub const DEFAULT_TYPE_FILTER: &str = "";
pub const DEFAULT_OUTPUT_DIR: &str = "output";
pub const DEFAULT_LATITUDE: &str = "0.0";
pub const DEFAULT_LONGITUDE: &str = "0.0";
pub const DEFAULT_ELEVATION: i64 = 0;
pub const DEFAULT_TIMEZONE: f64 = 0.0;
pub const DEFAULT_PRESSURE: i64 = 1000;
pub const DEFAULT_TEMPERATURE: i64 = 25;
pub const DEFAULT_RELATIVE_HUMIDITY: i64 = 50;
pub const DEFAULT_ALTITUDE_CONSTRAINT_MIN: i64 = 30;
pub const DEFAULT_ALTITUDE_CONSTRAINT_MAX: i64 = 80;
pub const DEFAULT_SIZE_CONSTRAINT_MIN: i64 = 10;
pub const DEFAULT_SIZE_CONSTRAINT_MAX: i64 = 300;
pub const DEFAULT_MOON_SEPARATION_MIN: i64 = 45;
pub const DEFAULT_FRACTION_OF_TIME_OBSERVABLE_THRESHOLD: i64 = 50;
pub const DEFAULT_MAX_NUMBER_WITHIN_THRESHOLD: i64 = 50;
pub const DEFAULT_USE_DARKNESS: bool = true;

#[derive(Debug, Deserialize)]
struct ConfigLocation {
    latitude: Option<String>,
    longitude: Option<String>,
    elevation: Option<i64>,
    timezone: Option<f64>
}

#[derive(Debug, Deserialize)]
struct ConfigEnvironment {
    pressure: Option<i64>,
    temperature: Option<i64>,
    humidity: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct ConfigConstraints {
    altitude_constraint_min: Option<i64>,
    altitude_constraint_max: Option<i64>,
    size_constraint_min: Option<i64>,
    size_constraint_max: Option<i64>,
    moon_separation_min: Option<i64>,
    use_darkness: Option<bool>,
    fraction_of_time_observable_threshold: Option<i64>,
    max_number_within_threshold: Option<i64>
}

#[derive(Debug, Deserialize)]
struct Configuration {
    observatory_name: Option<String>,
    observation_date: Option<String>,
    target_list: Option<String>,
    filter_type: Option<String>,
    output_dir: Option<String>,
    location: ConfigLocation,
    environment: ConfigEnvironment,
    constraints: ConfigConstraints
}

pub fn load_yaml <'a> (obs: &'a mut Observer, file: &str) {

    let yaml_str = fs::read_to_string(file).unwrap();
    // Parse the YAML content into the Config struct
    let config_yaml: Configuration = serde_yaml::from_str(&yaml_str).unwrap();

    // Todo Use NOW if no observation_date ie provided
    log::debug!("Filter: {:?}", config_yaml.filter_type.unwrap_or(DEFAULT_TYPE_FILTER.to_string()));

    // General
    obs.observatory_name = config_yaml.observatory_name
        .map(|s| s.to_string())  // Converts Option<String> to Option<String>
        .or(Some("My Observatory".to_string())); // Provides a default value if None

    // (obs.target_date.date.year, obs.target_date.date.month, obs.target_date.date.day) =
    //     naive_to_date(config_yaml.observation_date.unwrap_or("".to_string()));
    let (y, m, d) = naive_to_date(config_yaml.observation_date.unwrap_or("".to_string()));
    obs.target_date.date.year = y;
    obs.target_date.date.month = m;
    obs.target_date.date.day = d;
    obs.target_date.time.hour = 12;
    obs.target_date.time.min = 0;
    obs.target_date.time.sec = 0.0;
    obs.target_date.frac_day = (obs.target_date.time.hour as f64 +
        obs.target_date.time.min as f64 / 60.0 +
        obs.target_date.time.sec / 3600.0) / 24.0;
    obs.target_date.day_decimal = d as f64 + obs.target_date.frac_day;
    obs.target_date.jd = jd_from_date(y, m, obs.target_date.day_decimal);
    obs.target_date.jd2000= jd2000_from_date(y, m, obs.target_date.day_decimal);
    obs.target_date.jd2000_century = jd2000_century_from_jd(obs.target_date.jd);

    obs.output_dir = config_yaml.output_dir.unwrap_or(DEFAULT_OUTPUT_DIR.to_string());
    // Location
    obs.location.longitude = Degrees::from_str(config_yaml.location.longitude.unwrap_or(DEFAULT_LONGITUDE.to_string()), -180.0, 180.0).value;
    obs.location.latitude = Degrees::from_str(config_yaml.location.latitude.unwrap_or(DEFAULT_LONGITUDE.to_string()), -90.0, 90.0).value;
    obs.location.elevation = config_yaml.location.elevation.unwrap_or(DEFAULT_ELEVATION);
    obs.location.timezone = config_yaml.location.timezone.unwrap_or(DEFAULT_TIMEZONE);
    // Environment
    obs.environment.pressure = config_yaml.environment.pressure.unwrap_or(DEFAULT_PRESSURE);
    obs.environment.temperature = config_yaml.environment.temperature.unwrap_or(DEFAULT_TEMPERATURE);
    obs.environment.relative_humidity = config_yaml.environment.humidity.unwrap_or(DEFAULT_RELATIVE_HUMIDITY);
    // Constraints
    obs.constraint.altitude_constraint_min = config_yaml.constraints.altitude_constraint_min.unwrap_or(DEFAULT_ALTITUDE_CONSTRAINT_MIN);
    obs.constraint.altitude_constraint_max = config_yaml.constraints.altitude_constraint_max.unwrap_or(DEFAULT_ALTITUDE_CONSTRAINT_MAX);
    obs.constraint.size_constraint_min = config_yaml.constraints.size_constraint_min.unwrap_or(DEFAULT_SIZE_CONSTRAINT_MIN);
    obs.constraint.size_constraint_max = config_yaml.constraints.size_constraint_max.unwrap_or(DEFAULT_SIZE_CONSTRAINT_MAX);
    obs.constraint.moon_separation_min = config_yaml.constraints.moon_separation_min.unwrap_or(DEFAULT_MOON_SEPARATION_MIN);
    obs.constraint.use_darkness = config_yaml.constraints.use_darkness.unwrap_or(DEFAULT_USE_DARKNESS);
    obs.constraint.fraction_of_time_observable_threshold = config_yaml.constraints.fraction_of_time_observable_threshold.unwrap_or(DEFAULT_FRACTION_OF_TIME_OBSERVABLE_THRESHOLD);
    obs.constraint.max_number_within_threshold = config_yaml.constraints.max_number_within_threshold.unwrap_or(DEFAULT_MAX_NUMBER_WITHIN_THRESHOLD);
}