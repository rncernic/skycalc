#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod application;
mod menu;
mod utils;
mod widgets;

use crate::application::application::{load_from_yaml, save_to_yaml, Application};
use fltk::{app, enums::Shortcut, menu::MenuBar, menu::MenuFlag, prelude::*, window::Window};
use fltk_theme::{color_themes, ColorTheme, ThemeType, WidgetTheme};
use menu::about;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;
use utils::definers::{APP_TITLE, MENU_HEIGHT};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    // start with the initial dark theme
    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    theme.apply();

    let application = Rc::new(RefCell::new(Application::default()));

    let mut wind = Window::default()
        .with_size(800, 600)
        .with_label(APP_TITLE)
        .center_screen();

    let mut menu = MenuBar::new(0, 0, 800, MENU_HEIGHT, "");

    // Window call back to avoid program termination when ESC is pressed
    // from FLTK Book - FAQ
    wind.set_callback(|_| {
        if fltk::app::event() == fltk::enums::Event::Close {
            menu::file::exit::handle_exit();
        }
    });

    // File -> Config -> load
    let mut application_load_conf = Rc::clone(&application);
    menu.add(
        "&File/&Configuration",
        Shortcut::None,
        MenuFlag::Submenu,
        |_| {}
    );

    menu.add(
        "File/Configuration/&Load\t",
        Shortcut::Ctrl | 'l',
        MenuFlag::Normal,
        move |_| {
            menu::file::config::handle_load_configuration(&mut application_load_conf);
        },
    );

    // File -> Config -> Save
    let mut application_save_conf = Rc::clone(&application);
    menu.add(
        "File/Configuration/&Save\t",
        Shortcut::Ctrl | 's',
        MenuFlag::Normal,
        move |_| {
            menu::file::config::handle_save_configuration(&mut application_save_conf);
        },
    );

    // File -> Preferences
    menu.add(
        "&File/&Preferences\t",
        Shortcut::Ctrl | 'p',
        MenuFlag::MenuDivider,
        |_| {
            // menu::file::exit::handle_preferences();
        },
    );

    // File -> Exit
    menu.add(
        "&File/E&xit\t",
        Shortcut::Ctrl | 'x',
        MenuFlag::Normal,
        |_| {
            menu::file::exit::handle_exit();
        },
    );

    // Functions -> Observatory
    let mut application_observatory = Rc::clone(&application);
    menu.add(
        "F&unctions/&Observatory\t",
        Shortcut::Ctrl | 'o',
        MenuFlag::Normal,
        move |_| {
            menu::functions::observatory::handle_observatory(&mut application_observatory);
        },
    );

    // Functions -> Constraints
    let mut application_constraints = Rc::clone(&application);
    menu.add(
        "F&unctions/&Constraints\t",
        Shortcut::Ctrl | 'c',
        MenuFlag::Normal,
        move |_| {
            // menu::functions::constraint::handle_constraint(&mut application_constraints);
        },
    );

    // Functions -> Darkness
    let mut application_darkness = Rc::clone(&application);
    menu.add(
        "F&unctions/&Darkness\t",
        Shortcut::Ctrl | 'd',
        MenuFlag::Normal,
        move |_| {
            menu::functions::darkness::handle_darkness(&mut application_darkness);
        },
    );

    // Theme Options
    // menu.add("&View/&Themes/Color Themes/Dark", Shortcut::None, MenuFlag::Normal, |_| {
    menu.add("&View/&Themes/Dark", Shortcut::None, MenuFlag::Normal, |_| {
        let theme = ColorTheme::new(color_themes::DARK_THEME);
        theme.apply();
    });

    // menu.add("&View/&Themes/Color Themes/Black", Shortcut::None, MenuFlag::Normal, |_| {
    menu.add("&View/&Themes/Black", Shortcut::None, MenuFlag::Normal, |_| {
    let theme = ColorTheme::new(color_themes::BLACK_THEME);
        theme.apply();
    });

    // menu.add("&View/&Themes/Color Themes/Gray", Shortcut::None, MenuFlag::Normal, |_| {
    menu.add("&View/&Themes/Gray", Shortcut::None, MenuFlag::Normal, |_| {
            let theme = ColorTheme::new(color_themes::GRAY_THEME);
        theme.apply();
    });

    // menu.add("&View/&Themes/Widget Themes/Dark", Shortcut::None, MenuFlag::Normal, |_| {
    //     let widget_theme = WidgetTheme::new(ThemeType::Dark);
    //     widget_theme.apply();
    // });
    //
    // menu.add("&View/&Themes/Widget Themes/Classic", Shortcut::None, MenuFlag::Normal, |_| {
    //     let widget_theme = WidgetTheme::new(ThemeType::Classic);
    //     widget_theme.apply();
    // });


    // About
    let mut menu_about = menu.clone();
    let wind_about = wind.clone();
    menu.add(
        "&Help/&About\t",
        Shortcut::Ctrl | 'a',
        MenuFlag::Normal, {
        move |_| {
            about::about::handle_about(&mut menu_about, &wind_about);
        }
    });

    wind.end();
    wind.make_resizable(true);
    wind.show();

    while app.wait(){
        // Reduce frame updated to reduce CPU consumption
        std::thread::sleep(std::time::Duration::from_millis(32));
    }

    // app.run().unwrap();

    Ok(())
}
