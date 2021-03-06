extern crate twitter_stream;
extern crate futures;
extern crate tokio_core;
extern crate time as chrono_time;
extern crate oauth_client as oauth;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tiny_http;


//mod api;
mod data;
mod handler;
mod twitter;
mod keys;
mod stream_handler;

use data::{SharedData};

use std::{thread};
use std::sync::{Mutex, Arc};
use tiny_http::{Server, Response};
use std::time::SystemTime;

//use time::PreciseTime;


fn main() {
    //twitter::get_paris_coord();
    let shared_data = Arc::new(Mutex::new(SharedData::new()));
    let api_data = Arc::clone(&shared_data);
    let french_stream_data = Arc::clone(&shared_data);
    let english_stream_data = Arc::clone(&shared_data);
    //let paris_stream_data = Arc::clone(&shared_data);

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
        stream_handler::launch_french_stream("heureux, heuereuse, peur, terrifie, deprime, anxieux, stresse, devaste", french_stream_data);
    });

    let english_stream = thread::spawn( move || {
        stream_handler::launch_english_stream("scared, happy, sad, frightened, terrified", english_stream_data);
    });

    // let paris_stream = thread::spawn( move || {
    //     stream_handler::launch_parisian_stream("heureux, heuereuse, peur, terrifie, deprime, joyeux, anxieux, stresse, devaste", paris_stream_data);
    // });

    french_stream.join();
    english_stream.join();
   // paris_stream.join();
    api_thread.join();

}



