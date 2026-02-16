use std::fs;
use crate::model::Driver;

/// Load drivers from drivers.json
pub fn load_drivers() -> Vec<Driver> {
    fs::read_to_string("drivers.json")
        .map(|data| serde_json::from_str(&data).unwrap_or_else(|_| vec![]))
        .unwrap_or_else(|_| vec![])
}

/// Save drivers to drivers.json
pub fn save_drivers(drivers: &[Driver]) {
    let json = serde_json::to_string_pretty(drivers).unwrap();
    fs::write("drivers.json", json).unwrap();
}
