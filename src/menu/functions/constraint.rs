use std::cell::RefCell;
use std::rc::Rc;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt};
use fltk::{app, button, enums, window};
use fltk_evented::Listener;
use crate::application::application::Application;

pub fn handle_constraint(application: &mut Rc<RefCell<Application>>) -> bool {
    let mut window = window::Window::default()
        .with_label("Constraint setup")
        .with_size(290, 250)
        .center_screen();
    window.make_modal(true);

    // Apply button
    let mut btn_apply: Listener<_> = button::Button::new(20, 200, 50, 30, "Apply").into();
    btn_apply.clear_visible_focus();

    // Close button
    let mut btn_close: Listener<_> = button::Button::new(220, 200, 50, 30, "Close").into();
    btn_close.clear_visible_focus();

    window.show();
    window.end();

    let mut window_clone = window.clone();

    // Window call back to avoid program termination when ESC is pressed
    // from FLTK Book - FAQ
    window.set_callback( |w| {
        if fltk::app::event() == fltk::enums::Event::Close {
            w.hide();
        }
    });

    // Handlers for Close button
    // preserve button's original color
    let btn_color = btn_close.color();
    // close window when clicked
    btn_close.on_click(move |b| {
        window_clone.hide();
    });

    // change color on hover
    btn_close.on_hover(|b| {
        b.set_color(enums::Color::Red.lighter());
    });

    // reset color on leave
    btn_close.on_leave(move |b| {
        b.set_color(btn_color);
    });

    // Handlers for Apply button
    // preserve button's original color
    let btn_apply_color = btn_apply.color();
    // Apply changes
    let mut app_clone = Rc::clone(&application);
    btn_apply.set_callback( move |_| {
        todo!();
     });

    // change color on hover
    btn_apply.on_hover(|b| {
        b.set_color(enums::Color::Green.lighter());
    });

    // reset color on leave
    btn_apply.on_leave(move |b| {
        b.set_color(btn_apply_color);
    });

    while window.shown() {
        app::wait();
        // Reduce frame updated to reduce CPU consumption
        std::thread::sleep(std::time::Duration::from_millis(32));
    }

    true
}
