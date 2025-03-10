// src/menu/file/exit.rs
use fltk::app;

pub fn handle_exit() {
    println!("Quiting application");
    app::quit();
}
