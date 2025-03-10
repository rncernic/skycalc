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

use crate::application::environment::Environment;
use crate::application::moon::moon_alt_az_grid_utc;
use crate::application::observer::Observer;
use crate::application::sun::{sun_alt_az_grid_utc, TwilightType};
use crate::application::sun::TwilightType::{AstronomicalTwilight, CivilTwilight, NauticalTwilight, RiseSet};
use crate::application::time::Time;

#[derive(Debug)]
pub struct Darkness<'a> {
    pub observer: &'a Observer,
    pub time: &'a Time,
    pub environment: &'a Environment,
}

impl<'a> Darkness<'a> {
    pub fn new(observer: &'a Observer, time: &'a Time, environment: &'a Environment) -> Self {
        Self {
            observer,
            time,
            environment,
        }
    }

    pub fn darkness_utc(&self, twilight: TwilightType) -> (f64, f64) {
        const NUM_POINTS: usize = 1440;
        let target_night_start = (self.time.to_jd() + 0.5).floor() + 3.0 / 24.0;
        let target_night_end = target_night_start + 1.0;

        let sun = sun_alt_az_grid_utc(
            self.observer.latitude,
            self.observer.longitude,
            target_night_start,
            target_night_end,
            NUM_POINTS,
        );

        let moon = moon_alt_az_grid_utc(
            self.observer.latitude,
            self.observer.longitude,
            target_night_start,
            target_night_end,
            NUM_POINTS,
        );

        let darkness: Vec<f64> = sun
            .iter()
            .zip(moon.iter())
            .filter_map(|(sun, moon)| {
                if sun.1 <= twilight.angle() && moon.1 <= 0.125 {
                    Some(sun.0)
                } else {
                    None
                }
            })
            .collect();

        if darkness.is_empty() {
            (0.0, 0.0)
        } else {
            let start = darkness.iter().cloned().reduce(f64::min).unwrap_or(0.0);
            let end = darkness.iter().cloned().reduce(f64::max).unwrap_or(0.0);
            (start, end)
        }
    }

    fn darkness_utc_helper(&self, twilight: TwilightType) -> (f64, f64) {
        self.darkness_utc(twilight)
    }

    pub fn get_darkness_utc_riseset(&self) -> (f64, f64) {
        self.darkness_utc_helper(RiseSet)
    }

    pub fn get_darkness_utc_civil(&self) -> (f64, f64) {
        self.darkness_utc_helper(CivilTwilight)
    }

    pub fn get_darkness_utc_nautical(&self) -> (f64, f64) {
        self.darkness_utc_helper(NauticalTwilight)
    }

    pub fn get_darkness_utc_astronomical(&self) -> (f64, f64) {
        self.darkness_utc_helper(AstronomicalTwilight)
    }

    pub fn get_darkness_utc_astronomical_or_nautical(&self) -> (&'static str, (f64, f64)) {
        let astronomical_darkness = self.get_darkness_utc_astronomical();
        let nautical_darkness = self.get_darkness_utc_nautical();
        if astronomical_darkness == (0.0, 0.0) {
            if nautical_darkness == (0.0, 0.0) {
                ("none", (0.0, 0.0))
            } else {
                ("nautical", self.get_darkness_utc_nautical())
            }
        } else {
            ("astronomical", astronomical_darkness)
        }
    }

    fn to_local_time(&self, utc_darkness: (f64, f64)) -> (f64, f64) {
        match utc_darkness {
            (start, end) if start == 0.0 && end == 0.0 => (0.0, 0.0),
            (start, end) => {
                let offset = self.observer.timezone / 24.0;
                (start + offset, end + offset)
            }
        }
    }

    pub fn get_darkness_local_riseset(&self) -> (f64, f64) {
        self.to_local_time(self.get_darkness_utc_riseset())
    }

    pub fn get_darkness_local_civil(&self) -> (f64, f64) {
        self.to_local_time(self.get_darkness_utc_civil())
    }

    pub fn get_darkness_local_nautical(&self) -> (f64, f64) {
        self.to_local_time(self.get_darkness_utc_nautical())
    }

    pub fn get_darkness_local_astronomical(&self) -> (f64, f64) {
        self.to_local_time(self.get_darkness_utc_astronomical())
    }

    pub fn get_darkness_local_astronomical_or_nautical(&self) -> (&str, (f64, f64)) {
        let utc = self.get_darkness_utc_astronomical_or_nautical();
        (utc.0, self.to_local_time(utc.1))
    }

    fn format_darkness_time<F>(&self, time_selector: F, start: bool, format: Option<&str>) -> String
    where
        F: Fn() -> (f64, f64),
    {
        let (start_time, end_time) = time_selector();
        let time = if start { start_time } else { end_time };
        Time::from_jd(time).to_string(format)
    }

    pub fn get_darkness_utc_riseset_start_str(&self, format: Option<&str>) -> String {
        self.format_darkness_time(|| self.get_darkness_utc_riseset(), true, format)
    }

    pub fn get_darkness_utc_riseset_end_str(&self, format: Option<&str>) -> String {
        self.format_darkness_time(|| self.get_darkness_utc_riseset(), false, format)
    }

    pub fn get_darkness_utc_civil_start_str(&self, format: Option<&str>) -> String {
        self.format_darkness_time(|| self.get_darkness_utc_civil(), true, format)
    }

    pub fn get_darkness_local_utc_end_str(&self, format: Option<&str>) -> String {
        self.format_darkness_time(|| self.get_darkness_utc_civil(), false, format)
    }

    pub fn get_darkness_utc_nautical_start_str(&self, format: Option<&str>) -> String {
        self.format_darkness_time(|| self.get_darkness_utc_nautical(), true, format)
    }

    pub fn get_darkness_utc_nautical_end_str(&self, format: Option<&str>) -> String {
        self.format_darkness_time(|| self.get_darkness_utc_nautical(), false, format)
    }

    pub fn get_darkness_utc_astronomical_start_str(&self, format: Option<&str>) -> String {
        self.format_darkness_time(|| self.get_darkness_utc_astronomical(), true, format)
    }

    pub fn get_darkness_utc_astronomical_end_str(&self, format: Option<&str>) -> String {
        self.format_darkness_time(|| self.get_darkness_utc_astronomical(), false, format)
    }

    pub fn get_darkness_utc_astronomical_or_nautical_str(
        &self,
        format: Option<&str>,
    ) -> (&str, String) {
        let local = self.get_darkness_utc_astronomical_or_nautical();
        (local.0, self.format_darkness_time(|| local.1, true, format))
    }
    pub fn get_darkness_local_riseset_start_str(&self, format: Option<&str>) -> String {
        self.format_darkness_time(|| self.get_darkness_local_riseset(), true, format)
    }

    pub fn get_darkness_local_riseset_end_str(&self, format: Option<&str>) -> String {
        self.format_darkness_time(|| self.get_darkness_local_riseset(), false, format)
    }

    pub fn get_darkness_local_civil_start_str(&self, format: Option<&str>) -> String {
        self.format_darkness_time(|| self.get_darkness_local_civil(), true, format)
    }

    pub fn get_darkness_local_civil_end_str(&self, format: Option<&str>) -> String {
        self.format_darkness_time(|| self.get_darkness_local_civil(), false, format)
    }

    pub fn get_darkness_local_nautical_start_str(&self, format: Option<&str>) -> String {
        self.format_darkness_time(|| self.get_darkness_local_nautical(), true, format)
    }

    pub fn get_darkness_local_nautical_end_str(&self, format: Option<&str>) -> String {
        self.format_darkness_time(|| self.get_darkness_local_nautical(), false, format)
    }

    pub fn get_darkness_local_astronomical_start_str(&self, format: Option<&str>) -> String {
        self.format_darkness_time(|| self.get_darkness_local_astronomical(), true, format)
    }

    pub fn get_darkness_local_astronomical_end_str(&self, format: Option<&str>) -> String {
        self.format_darkness_time(|| self.get_darkness_local_astronomical(), false, format)
    }

    pub fn get_darkness_local_astronomical_or_nautical_start_str(
        &self,
        format: Option<&str>,
    ) -> (&str, String) {
        let local = self.get_darkness_local_astronomical_or_nautical();
        (local.0, self.format_darkness_time(|| local.1, true, format))
    }

    pub fn get_darkness_local_astronomical_or_nautical_end_str(
        &self,
        format: Option<&str>,
    ) -> (&str, String) {
        let local = self.get_darkness_local_astronomical_or_nautical();
        (
            local.0,
            self.format_darkness_time(|| local.1, false, format),
        )
    }
}
