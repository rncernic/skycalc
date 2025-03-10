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
#![allow(dead_code, unused_variables)]

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Constraints {
    #[serde(
        default = "default_min_altitude",
        deserialize_with = "deserialize_min_altitude"
    )]
    pub min_altitude: i64, // 20
    #[serde(
        default = "default_max_altitude",
        deserialize_with = "deserialize_max_altitude"
    )]
    pub max_altitude: i64, // 80
    #[serde(
        default = "default_min_size",
        deserialize_with = "deserialize_min_size"
    )]
    pub min_size: i64, // 10
    #[serde(
        default = "default_max_size",
        deserialize_with = "deserialize_max_size"
    )]
    pub max_size: i64, // 300
    #[serde(
        default = "default_moon_separation",
        deserialize_with = "deserialize_moon_separation"
    )]
    pub moon_separation: i64, // 45
    #[serde(
        default = "default_frac_observable_time",
        deserialize_with = "deserialize_frac_observable_time"
    )]
    pub frac_observable_time: i64, // 50
    #[serde(
        default = "default_max_targets",
        deserialize_with = "deserialize_max_targets"
    )]
    pub max_targets: i64, // 50
    #[serde(
        default = "default_use_darkness",
        deserialize_with = "deserialize_use_darkness"
    )]
    pub use_darkness: bool, // false
}

pub fn default_min_altitude() -> i64 {
    20
}

pub fn default_max_altitude() -> i64 {
    80
}

pub fn default_min_size() -> i64 {
    10
}

pub fn default_max_size() -> i64 {
    300
}

pub fn default_moon_separation() -> i64 {
    45
}

pub fn default_frac_observable_time() -> i64 {
    50
}

pub fn default_max_targets() -> i64 {
    50
}

pub fn default_use_darkness() -> bool {
    false
}

fn deserialize_min_altitude<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<i64> = Option::deserialize(deserializer)?;
    // If the value is None (either missing or null), use the default value
    match value {
        Some(value) => Ok(value),
        None => Ok(default_min_altitude()), // Use the default value
    }
}

fn deserialize_max_altitude<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<i64> = Option::deserialize(deserializer)?;
    // If the value is None (either missing or null), use the default value
    match value {
        Some(value) => Ok(value),
        None => Ok(default_max_altitude()), // Use the default value
    }
}

fn deserialize_min_size<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<i64> = Option::deserialize(deserializer)?;
    // If the value is None (either missing or null), use the default value
    match value {
        Some(value) => Ok(value),
        None => Ok(default_min_size()), // Use the default value
    }
}

fn deserialize_max_size<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<i64> = Option::deserialize(deserializer)?;
    // If the value is None (either missing or null), use the default value
    match value {
        Some(value) => Ok(value),
        None => Ok(default_max_size()), // Use the default value
    }
}

fn deserialize_moon_separation<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<i64> = Option::deserialize(deserializer)?;
    // If the value is None (either missing or null), use the default value
    match value {
        Some(value) => Ok(value),
        None => Ok(default_moon_separation()), // Use the default value
    }
}

fn deserialize_frac_observable_time<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<i64> = Option::deserialize(deserializer)?;
    // If the value is None (either missing or null), use the default value
    match value {
        Some(value) => Ok(value),
        None => Ok(default_frac_observable_time()), // Use the default value
    }
}

fn deserialize_max_targets<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<i64> = Option::deserialize(deserializer)?;
    // If the value is None (either missing or null), use the default value
    match value {
        Some(value) => Ok(value),
        None => Ok(default_max_targets()), // Use the default value
    }
}

fn deserialize_use_darkness<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<bool> = Option::deserialize(deserializer)?;
    // If the value is None (either missing or null), use the default value
    match value {
        Some(value) => Ok(value),
        None => Ok(default_use_darkness()), // Use the default value
    }
}

impl Constraints {
    pub fn new(
        self,
        min_altitude: i64,
        max_altitude: i64,
        min_size: i64,
        max_size: i64,
        moon_separation: i64,
        frac_observable_time: i64,
        max_targets: i64,
        use_darkness: bool,
    ) -> Self {
        Self {
            min_altitude,
            max_altitude,
            min_size,
            max_size,
            moon_separation,
            frac_observable_time,
            max_targets,
            use_darkness,
            ..self
        }
    }
}

// TODO Update
impl std::fmt::Display for Constraints {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "min altitude: {} deg, max altitude: {} deg, min size: {} arcmin",
            self.min_altitude, self.max_altitude, self.min_size
        )
    }
}
