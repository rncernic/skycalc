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

// TODO Implement test
// TODO Convert local time read from yaml file to UTC
#![allow(dead_code, unused_variables)]

use std::fs::File;
use std::io::Read;
use serde::{Deserialize};
use std::error::Error;
use crate::constraint::{default_frac_observable_time,
                        default_max_altitude,
                        default_max_size,
                        default_max_targets,
                        default_min_altitude,
                        default_min_size,
                        default_moon_separation,
                        default_use_darkness,
                        Constraints};
use crate::environment::{default_humidity,
                         default_pressure,
                         default_temperature,
                         Environment};
use crate::observer::{default_elevation,
                      default_lat,
                      default_lon,
                      default_name,
                      default_timezone,
                      Observer};
use crate::time::{Time};

pub const DEFAULT_TARGET_LIST: &str = "OpenNGC";
pub const DEFAULT_TYPE_FILTER: &str = "";
pub const DEFAULT_OUTPUT_DIR: &str = "output";

#[derive(Debug, Deserialize)]
struct Config {
    observer: Observer,
    time: Time,
    environment: Environment,
    constraints: Constraints
}

// Function to return default values for Config
fn default_config() -> (Observer, Time, Environment, Constraints) {
    (
        Observer {
            name: default_name(),
            latitude: default_lat(),
            longitude: default_lon(),
            elevation: default_elevation(),
            timezone: default_timezone()
        },
        Time::default(),
        Environment {
            temperature: default_temperature(),
            humidity: default_humidity(),
            pressure: default_pressure(),
        },
        Constraints {
            min_altitude: default_min_altitude(),
            max_altitude: default_max_altitude(),
            min_size: default_min_size(),
            max_size: default_max_size(),
            moon_separation: default_moon_separation(),
            frac_observable_time: default_frac_observable_time(),
            max_targets: default_max_targets(),
            use_darkness: default_use_darkness()
        }
    )
}

pub fn load_from_yaml(file_path: &str) -> Result<(Observer, Time, Environment, Constraints), Box<dyn Error>> {
    let mut contents = String::new();

    // Try to open the file
    match File::open(file_path) {
        Ok(mut file) => {
            file.read_to_string(&mut contents)?;
            let config: Config = serde_yaml::from_str(&contents)?;
            Ok((config.observer, config.time, config.environment, config.constraints))
        }
        Err(_) => {
            // File not found or unreadable, use default values
            println!("YAML configuration file not found. Using default values.");
            Ok(default_config())
        }
    }
}