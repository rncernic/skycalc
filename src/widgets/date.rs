use std::ops::{Deref, DerefMut};
use fltk::input::Input;
use fltk::prelude::*;
use crate::application::time::{from_str_or_now};

#[derive(Clone)]
pub struct DateInput {
    pub date_input: Input
}

impl Deref for DateInput {
    type Target = Input;
    fn deref(&self) -> &Self::Target {
        &self.date_input
    }
}

impl DerefMut for DateInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.date_input
    }
}

impl DateInput {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> DateInput {
        let mut input = Input::new(x, y, w, h, label);
        input.set_maximum_size(10); // max size YYYY-MM-DD
        input.set_value(""); // set initial value
        DateInput { date_input: input }
    }

    pub fn validate(&mut self) {
        let date_str = from_str_or_now(&self.date_input.value()).to_string(Some("yyyymmdd"));
        self.date_input.set_value(&date_str);
    }

    pub fn get_day(&self) -> u64 {
        from_str_or_now(&self.date_input.value()).day
    }

    pub fn get_month(&self) -> u64 {
        from_str_or_now(&self.date_input.value()).month
    }

    pub fn get_year(&self) -> i64 {
        from_str_or_now(&self.date_input.value()).year
    }
}
