use std::sync::{Mutex, Arc};
use twitter;
use data::SharedData;
use data::mood::MoodLocation;

pub fn paris_trend() -> String {
    generic_trend("615702")
}

pub fn france_trend() -> String {
    generic_trend("23424819")
}

pub fn english_trend() -> String {
    generic_trend("23424977")
}

pub fn paris_mood(api_data: &Arc<Mutex<SharedData>>) -> String {
    generic_mood("f", api_data)
}

pub fn france_mood(api_data: &Arc<Mutex<SharedData>>) -> String {
    generic_mood("f", api_data)
}

pub fn english_mood(api_data: &Arc<Mutex<SharedData>>) -> String {
    generic_mood("e", api_data)
}

pub fn generic_trend(location: &str) -> String {
    twitter::get_top_trend(location)
}

pub fn generic_mood(location: &str, api_data: &Arc<Mutex<SharedData>>) -> String {
    match location {
        "f" => {
            let data = api_data.lock().unwrap();
            data.get_geo_mood(&MoodLocation::French).get_current_mood()
        },
        "e" => {
            let data = api_data.lock().unwrap();
            data.get_geo_mood(&MoodLocation::English).get_current_mood()
        },
        _ => {
            let data = api_data.lock().unwrap();
            data.get_geo_mood(&MoodLocation::French).get_current_mood()
        }
    }
}