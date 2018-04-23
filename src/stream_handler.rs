use futures::{Future, Stream};
use tokio_core::reactor::Core;
use twitter_stream::{Token, TwitterStreamBuilder};
use twitter_stream::message::StreamMessage;
use data::SharedData;
use data::mood::{MoodLocation, MoodType};
use std::sync::{Mutex, Arc};
use keys;


// TODO: pass a HashMap of <MoodType, Vec![String]> representing each keyword corresponding to a mood and use that in the match clause
pub fn generic_launch_stream(keyword: &str, shared_data: Arc<Mutex<SharedData>>, mood_location: MoodLocation) {
    let token = Token::new(keys::CONSUMER_KEY, keys::CONSUMER_SECRET, keys::ACCESS_TOKEN, keys::ACCESS_SECRET);

    let mut core = Core::new().unwrap();

    let future = TwitterStreamBuilder::filter(&token).handle(&core.handle()).track(Some(keyword)).listen().flatten_stream().for_each(|json| {
        if let Ok(StreamMessage::Tweet(tweet)) = StreamMessage::from_str(&json) {
            
            match tweet.text {
                _ if tweet.text.contains("peur") || tweet.text.contains("terrifie") || tweet.text.contains("anxieux") || tweet.text.contains("stress") => {
                    let mut data = shared_data.lock().unwrap();
                    let mood = data.get_geo_mood_mut(&mood_location);
                    mood.increment_mood(MoodType::Scared);
                }
                _ if tweet.text.contains("heureux") || tweet.text.contains("heureuse") || tweet.text.contains("joyeux") => {
                    let mut data = shared_data.lock().unwrap();
                    let mood = data.get_geo_mood_mut(&mood_location);
                    mood.increment_mood(MoodType::Happy);
                }
                _ if tweet.text.contains("deprime") || tweet.text.contains("devaste") => {
                    let mut data = shared_data.lock().unwrap();
                    let mood = data.get_geo_mood_mut(&mood_location);
                    mood.increment_mood(MoodType::Sad);
                }
                _ => ()
            }
        }
        Ok(())
    });

    core.run(future).unwrap();
}
