use futures::{Future, Stream};
use tokio_core::reactor::Core;
use twitter_stream::{Token, TwitterStreamBuilder};
use twitter_stream::message::StreamMessage;
use data::SharedData;
use data::mood::{MoodLocation, MoodKeywords};
use std::sync::{Mutex, Arc};
use keys;

pub fn generic_launch_stream(keyword: Vec<MoodKeywords>, shared_data: Arc<Mutex<SharedData>>, mood_location: MoodLocation) {
    let token = Token::new(keys::CONSUMER_KEY, keys::CONSUMER_SECRET, keys::ACCESS_TOKEN, keys::ACCESS_SECRET);

    let mut core = Core::new().unwrap();
    let twitter_keywords = generate_twitter_string(&keyword);

    let future = TwitterStreamBuilder::filter(&token).handle(&core.handle()).track(Some(&twitter_keywords)).listen().flatten_stream().for_each(|json| {
        if let Ok(StreamMessage::Tweet(tweet)) = StreamMessage::from_str(&json) {
            'outer: for mood_words in keyword.iter() {
                for word in mood_words.keywords().iter() {
                    if tweet.text.contains(word) {
                        let mut data = shared_data.lock().unwrap();
                        let mood = data.get_geo_mood_mut(&mood_location);
                        mood.increment_mood(mood_words.mood_type());
                        break 'outer;
                    }
                }
            }
        }
        Ok(())
    });

    core.run(future).unwrap();
}

fn generate_twitter_string(moods: &Vec<MoodKeywords>) -> String {
    let mut twitter_string = String::new();
    for mood in moods.iter() {
        for word in mood.keywords().iter() {
            twitter_string.push_str(&format!("{},", word))
        }
    }
    twitter_string.pop(); // Remove the last comma
    twitter_string
}

#[cfg(test)]
mod tests {
    use super::*;
    use data::mood::{MoodType, MoodKeywords};

    #[test]
    fn check_twitter_string_generation() {
        let sad = MoodKeywords::new(MoodType::Sad, vec!["deprime".to_string(), "anxieux".to_string(), "devaste".to_string()]);
        let moods_keywords = vec![sad];
        let twitter_string = generate_twitter_string(&moods_keywords);
        assert_eq!("deprime,anxieux,devaste", &twitter_string);
    }

    #[test]
    fn check_twitter_string_generation2() {
        let sad = MoodKeywords::new(MoodType::Sad, vec!["deprime".to_string(), "anxieux".to_string(), "devaste".to_string()]);
        let happy = MoodKeywords::new(MoodType::Happy, vec!["heureux".to_string(), "heureuse".to_string()]);
        let scared = MoodKeywords::new(MoodType::Scared, vec!["peur".to_string(), "terrifie".to_string()]);
        let moods_keywords = vec![sad, happy, scared];
        let twitter_string = generate_twitter_string(&moods_keywords);
        assert_eq!("deprime,anxieux,devaste,heureux,heureuse,peur,terrifie", &twitter_string);
    }
}
