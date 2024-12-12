// https://github.com/boom-astro/flare/tree/main

#![allow(dead_code, unused_variables)]

mod datetime;
mod utils;
mod sun;
mod observer;
mod environment;
mod moon;
mod transformations;
mod location;
mod earth;
mod darkness;
mod config;
mod constraints;
mod angle;
mod reports;

//use std::io::prelude::*;
use clap::{Arg, command, value_parser};
use std::path::Path;
use std::string::ToString;
use std::time::{SystemTime, UNIX_EPOCH};
use simple_logger::SimpleLogger as sl;
use location::Location;
use datetime::{ymd_hms_from_jd, DateTime};
use environment::Environment;
use observer::Observer;
use sun::sun_alt_az_grid_utc;
use moon::{moon_alt_az_grid_utc, MoonRS};
use config::{load_yaml};
use MoonRS::NeverRise;
use constraints::Constraint;
use crate::reports::darkness_report;

// TODO Configuration in YAML file
fn main() {
    // Init log
    sl::new().init().unwrap();

    // Start counting time to evaluate performance
    let p_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    log::info!("Start processing @ {:?}", p_start);

    // Initialize observe object
    let target_date = DateTime::new();
    let location = Location::new();
    let environment = Environment::new();
    let constraint = Constraint::new();
    let mut obs = Observer::new("My observatory".to_string(), "output".to_string(),
                                target_date, location, environment, constraint);

    // Read YAML file if exists, otherwise use defaults
    // let yaml_file = "config_tromso.yaml";
    let yaml_file = "config.yaml";
    if Path::new(yaml_file).exists() {
        load_yaml(&mut obs, yaml_file);
    } else {
        // Todo use Now as default for date
        // Todo get values from command line
        &obs.target_date.julian(2_460_580.0);
        &obs.location.location(-23.1,-46.5, 780, -3.0);
        &obs.environment.environment(1010, 25, 40);
        //&obs.constraint.constraint();
    }

    log::debug!("{:?}", obs);

    log::info!("Observatory: {:?}", obs.observatory_name);
    log::info!("Output dir: {:?}", obs.output_dir);
    log::info!("Target date: {0:?}-{1:?}-{2:?} JD: {3:?} TZ:{4:?}", obs.target_date.date.year,
             obs.target_date.date.month, obs.target_date.date.day, obs.target_date.jd,
        obs.location.timezone);
    log::info!("Latitude: {:?} Longitude: {:?} Elevation: {:?}", obs.location.latitude, obs.location.longitude,
        obs.location.elevation);
    log::info!("Dec: {:?} RA: {:?}", obs.get_sun_position().1, obs.get_sun_position().0);
    // log::info!("Sunrise: {:?} {:?}", obs.get_sunrise_sunset_local().0, ymd_hms_from_jd(obs.get_sunrise_sunset_local_grid().0));
    // log::info!("Sunset : {:?} {:?}", obs.get_sunrise_sunset_local().2, ymd_hms_from_jd(obs.get_sunrise_sunset_local_grid().1));

    match obs.get_sunrise_local_grid(-0.8333) {
        Ok(rise) => log::debug!("Sunrise: {:?}", ymd_hms_from_jd(rise)),
        Err(e) => log::debug!("Sunrise: {:?}", e),
    }

    match obs.get_sunset_local_grid(-0.8333) {
        Ok(set) => log::debug!("Sunset: {:?}", ymd_hms_from_jd(set)),
        Err(e) => log::debug!("Sunset: {:?}", e),
    }

    // log::debug!("{:?}", obs.get_moon_position_low_precision());
    // log::debug!("{:?}", obs.get_moon_position_high_precision());
    // log::debug!("{:?}", obs.get_moon_distance());

    match obs.get_moonrise_local_grid() {
        Ok(rise) => log::debug!("Moon rise: {:?}", ymd_hms_from_jd(rise)),
        Err(e) => log::debug!("Moon rise: {:?}", e),
    }

    match obs.get_moonset_local_grid() {
        Ok(set) => log::debug!("Moon set: {:?}", ymd_hms_from_jd(set)),
        Err(e) => log::debug!("Moon set: {:?}", e),
    }

    let (start, end) = obs.get_darkness_astronomical_utc();
    log::info!("Astro Start: {:?}", ymd_hms_from_jd(start - 3.0 / 24.0));
    log::info!(" End:{:?}", ymd_hms_from_jd(end - 3.0 / 24.0));
    log::info!(" Duration: {:?}", (end - start) * 24.0);

    let (start, end) = obs.get_darkness_nautical_utc();
    log::info!("Naut  Start: {:?}", ymd_hms_from_jd(start - 3.0 / 24.0));
    log::info!(" End:{:?}", ymd_hms_from_jd(end - 3.0 / 24.0));
    log::info!(" Duration: {:?}", (end - start) * 24.0);

    let (start, end) = obs.get_darkness_civil_utc();
    log::info!("Civil Start: {:?}", ymd_hms_from_jd(start - 3.0 / 24.0));
    log::info!(" End:{:?}", ymd_hms_from_jd(end - 3.0 / 24.0));
    log::info!(" Duration: {:?}", (end - start) * 24.0);

    let (start, end) = obs.get_darkness_rise_set_utc();
    log::info!("R/S   Start: {:?}", ymd_hms_from_jd(start - 3.0 / 24.0));
    log::info!(" End:{:?}", ymd_hms_from_jd(end - 3.0 / 24.0));
    log::info!(" Duration: {:?}", (end - start) * 24.0);

    darkness_report(obs);

    let p_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    log::info!("Execution time: {:?}", p_end - p_start);

    let file = Arg::new("file")
        .short('f')
        .long("file")
        .help("short help")
        .long_help("long help, more explanation")
        .default_value("skycalc.txt");

    let lat = Arg::new("lat")
        .long("lat")
        .help("latitude")
        .allow_hyphen_values(true)
        .long_help("Observatory latitude")
        .value_parser(value_parser!(f64))
        .default_value("0.0");

    let matches = command!() // requires `cargo` feature
        .arg(file)
        .arg(lat)
        .get_matches();

    // to get one (the first) match
    if let Some(file) = matches.get_one::<String>("file") {
        log::debug!("Value for config: {}", file);
    }

    // to get many (all) matches
    if let Some(values) = matches.get_many::<String>("file") {
        for v in values {
            log::debug!("{}", v);
        }
    }

    let lat: f64 = *matches.get_one::<f64>("lat").expect("Number is required");
    log::debug!("Latitude: {}", lat);

    // let latitudes = vec![
    //     "40.7128",                // Decimal degrees
    //     "40°42' N",               // DMS without seconds
    //     "40°42'46\" N",           // DMS with degrees, minutes, and seconds
    //     "90°00' S",               // DMS without seconds (boundary case)
    //     "bullshit lat",           // Invalid input
    //     "100°00'00\" N",          // Out-of-bounds latitude
    // ];
    //
    // let longitudes = vec![
    //     "-74.0060",               // Decimal degrees
    //     "74°00' W",               // DMS without seconds
    //     "74°00'21\" W",           // DMS with degrees, minutes, and seconds
    //     "180°00' E",              // DMS at boundary
    //     "bullshit lon",           // Invalid input
    //     "-200°00'00\" W",         // Out-of-bounds longitude
    // ];
    //
    // // Parsing latitudes (valid range: -90 to 90)
    // for lat in latitudes {
    //     match Degrees::from_str(lat, -90.0, 90.0) {
    //         deg => println!("Parsed latitude: {:?}", deg),
    //     }
    // }
    //
    // println!("---");
    //
    // // Parsing longitudes (valid range: -180 to 180)
    // for lon in longitudes {
    //     match Degrees::from_str(lon, -180.0, 180.0) {
    //         deg => println!("Parsed longitude: {:?}", deg),
    //     }
    // }
}