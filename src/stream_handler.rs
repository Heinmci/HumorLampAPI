use futures::{Future, Stream};
use tokio_core::reactor::Core;
use twitter_stream::{Token, TwitterStreamBuilder};
use twitter_stream::message::StreamMessage;
use data::SharedData;
use std::sync::{Mutex, Arc};
use keys;

pub fn launch_french_stream(keyword: &str, shared_data: Arc<Mutex<SharedData>>) {
    let token = Token::new(keys::CONSUMER_KEY, keys::CONSUMER_SECRET, keys::ACCESS_TOKEN, keys::ACCESS_SECRET);

    let mut core = Core::new().unwrap();

    let future = TwitterStreamBuilder::filter(&token).handle(&core.handle()).track(Some(keyword)).listen().flatten_stream().for_each(|json| {
        if let Ok(StreamMessage::Tweet(tweet)) = StreamMessage::from_str(&json) {
            
            match tweet.text {
                _ if tweet.text.contains("peur") || tweet.text.contains("terrifie") || tweet.text.contains("anxieux") || tweet.text.contains("stress") => {
                    let mut data = shared_data.lock().unwrap();
                    data.french_mood.incr_scared();
                    //println!("{}", tweet.text);
                    //println!("Scared: {:?}", data.french_mood.get_mood_last_8_minutes());
                }
                _ if tweet.text.contains("heureux") || tweet.text.contains("heureuse") || tweet.text.contains("joyeux") => {
                    let mut data = shared_data.lock().unwrap();
                    data.french_mood.incr_happy();
                    //println!("{}", tweet.text);
                    //println!("Happy: {:?}", data.french_mood.get_mood_last_8_minutes());
                }
                _ if tweet.text.contains("deprime") || tweet.text.contains("devaste") => {
                    let mut data = shared_data.lock().unwrap();
                    data.french_mood.incr_sad();
                    //println!("{}", tweet.text);
                    //println!("Sad: {:?}", data.french_mood.get_mood_last_8_minutes());
                }
                _ => ()
            }
        }
        Ok(())
    });

    core.run(future).unwrap();
}

pub fn launch_english_stream(keyword: &str, shared_data: Arc<Mutex<SharedData>>) {
    let token = Token::new(keys::CONSUMER_KEY, keys::CONSUMER_SECRET, keys::ACCESS_TOKEN, keys::ACCESS_SECRET);

    let mut core = Core::new().unwrap();

    let future = TwitterStreamBuilder::filter(&token).handle(&core.handle()).track(Some(keyword)).listen().flatten_stream().for_each(|json| {
        if let Ok(StreamMessage::Tweet(tweet)) = StreamMessage::from_str(&json) {
            
            match tweet.text {
                _ if tweet.text.contains("scared") || tweet.text.contains("terrified") || tweet.text.contains("frightened") => {
                    let mut data = shared_data.lock().unwrap();
                    data.english_mood.incr_scared();
                    //println!("{}", tweet.text);
                    //println!("Scared: {:?}", data.english_mood.get_mood_last_8_minutes());
                }
                _ if tweet.text.contains("happy") => {
                    let mut data = shared_data.lock().unwrap();
                    data.english_mood.incr_happy();
                    //println!("{}", tweet.text);
                    //println!("Happy: {:?}", data.english_mood.get_mood_last_8_minutes());
                }
                _ if tweet.text.contains("sad") || tweet.text.contains("depressed") => {
                    let mut data = shared_data.lock().unwrap();
                    data.english_mood.incr_sad();
                    //println!("{}", tweet.text);
                    //println!("Sad: {:?}", data.english_mood.get_mood_last_8_minutes());
                }
                _ => ()
            }
        }
        Ok(())
    });

    core.run(future).unwrap();
}

pub fn launch_parisian_stream(keyword: &str, shared_data: Arc<Mutex<SharedData>>) {
    let token = Token::new(keys::CONSUMER_KEY, keys::CONSUMER_SECRET, keys::ACCESS_TOKEN, keys::ACCESS_SECRET);

    let mut core = Core::new().unwrap();

    let future = TwitterStreamBuilder::filter(&token).handle(&core.handle()).track(Some(keyword)).locations(Some(&[((-122.75,36.8),(-121.75,37.8))])).listen().flatten_stream().for_each(|json| {
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