use std::cell::RefCell;
use std::rc::Rc;
use fltk::prelude::{GroupExt, InputExt, WidgetBase, WidgetExt, WindowExt};
use fltk::{app, button, enums, window};
use fltk::enums::{Align, Event, Key};
use fltk::input::{FloatInput, Input, IntInput};
use fltk_evented::Listener;
use libm::fabs;
use crate::application::application::Application;
use crate::widgets::angle::AngleInput;
use crate::widgets::label::Label;

pub fn handle_observatory(mut application: &mut Rc<RefCell<Application>>) -> bool {
    let mut window = window::Window::default()
        .with_label("Observatory setup")
        .with_size(290, 250)
        .center_screen();
    window.make_modal(true);

    // Name
    Label::new(10, 10, 80, 20, "Name", Align::Left | Align::Inside);
    let mut name = Input::new(10, 30, 270, 25, "");
    name.set_maximum_size(35);
    if let Some(name_str) = &application.borrow_mut().observer.name {
        name.set_value(name_str.as_str());
    }

    // Elevation
    Label::new(10, 60, 80, 20, "Elevation (m)", Align::Left | Align::Inside);
    let mut elevation = IntInput::new(10, 80, 80, 25, "");
    elevation.set_maximum_size(4);
    elevation.set_value(&application.borrow_mut().observer.elevation.to_string());

    // Latitude
    Label::new(150, 60, 80, 20, "Latitude", Align::Left | Align::Inside);
    let mut latitude = AngleInput::new(150, 80, 130, 25, "", -90., 90.);
    latitude.set_value(&format!("{:.6}",&application.borrow_mut().observer.latitude));

    // Timezone
    Label::new(10, 110, 80, 20, "TZ", Align::Left | Align::Inside);
    let mut timezone = FloatInput::new(10, 130, 50, 25, "");
    timezone.set_value(&application.borrow_mut().observer.timezone.to_string());

    // Longitude
    Label::new(150, 110, 80, 20, "Longitude", Align::Left | Align::Inside);
    let mut longitude = AngleInput::new(150, 130, 130, 25, "", -180., 180.);
    longitude.set_value(&format!("{:.6}",&application.borrow_mut().observer.longitude));

    // Apply button
    let mut btn_apply: Listener<_> = button::Button::new(20, 200, 50, 30, "Apply").into();
    btn_apply.clear_visible_focus();

    // Close button
    let mut btn_close: Listener<_> = button::Button::new(220, 200, 50, 30, "Close").into();
    btn_close.clear_visible_focus();

    window.show();
    window.end();

    let mut window_clone = window.clone();
    let name_input_clone = name.clone();
    let latitude_input_clone = latitude.angle_input.clone();
    let latitude_update_clone = latitude.angle_input.clone();
    let longitude_input_clone = longitude.angle_input.clone();
    let longitude_update_clone = longitude.angle_input.clone();
    let timezone_input_clone = timezone.clone();
    let timezone_update_clone = timezone.clone();
    let elevation_input_clone = elevation.clone();
    let elevation_update_clone = elevation.clone();

    // Window call back to avoid program termination when ESC is pressed
    // from FLTK Book - FAQ
    window.set_callback(|w| {
        if fltk::app::event() == fltk::enums::Event::Close {
            w.hide();
        }
    });

    // Listener::from_widget(latitude_input_clone).on(fltk::enums::Event::Unfocus, move |_| {
    //     latitude.validate();
    // });

    latitude_input_clone.clone().handle(move |_, ev| {
        match ev {
            Event::Unfocus => {
                latitude.validate();
                true
            }
            Event::KeyDown => {
                let key = app::event_key();
                if key == Key::Enter {
                    latitude.validate();
                    true
                } else {
                    false
                }
            }
            _ => false
        }
    });


    // Listener::from_widget(longitude_input_clone).on(fltk::enums::Event::Unfocus, move |_| {
    //     longitude.validate();
    // });

    longitude_input_clone.clone().handle(move |_, ev| {
        match ev {
            Event::Unfocus => {
                longitude.validate();
                true
            }
            Event::KeyDown => {
                let key = app::event_key();
                if key == Key::Enter {
                    longitude.validate();
                    true
                } else {
                    false
                }
            }
            _ => false
        }
    });

    // Listener::from_widget(timezone_input_clone).on(fltk::enums::Event::Unfocus, move |_| {
    //     let timezone_value = timezone.value().parse::<f64>().unwrap_or(0.0);
    //     if fabs(timezone_value) > 12.0 { timezone.set_value( "0.0" )};
    // });

    let mut app_clone = application.clone();
    timezone_input_clone.clone().handle(move |_, ev| {
        match ev {
            Event::Unfocus => {
                let timezone_value = timezone.value().parse::<f64>().unwrap_or(0.0);
                if fabs(timezone_value) > 12.0 { timezone.set_value( "0.0" )};
                app_clone.borrow_mut().observer.timezone = timezone_value;
                timezone.set_value(&app_clone.borrow_mut().observer.timezone.to_string());
                true
            }
            Event::KeyDown => {
                let key = app::event_key();
                if key == Key::Enter {
                    let timezone_value = timezone.value().parse::<f64>().unwrap_or(0.0); // Handle potential parse errors. Default to 0.0
                    if fabs(timezone_value) > 12.0 { timezone.set_value( "0.0" )};
                    app_clone.borrow_mut().observer.timezone = timezone_value;
                    timezone.set_value(&app_clone.borrow_mut().observer.timezone.to_string());

                    // Optionally move focus
                    // next_widget.take_focus();
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    });


    // Listener::from_widget(elevation_input_clone).on(fltk::enums::Event::Unfocus, move |_| {
    //     let elevation_value = elevation.value().parse::<f64>().unwrap_or(0.0);
    //     if elevation_value < 0.0 { elevation.set_value("0.0") };
    // });
    let mut app_clone = application.clone();
    elevation_input_clone.clone().handle(move |_, ev| {
        match ev {
            Event::Unfocus => {
                let elevation_value = elevation.value().parse::<f64>().unwrap_or(0.0);
                if elevation_value < 0.0 { elevation.set_value("0.0") };
                app_clone.borrow_mut().observer.elevation = elevation_value as i64;
                elevation.set_value(&app_clone.borrow_mut().observer.elevation.to_string());
                true
            }
            Event::KeyDown => {
                let key = app::event_key();
                if key == Key::Enter {
                    let elevation_value = elevation.value().parse::<f64>().unwrap_or(0.0);
                    if elevation_value < 0.0 { elevation.set_value("0.0") };
                    app_clone.borrow_mut().observer.elevation = elevation_value as i64;
                    elevation.set_value(&app_clone.borrow_mut().observer.elevation.to_string());

                    // Optionally move focus
                    // next_widget.take_focus();
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    });

    // Handlers for Close button
    // preserve button's original color
    let btn_close_color = btn_close.color();
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
        b.set_color(btn_close_color);
    });

    // Handlers for Apply button
    // preserve button's original color
    let btn_apply_color = btn_apply.color();
    // Apply changes
    let mut app_clone = Rc::clone(&application);
    btn_apply.set_callback( move |_| {
        // update observer
        app_clone.borrow_mut().observer.name = Some(name.value().to_string());
        app_clone.borrow_mut().observer.elevation = elevation_update_clone.value().parse().unwrap_or(0); // Handle parsing errors
        app_clone.borrow_mut().observer.latitude = latitude_update_clone.value().parse().unwrap_or(0.0);
        app_clone.borrow_mut().observer.longitude = longitude_update_clone.value().parse().unwrap_or(0.0);
        app_clone.borrow_mut().observer.timezone = timezone_update_clone.value().parse().unwrap_or(0.0); // Handle parsing errors
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
        fltk::app::wait();

        // Reduce frame updated to reduce CPU consumption
        std::thread::sleep(std::time::Duration::from_millis(32));
    }

    true
}
