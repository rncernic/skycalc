// src/menu/functions/darkness.rs

use crate::application::application::Application;
use crate::utils::definers::TOOLTIP_DATE_INPUT;
use crate::widgets::{date::DateInput, label::Label};
use fltk::enums::{Align, Event, FrameType, Key};
use fltk::frame::Frame;
use fltk::input::FloatInput;
use fltk::prelude::{GroupExt, InputExt, WidgetBase, WidgetExt, WindowExt};
use fltk::{app, button, enums, window};
use fltk_evented::Listener;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use crate::application::darkness::Darkness;
use crate::application::moon::Moon;
use crate::application::reports::darkness_report;
use crate::application::sun::RiseSetType::{Next};
use crate::application::sun::Sun;
use crate::application::sun::TwilightType::{AstronomicalTwilight, CivilTwilight, NauticalTwilight, RiseSet};
use crate::application::time::Time;
use crate::menu;

fn calculate_sun(application: &Application) -> (String, String, String, String, String, String, String, String) {
    let sun = Sun::new(&application.observer, &application.time, &application.environment);

    // Rise/Set
    let sunrise = sun.get_sunrise_local_str(Next, RiseSet, Some("short"));
    let sunset = sun.get_sunset_local_str(Next, RiseSet, Some("short"));

    // Civil twilight
    let civ_tw_start = sun.get_sunset_local_str(Next, CivilTwilight, Some("short"));
    let civ_tw_end = sun.get_sunrise_local_str(Next, CivilTwilight, Some("short"));

    // Nautical twilight
    let naut_tw_start = sun.get_sunset_local_str(Next, NauticalTwilight, Some("short"));
    let naut_tw_end = sun.get_sunrise_local_str(Next, NauticalTwilight, Some("short"));

    // Astronomical twilight
    let astro_tw_start = sun.get_sunset_local_str(Next, AstronomicalTwilight, Some("short"));
    let astro_tw_end = sun.get_sunrise_local_str(Next, AstronomicalTwilight, Some("short"));

    (sunrise, sunset, civ_tw_start, civ_tw_end, naut_tw_start, naut_tw_end,
     astro_tw_start, astro_tw_end)
}

fn calculate_moon(application: &Application) -> (String, String) {
    let moon = Moon::new(&application.observer, &application.time, &application.environment);
    let moonrise = moon.get_moonrise_local_str(Next, Some("short"));
    let moonset = moon.get_moonset_local_str(Next, Some("short"));

    (moonrise, moonset)
}

fn calculate_darkness(application: &Application) -> (String, String, String, String) {
    let darkness = Darkness::new(&application.observer, &application.time, &application.environment);
    let astronomical_dso_start = darkness.get_darkness_local_astronomical_start_str(Some("short"));
    let astronomical_dso_end = darkness.get_darkness_local_astronomical_end_str(Some("short"));
    let nautical_dso_start = darkness.get_darkness_local_nautical_start_str(Some("short"));
    let nautical_dso_end = darkness.get_darkness_local_nautical_end_str(Some("short"));

    (astronomical_dso_start, astronomical_dso_end, nautical_dso_start, nautical_dso_end)
}

pub fn handle_darkness(mut application: &mut Rc<RefCell<Application>>) -> bool {
    let mut window = window::Window::default()
        .with_label("Darkness Calculator")
        .with_size(450, 480)
        .center_screen();
    window.make_modal(true);

    // Observatory
    Label::new(10, 10, 60, 20, "Observatory:", Align::Left | Align::Inside);
    let mut _observatory = Label::new(100, 10, 200, 20, "", Align::Left | Align::Inside);
    if let Some(name_str) = &application.borrow_mut().observer.name {
        _observatory.set_label(name_str.as_str());
    }

    // Latitude
    Label::new(10, 35, 60, 20, "Latitude:", Align::Left | Align::Inside);
    let mut _latitude = Label::new(75, 35, 130, 20, "", Align::Left | Align::Inside);
    _latitude.set_label(&format!("{:.6}",&application.borrow_mut().observer.latitude));

    // Longitude
    Label::new(160, 35, 60, 20, "Longitude:", Align::Left | Align::Inside);
    let mut _longitude = Label::new(235, 35, 130, 20, "", Align::Left | Align::Inside);
    _longitude.set_label(&format!("{:.6}",&application.borrow_mut().observer.longitude));

    // Elevation
    Label::new(330, 35, 60, 20, "Elevation:", Align::Left | Align::Inside);
    let mut _elevation = Label::new(400, 35, 130, 20, "", Align::Left | Align::Inside);
    _elevation.set_label(&application.borrow_mut().observer.elevation.to_string());

    // Date
    Label::new(10, 65, 80, 20, "Date:", Align::Left | Align::Inside);
    let mut date = DateInput::new(60, 65, 100, 20, "");
    date.validate(); // populate date input field with now() when window is opened
    date.set_tooltip(TOOLTIP_DATE_INPUT);

    // Timezone
    Label::new(180, 65, 80, 20, "Timezone:", Align::Left | Align::Inside);
    let mut timezone = FloatInput::new(260, 65, 50, 20, "");
    timezone.set_value(&application.borrow_mut().observer.timezone.to_string());

    // Observatory button
    let mut btn_observatory: Listener<_> = button::Button::new(350, 65, 80, 20, "Obs. Setup").into();
    btn_observatory.clear_visible_focus();

    // Divider
    Frame::new(10, 100, 430, 1, "").set_frame(FrameType::BorderBox);

    // Sunrise / sunset
    Label::new(10, 110, 80, 20, "Sunset", Align::Left | Align::Inside);
    let mut sunset_label = Label::new(120, 110, 80, 20, "", Align::Left | Align::Inside);
    Label::new(230, 110, 80, 20, "Sunrise", Align::Left | Align::Inside);
    let mut sunrise_label = Label::new(340, 110, 80, 20, "", Align::Left | Align::Inside);

    // Civil twilight
    Label::new(10, 130, 80, 20, "Civ Tw end", Align::Left | Align::Inside);
    let mut civ_tw_start_label = Label::new(120, 130, 80, 20, "", Align::Left | Align::Inside);
    Label::new(230, 130, 80, 20, "Civ Tw start", Align::Left | Align::Inside);
    let mut civ_tw_end_label = Label::new(340, 130, 80, 20, "", Align::Left | Align::Inside);

    // Nautical twilight
    Label::new(10, 150, 80, 20, "Naut Tw end", Align::Left | Align::Inside);
    let mut naut_tw_start_label = Label::new(120, 150, 80, 20, "", Align::Left | Align::Inside);
    Label::new(230, 150, 80, 20, "Naut Tw start", Align::Left | Align::Inside);
    let mut naut_tw_end_label = Label::new(340, 150, 80, 20, "", Align::Left | Align::Inside);

    // Astronomical twilight
    Label::new(10, 170, 80, 20, "Astro Tw end", Align::Left | Align::Inside);
    let mut astro_tw_start_label = Label::new(120, 170, 80, 20, "", Align::Left | Align::Inside);
    Label::new(230, 170, 80, 20, "Astro Tw start", Align::Left | Align::Inside);
    let mut astro_tw_end_label = Label::new(340, 170, 80, 20, "", Align::Left | Align::Inside);

    // Divider
    Frame::new(10, 200, 430, 1, "").set_frame(FrameType::BorderBox);

    // Moon rise / Moon set
    Label::new(10, 210, 80, 20, "Moon rise", Align::Left | Align::Inside);
    let mut moonrise_label = Label::new(120, 210, 80, 20, "", Align::Left | Align::Inside);
    Label::new(230, 210, 80, 20, "Moon set", Align::Left | Align::Inside);
    let mut moonset_label = Label::new(340, 210, 80, 20, "", Align::Left | Align::Inside);

    // Divider
    Frame::new(10, 240, 430, 1, "").set_frame(FrameType::BorderBox);

    // DSO Astro - Deep Sky Object darkness for astronomical rise and set
    Label::new(10, 250, 80, 20, "DSO Astro start", Align::Left | Align::Inside);
    let mut astronomical_dso_start_label = Label::new(120, 250, 80, 20, "", Align::Left | Align::Inside);
    Label::new(230, 250, 80, 20, "DSO Astro end", Align::Left | Align::Inside);
    let mut astronomical_dso_end_label = Label::new(340, 250, 80, 20, "", Align::Left | Align::Inside);
    // DSO Naut - Deep Sky Object darkness for nautical rise and set
    Label::new(10, 270, 80, 20, "DSO Naut start", Align::Left | Align::Inside);
    let mut nautical_dso_start_label = Label::new(120, 270, 80, 20, "", Align::Left | Align::Inside);
    Label::new(230, 270, 80, 20, "DSO Naut end", Align::Left | Align::Inside);
    let mut nautical_dso_end_label = Label::new(340, 270, 80, 20, "dd-mm hh:mm", Align::Left | Align::Inside);

    // Divider
    Frame::new(10, 300, 430, 1, "").set_frame(FrameType::BorderBox);

    // NB Astro - Narrow band darkness for astronomical rise and set
    Label::new(10, 310, 80, 20, "NB Astro start", Align::Left | Align::Inside);
    let mut astronomical_nb_start_label = Label::new(120, 310, 80, 20, "", Align::Left | Align::Inside);
    Label::new(230, 310, 80, 20, "NB Astro end", Align::Left | Align::Inside);
    let mut astronomical_nb_end_label = Label::new(340, 310, 80, 20, "", Align::Left | Align::Inside);
    // NB Naut - Narrow band darkness for nautical rise and set
    Label::new(10, 330, 80, 20, "NB Naut start", Align::Left | Align::Inside);
    let mut nautical_nb_start_label = Label::new(120, 330, 80, 20, "", Align::Left | Align::Inside);
    Label::new(230, 330, 80, 20, "NB Naut end", Align::Left | Align::Inside);
    let mut nautical_nb_end_label = Label::new(340, 330, 80, 20, "", Align::Left | Align::Inside);

    // Divider
    Frame::new(10, 360, 430, 1, "").set_frame(FrameType::BorderBox);

    // Export button
    let mut btn_export: Listener<_> = button::Button::new(20, 430, 50, 30, "Export").into();
    btn_export.clear_visible_focus();

    // TODO Add buttons previous day - today - next day

    // Close button
    let mut btn_close: Listener<_> = button::Button::new(380, 430, 50, 30, "Close").into();
    btn_close.clear_visible_focus();

    window.end();
    window.show();

    let mut window_clone = window.clone();
    let date_input_clone = date.date_input.clone();
    let timezone_input_clone = timezone.clone();
    let mut timezone_observatory_clone = timezone.clone();
    let mut application_clone = Rc::clone(&application);
    let mut application_clone_calculations = Rc::clone(&application);

    // Window call back to avoid program termination when ESC is pressed
    // from FLTK Book - FAQ
    window.set_callback( |w| {
        if fltk::app::event() == enums::Event::Close {
            w.hide();
        }
    });

    // Listener::from_widget(date_input_clone).on(enums::Event::Unfocus, move |_| {
    //     date.validate();
    //     application_clone.borrow_mut().time.day = date.get_day();
    //     application_clone.borrow_mut().time.month = date.get_month();
    //     application_clone.borrow_mut().time.year = date.get_year();
    // });

    // Listener::from_widget(date_input_clone).on_unfocus(move |_| {
    //     date.validate();
    //     application_clone.borrow_mut().time.day = date.get_day();
    //     application_clone.borrow_mut().time.month = date.get_month();
    //     application_clone.borrow_mut().time.year = date.get_year();
    // });

    date_input_clone.clone().handle(move |_, ev| {
        match ev {
            Event::Unfocus => {
                date.validate();
                application_clone.borrow_mut().time.day = date.get_day();
                application_clone.borrow_mut().time.month = date.get_month();
                application_clone.borrow_mut().time.year = date.get_year();

                // TODO Add this code to the "increment a day" button
                // let new_date = Time::from_jd(application_clone.borrow().time.to_jd() + 1.0);
                // date.set_value(&new_date.to_string(Some("yyyymmdd")));

                true
            }
            Event::KeyDown => {
                let key = app::event_key();
                if key == Key::Enter {
                    date.validate();
                    let mut app = application_clone.borrow_mut();
                    app.time.day = date.get_day();
                    app.time.month = date.get_month();
                    app.time.year = date.get_year();

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

    let mut app_clone = application.clone();
    // Listener::from_widget(timezone_input_clone).on(enums::Event::Unfocus, move |_| {
    //     let timezone_value = timezone.value().parse::<f64>().unwrap_or(0.0); // Handle potential parse errors. Default to 0.0
    //     app_clone.borrow_mut().observer.timezone = timezone_value;
    //     timezone.set_value(&app_clone.borrow_mut().observer.timezone.to_string());
    // });
    timezone_input_clone.clone().handle(move |_, ev| {
        match ev {
            Event::Unfocus => {
                let timezone_value = timezone.value().parse::<f64>().unwrap_or(0.0); // Handle potential parse errors. Default to 0.0
                app_clone.borrow_mut().observer.timezone = timezone_value;
                timezone.set_value(&app_clone.borrow_mut().observer.timezone.to_string());
                true
            }
            Event::KeyDown => {
                let key = app::event_key();
                if key == Key::Enter {
                    let timezone_value = timezone.value().parse::<f64>().unwrap_or(0.0); // Handle potential parse errors. Default to 0.0
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
    btn_close.on_leave( move |b| {
        b.set_color(btn_close_color);
    });

    // Handlers for Export button
    // preserve button's original color
    let btn_export_color = btn_export.color();
    // Export to file when clicked
    let mut application_clone_darkness_report = application.clone();
    btn_export.on_click(move |_| {
        darkness_report(&application_clone_darkness_report.borrow().observer,
                        &application_clone_darkness_report.borrow().time,
                        &application_clone_darkness_report.borrow().environment);
    });

    // change color on hover
    btn_export.on_hover(|b| {
        b.set_color(enums::Color::Green.lighter());
    });

    // reset color on leave
    btn_export.on_leave(move |b| {
        b.set_color(btn_export_color);
    });

    let mut application_observatory = Rc::clone(&application);
    // Handle for Observatory button
    // preserve button's original color
    let btn_observatory_color = btn_observatory.color();
    // Show Observatory dialog when button clicked
    btn_observatory.on_click(move |b| {
        menu::functions::observatory::handle_observatory(&mut application_observatory);
        if let Some(name_str) = &application_observatory.borrow_mut().observer.name {
            _observatory.set_label(name_str.as_str());
        }
        _elevation.set_label(&application_observatory.borrow_mut().observer.elevation.to_string());
        _latitude.set_label(&application_observatory.borrow_mut().observer.latitude.to_string());
        _longitude.set_label(&application_observatory.borrow_mut().observer.longitude.to_string());
        timezone_observatory_clone.set_value(&application_observatory.borrow_mut().observer.timezone.to_string());
    });

    // change color on hover
    btn_observatory.on_hover(|b| {
        b.set_color(enums::Color::Blue);
    });

    // reset color on leave
    btn_observatory.on_leave(move |b| {
        b.set_color(btn_observatory_color);
    });


    while window.shown() {
        // Update calculations
        let (sunrise, sunset, civ_tw_start, civ_tw_end,
            naut_tw_start, naut_tw_end, astro_tw_start, astro_tw_end) =
            calculate_sun(&application_clone_calculations.borrow_mut());

        let (moonrise, moonset) =
            calculate_moon(&application_clone_calculations.borrow_mut());

        let (astronomical_dso_start, astronomical_dso_end,
            nautical_dso_start, nautical_dso_end) =
            calculate_darkness(&application_clone_calculations.borrow_mut());

        // Update Sun labels
        sunrise_label.set_label(&sunrise);
        sunset_label.set_label(&sunset);
        civ_tw_start_label.set_label(&civ_tw_start);
        civ_tw_end_label.set_label(&civ_tw_end);
        naut_tw_start_label.set_label(&naut_tw_start);
        naut_tw_end_label.set_label(&naut_tw_end);
        astro_tw_start_label.set_label(&astro_tw_start);
        astro_tw_end_label.set_label(&astro_tw_end);

        // Update Moon labels
        moonrise_label.set_label(&moonrise);
        moonset_label.set_label(&moonset);

        // Update Darkness labels
        astronomical_dso_start_label.set_label(&astronomical_dso_start);
        astronomical_dso_end_label.set_label(&astronomical_dso_end);
        nautical_dso_start_label.set_label(&nautical_dso_start);
        nautical_dso_end_label.set_label(&nautical_dso_end);

        astronomical_nb_start_label.set_label(&astro_tw_start);
        astronomical_nb_end_label.set_label(&astro_tw_end);
        nautical_nb_start_label.set_label(&naut_tw_start);
        nautical_nb_end_label.set_label(&naut_tw_end);

        //Redraw window to update labels
        window.redraw();

        fltk::app::wait();

        // Reduce frame updated to reduce CPU consumption
        std::thread::sleep(std::time::Duration::from_millis(32));
    }

    true
}

