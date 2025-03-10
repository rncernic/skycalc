use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use fltk::dialog::{FileDialog, FileDialogType};
use crate::application::application::{load_from_yaml, save_to_yaml, Application};

pub fn handle_save_configuration(application: &mut Rc<RefCell<Application>>) {
    let mut dialog = FileDialog::new(FileDialogType::BrowseSaveFile);
    dialog.set_filter("Configuration Files\t*.{yaml}");
    dialog.show();

    if let Some(filename) = dialog.filename().to_str() {
        let mut path = PathBuf::from(filename);

        if let Some(extension) = path.extension() {
            if extension.to_str() != Some("yaml") {
                path.set_extension("yaml");
            }
        } else {
            path.set_extension("yaml");
        }

        save_to_yaml(path, application).expect("Failed to save configuration file");
    }
}

pub fn handle_load_configuration(application: &mut Rc<RefCell<Application>>) {
    let mut dialog = FileDialog::new(FileDialogType::BrowseFile);
    dialog.set_filter("Configuration Files\t*.{yaml}");
    dialog.show();

    if let Some(filename) = dialog.filename().to_str() {
        load_from_yaml(filename, application).expect("Failed to load configuration file");
    }
}