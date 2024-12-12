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

use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct Environment {
    #[serde(default = "default_temperature", deserialize_with = "deserialize_temperature")]
    pub temperature: i64,
    #[serde(default = "default_humidity", deserialize_with = "deserialize_humidity")]
    pub humidity: i64,
    #[serde(default = "default_pressure", deserialize_with = "deserialize_pressure")]
    pub pressure: i64,
}

// Default value functions for Environment fields
pub fn default_temperature() -> i64 {
    20
}

pub fn default_humidity() -> i64 {
    50
}

pub fn default_pressure() -> i64 {
    1010
}

fn deserialize_temperature<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<i64> = Option::deserialize(deserializer)?;
    // If the value is None (either missing or null), use the default value
    match value {
        Some(value) => Ok(value),
        None => Ok(default_temperature()), // Use the default value
    }
}

fn deserialize_humidity<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<i64> = Option::deserialize(deserializer)?;
    // If the value is None (either missing or null), use the default value
    match value {
        Some(value) => Ok(value),
        None => Ok(default_humidity()), // Use the default value
    }
}

fn deserialize_pressure<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<i64> = Option::deserialize(deserializer)?;
    // If the value is None (either missing or null), use the default value
    match value {
        Some(value) => Ok(value),
        None => Ok(default_pressure()), // Use the default value
    }
}

impl Environment {
    pub fn new(self, pressure: i64, temperature: i64, humidity: i64) -> Environment {
        Environment {pressure, temperature, humidity, ..self}
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "temperature: {} C, humidity: {} %, pressure: {} mbar",
                   self.temperature, self.humidity, self.pressure)
    }
}