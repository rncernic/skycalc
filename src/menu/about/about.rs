// // src/menu/about/about.rs

use crate::utils::definers::{APP_COPYRIGHT, APP_TITLE, APP_VERSION};
use fltk::{app, enums::Event, frame, frame::Frame, group, prelude::*, window::Window};
use std::cell::RefCell;
use std::rc::Rc;
use fltk::enums::Align;
use crate::application::application::Application;
use crate::widgets::label::Label;

pub fn handle_about(menu: &mut fltk::menu::MenuBar, parent: &Window) {
    // Shared state to track if the about window is open
    static mut IS_ABOUT_OPEN: Option<Rc<RefCell<bool>>> = None;

    unsafe {
        if IS_ABOUT_OPEN.is_none() {
            IS_ABOUT_OPEN = Some(Rc::new(RefCell::new(false)));
        }
    }

    // Access the shared state
    let is_about_open = unsafe { IS_ABOUT_OPEN.as_ref().unwrap().clone() };

    let mut is_open = is_about_open.borrow_mut();
    if *is_open {
        println!("About window is already open!");
        return;
    }
    *is_open = true;

    // Deactivate the menu bar
    menu.deactivate();

    // Create the About window as a child of the main window
    let mut about_win = Window::new(300, 300, 400, 200, "About");
    about_win.make_modal(true); // Ensure it's modal (blocks interaction with the main window)
    about_win.make_resizable(false); // Do not allow resizing of the window

    let mut frame = Frame::default()
        .with_size(300, 100)
        .with_pos(50, 30)
        .with_label(&*format!("{}\n\n {}\n\n {}", APP_TITLE, APP_VERSION, APP_COPYRIGHT).to_string());
    frame.set_label_size(14);
    frame.set_align(fltk::enums::Align::Center | fltk::enums::Align::Inside);

    // Set up the callback for closing the window (using the close button)
    about_win.set_callback({
        let is_about_open = is_about_open.clone();
        let mut menu = menu.clone();
        move |win| {
            // Reset the flag and reactivate the menu bar when the window is closed
            *is_about_open.borrow_mut() = false;
            menu.activate();
            win.hide(); // Hide the window when it's closed
        }
    });

    // Handle mouse clicks outside the About window
    about_win.handle({
        let is_about_open = is_about_open.clone();
        let mut menu = menu.clone();
        move |win, ev| {
            if ev == Event::Push {
                let mouse_x = app::event_x();
                let mouse_y = app::event_y();

                let x = win.x();
                let y = win.y();
                let w = win.width();
                let h = win.height();

                if mouse_x < x || mouse_x > x + w || mouse_y < y || mouse_y > y + h {
                    win.hide(); // Hide the window if clicked outside
                    *is_about_open.borrow_mut() = false;
                    menu.activate();
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
    });

    about_win.show();
}
