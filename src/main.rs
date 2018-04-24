extern crate twitter_stream;
extern crate futures;
extern crate tokio_core;
extern crate tokio;
extern crate time as chrono_time;
extern crate oauth_client as oauth;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tiny_http;

mod data;
mod handler;
mod twitter;
mod keys;
mod stream_handler;

use data::{SharedData};
use data::mood::{MoodLocation, MoodType, MoodKeywords};

use std::{thread};
use std::sync::{Mutex, Arc};
use tiny_http::{Server, Response};
use std::time::SystemTime;
use tokio::prelude::*;
use tokio::timer::Interval;
use std::time::{Duration, Instant};

// TODO: Import Moods and Colours from a file rather than in the code / Resolve tokio dependency nightmare...

fn main() {
    let shared_data = Arc::new(Mutex::new(SharedData::new()));
    let api_data = Arc::clone(&shared_data);
    let french_stream_data = Arc::clone(&shared_data);
    let english_stream_data = Arc::clone(&shared_data);
    let timer_shared_data = Arc::clone(&shared_data);

    let api_thread = thread::spawn( move || {
        let server = Server::http("0.0.0.0:8080").unwrap();

        for request in server.incoming_requests() {
            println!("received request! method: {:?}, url: {:?}, time: {:?}",
                request.method(),
                request.url(),
                SystemTime::now()
            );

            let return_string = match request.url() {
                "/paris_trend" => handler::paris_trend(),
                "/france_trend" => handler::france_trend(),
                "/english_trend" => handler::english_trend(),
                "/paris_mood" => handler::paris_mood(&api_data),
                "/france_mood" => handler::france_mood(&api_data),
                "/english_mood" => handler::english_mood(&api_data),
                _ => String::from("Unknown route")

            };
            request.respond(Response::from_string(return_string));
        }

    });
    
    let french_stream = thread::spawn( move || {
        let sad = MoodKeywords::new(MoodType::Sad, vec!["deprime".to_string(), "anxieux".to_string(), "devaste".to_string()]);
        let happy = MoodKeywords::new(MoodType::Happy, vec!["heureux".to_string(), "heuereuse".to_string()]);
        let scared = MoodKeywords::new(MoodType::Scared, vec!["peur".to_string(), "terrifie".to_string()]);
        let moods_keywords = vec![sad, happy, scared];
        stream_handler::generic_launch_stream(moods_keywords, french_stream_data, MoodLocation::French);
    });

    let english_stream = thread::spawn( move || {
        let sad = MoodKeywords::new(MoodType::Sad, vec!["sad".to_string(), "devastated".to_string()]);
        let happy = MoodKeywords::new(MoodType::Happy, vec!["happy".to_string(), "extatic".to_string()]);
        let scared = MoodKeywords::new(MoodType::Scared, vec!["scared".to_string(), "frightened".to_string(), "terrified".to_string()]);
        let moods_keywords = vec![sad, happy, scared];
        stream_handler::generic_launch_stream(moods_keywords, english_stream_data, MoodLocation::English);
    });

    let task = Interval::new(Instant::now(), Duration::from_secs(60))
        .for_each(move |instant| {
            println!("Adding period to vecdeque");
            let mut data = timer_shared_data.lock().unwrap();
            let geo_moods = data.get_geo_moods_mut();
            for (_, mood_history) in geo_moods.iter_mut() {
                mood_history.add_new_period_to_history();
                println!("{:?}", mood_history);
            }
            Ok(())
        })
        .map_err(|e| panic!("interval errored; err={:?}", e));

    tokio::run(task);




    french_stream.join();
    english_stream.join();
    api_thread.join();

}



