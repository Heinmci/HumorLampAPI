use std::sync::{Mutex, Arc};
use twitter;
use data::SharedData;

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
    generic_mood("p", api_data)
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
        "p" => {
            let data = api_data.lock().unwrap();
            data.paris_mood.get_mood_last_8_minutes()
        },
        "f" => {
            let data = api_data.lock().unwrap();
            data.french_mood.get_mood_last_8_minutes()
        },
        "e" => {
            let data = api_data.lock().unwrap();
            data.english_mood.get_mood_last_8_minutes()
        },
        _ => {
            let data = api_data.lock().unwrap();
            data.paris_mood.get_mood_last_8_minutes()
        }
    }
}