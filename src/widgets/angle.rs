use crate::application::observer::degrees_from_str;
use fltk::input::Input;
use fltk::prelude::*;
use std::ops::{Deref, DerefMut};
#[derive(Clone)]
pub struct AngleInput {
    pub angle_input: Input,
    pub min: f64,
    pub max: f64,
}

impl Deref for AngleInput {
    type Target = Input;
    fn deref(&self) -> &Self::Target {
        &self.angle_input
    }
}

impl DerefMut for AngleInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.angle_input
    }
}

impl AngleInput {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str, min: f64, max: f64) -> AngleInput {
        let mut input = Input::new(x, y, w, h, label);
        input.set_maximum_size(14); // max size YYYY-MM-DD
        input.set_value("0.000000"); // set initial value
        AngleInput { angle_input: input, min, max }
    }

    pub fn get_angle(&mut self) -> f64 {
        degrees_from_str(&self.angle_input.value(), self.min, self.max)
    }

    pub fn validate(&mut self) {
        let angle = degrees_from_str(&self.angle_input.value(), self.min, self.max);
        self.angle_input.set_value(&format!("{:.6}", &angle));
    }
}
