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

use data::SharedData;
use futures::{Future, Stream};
use tokio_core::reactor::Core;
use twitter_stream::{Token, TwitterStreamBuilder};
use twitter_stream::message::StreamMessage;
use std::{thread};
use std::sync::{Mutex, Arc};
use tiny_http::{Server, Response};

//use time::PreciseTime;


fn main() {
    let shared_data = Arc::new(Mutex::new(SharedData::new()));
    let api_data = Arc::clone(&shared_data);
    let french_stream_data = Arc::clone(&shared_data);
    let api_thread = thread::spawn( move || {
        let server = Server::http("0.0.0.0:8080").unwrap();

        for request in server.incoming_requests() {
            println!("received request! method: {:?}, url: {:?}",
                request.method(),
                request.url()
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
    
    let stream1 = thread::spawn( move || {
        launch_stream("heureux, heuereuse, peur, terrifie, deprime, joyeux, anxieux, stresse, devaste", french_stream_data);
    });

    // let stream2 = thread::spawn( move || {
    //     launch_stream("europa universalis");
    // });

    stream1.join();
    //stream2.join();
    api_thread.join();

}

fn launch_stream(keyword: &str, shared_data: Arc<Mutex<SharedData>>) {
    let token = Token::new(keys::CONSUMER_KEY, keys::CONSUMER_SECRET, keys::ACCESS_TOKEN, keys::ACCESS_SECRET);

    let mut core = Core::new().unwrap();

    let future = TwitterStreamBuilder::filter(&token).handle(&core.handle()).track(Some(keyword)).listen().flatten_stream().for_each(|json| {
        if let Ok(StreamMessage::Tweet(tweet)) = StreamMessage::from_str(&json) {
            
            match tweet.text {
                _ if tweet.text.contains("peur") || tweet.text.contains("terrifie") || tweet.text.contains("anxieux") || tweet.text.contains("stress") => {
                    let mut data = shared_data.lock().unwrap();
                    data.french_mood.incr_scared();
                    println!("{}", tweet.text);
                    println!("Scared: {:?}", data.french_mood.get_mood_last_8_minutes());
                }
                _ if tweet.text.contains("heureux") || tweet.text.contains("heureuse") || tweet.text.contains("joyeux") => {
                    let mut data = shared_data.lock().unwrap();
                    data.french_mood.incr_happy();
                    println!("{}", tweet.text);
                    println!("Happy: {:?}", data.french_mood.get_mood_last_8_minutes());
                }
                _ if tweet.text.contains("deprime") || tweet.text.contains("devaste") => {
                    let mut data = shared_data.lock().unwrap();
                    data.french_mood.incr_sad();
                    println!("{}", tweet.text);
                    println!("Sad: {:?}", data.french_mood.get_mood_last_8_minutes());
                }
                _ => ()
            }
        }
        Ok(())
    });

    core.run(future).unwrap();
}

