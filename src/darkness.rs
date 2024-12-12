use crate::sun_alt_az_grid_utc;
use crate::moon_alt_az_grid_utc;

const HORIZON: [f64; 4] = [-0.833, -6.0, -12.0, -18.0];

// TODO Consider nautical darkness when astronomical is not possible (extreme latitudes)
// TODO Report never rises and never sets for Sun and Moon
// TODO Treat NaN
// Todo treat timezone
// Todo include interpolation
// horizon 0 - rise/set, 1 - civil, 2 - nautical, 3 - astronomical
pub fn darkness_utc(lat: f64, lon: f64, jd: f64, horizon: usize) -> (f64, f64){
    let num_points = 1440;
    let target_night_start = (jd + 0.5).floor() + 3.0 / 24.0;
    let target_night_end = target_night_start + 1.0;
    let sun = sun_alt_az_grid_utc(lat, lon, target_night_start, target_night_end, num_points);
    let moon = moon_alt_az_grid_utc(lat, lon, target_night_start, target_night_end, num_points);

    let mut darkness: Vec<f64> = Vec::new();

    for i in 0..sun.len() {
        if sun[i].1 <= HORIZON[horizon] && moon[i].1 <= 0.125 {
            darkness.push(sun[i].0);
        }
    }

    let end = darkness.iter().copied().fold(f64::NAN, f64::max);
    let start = darkness.iter().copied().fold(f64::NAN, f64::min);

    (start, end)
}