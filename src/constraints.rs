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

#[derive(Debug, Copy, Clone, Default)]
pub struct Constraint {
    pub altitude_constraint_min: i64,
    pub altitude_constraint_max: i64,
    pub size_constraint_min: i64,
    pub size_constraint_max: i64,
    pub moon_separation_min: i64,
    pub use_darkness: bool,
    pub fraction_of_time_observable_threshold: i64,
    pub max_number_within_threshold: i64,
}

impl Constraint {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn constraint (self, altitude_constraint_min: i64, altitude_constraint_max: i64,
                       size_constraint_min: i64, size_constraint_max: i64, moon_separation_min: i64,
                       fraction_of_time_observable_threshold: i64, max_number_within_threshold: i64,
                       use_darkness: bool) -> Self {
        Self {altitude_constraint_min, altitude_constraint_max, size_constraint_min,
            size_constraint_max, moon_separation_min, fraction_of_time_observable_threshold,
            max_number_within_threshold, use_darkness, ..self}
    }
}