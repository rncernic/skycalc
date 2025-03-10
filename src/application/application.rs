use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::rc::Rc;
use crate::application::constraint::{default_frac_observable_time,
                                     default_max_altitude,
                                     default_max_size,
                                     default_max_targets,
                                     default_min_altitude,
                                     default_min_size,
                                     default_moon_separation,
                                     default_use_darkness,
                                     Constraints};
use crate::application::environment::{default_humidity,
                         default_pressure,
                         default_temperature,
                         Environment};
use crate::application::observer::{default_elevation,
                      default_lat,
                      default_lon,
                      default_name,
                      default_timezone,
                      Observer};
use crate::application::time::{Time};

pub const DEFAULT_TARGET_LIST: &str = "OpenNGC";
pub const DEFAULT_TYPE_FILTER: &str = "";
pub const DEFAULT_OUTPUT_DIR: &str = "output";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Application {
    pub observer: Observer,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub time: Time,
    pub environment: Environment,
    pub constraints: Constraints,
}

// Function to return default values for Config
fn default_config() -> (Observer, Time, Environment, Constraints) {
    (
        Observer {
            name: default_name(),
            latitude: default_lat(),
            longitude: default_lon(),
            elevation: default_elevation(),
            timezone: default_timezone()
        },
        Time::default(),
        Environment {
            temperature: default_temperature(),
            humidity: default_humidity(),
            pressure: default_pressure(),
        },
        Constraints {
            min_altitude: default_min_altitude(),
            max_altitude: default_max_altitude(),
            min_size: default_min_size(),
            max_size: default_max_size(),
            moon_separation: default_moon_separation(),
            frac_observable_time: default_frac_observable_time(),
            max_targets: default_max_targets(),
            use_darkness: default_use_darkness()
        }
    )
}

pub fn load_from_yaml(file_path: &str, application: &mut Rc<RefCell<Application>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();

    // Try to open the file
    match File::open(file_path) {
        Ok(mut file) => {
            if let Err(e) = file.read_to_string(&mut contents) {
                return Err(Box::new(e));
            }

            match serde_yaml::from_str(&contents) {
                Ok(config) => {
                    *application.borrow_mut() = config;
                    Ok(())
                }
                Err(e) => {
                    Err(Box::new(e))
                }
            }
        }
        Err(_) => {
            // File not found or unreadable, use default values
            println!("YAML configuration file not found. Using default values. {:?}", file_path);
            let (observer, time, environment, constraints) = default_config();
            *application.borrow_mut() = Application {
                observer,
                time,
                environment,
                constraints,
            };
            Ok(())
        }
    }
}

pub fn save_to_yaml(file_path: PathBuf, application: &mut Rc<RefCell<Application>>) -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?; // TODO Treat errors when writing

    serde_yaml::to_writer(f, &*application.borrow())?; // Borrow immutably and dereference

    Ok(())
}
