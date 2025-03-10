use std::ops::{Deref, DerefMut};
use fltk::enums::Align;
use fltk::frame;
use fltk::prelude::{WidgetBase, WidgetExt};

#[derive(Clone)]
pub struct Label {
    pub label: frame::Frame
}

impl Deref for Label {
    type Target = frame::Frame;
    fn deref(&self) -> &Self::Target {
        &self.label
    }
}

impl DerefMut for Label {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.label
    }
}

impl Label {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str, align: Align) -> Label {
        let mut label_frm = frame::Frame::new(x, y, w, h, label)
            .with_align(align);
        Label { label: label_frm }
    }
}