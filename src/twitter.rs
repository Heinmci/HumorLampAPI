use oauth;
use std::collections::HashMap;
use std::borrow::Cow;
use serde_json;
use oauth::Token;
use keys;

pub fn get_top_trend(location: &str) -> String {
    let consumer = Token::new(keys::CONSUMER_KEY, keys::CONSUMER_SECRET);
    let access = Token::new(keys::ACCESS_TOKEN, keys::ACCESS_SECRET);

    let mut test: HashMap<Cow<str>, Cow<str>> = HashMap::new();
    test.insert("id".into(), location.into());
    let bytes = oauth::get("https://api.twitter.com/1.1/trends/place.json",
                           &consumer,
                           Some(&access),
                           Some(&test)).expect("a");
    let test = String::from_utf8(bytes).expect("b");
    //println!("{}", test);
    //let v: Value = serde_json::from_str(&test).unwrap();
    //let t: &Value = &v[0]["trends"][0];
    //println!("{:#?}", v);
    let custom: Result<Vec<CustomObject>, serde_json::Error> = serde_json::from_str(&test);

    if custom.is_err() {
        return String::from("Problem occured");
    } 
    
    let mut biggest_trend: String = String::from("No Current Trends"); 
    let mut max_nb_tweets: u32 = 0;
    for i in custom.unwrap()[0].trends.iter() {
        if let Some(nb_tweets) = i.tweet_volume {
            if nb_tweets > max_nb_tweets {
                max_nb_tweets = nb_tweets;
                biggest_trend = i.name.clone();
            }
        }
    }
    println!("{}", biggest_trend);
    
    biggest_trend
}

#[derive(Serialize, Deserialize, Debug)]
struct Trend {
    pub name: String,
    promoted_content: Option<String>,
    query: String,
    pub tweet_volume: Option<u32>,
    url: String
}

#[derive(Serialize, Deserialize)]
struct CustomObject {
    pub trends: Vec<Trend>,    
    as_of: String,
    created_at: String,
    locations: Vec<Location>,
}

#[derive(Serialize, Deserialize)]
struct Location {
    name: String,
    woeid: u32
}