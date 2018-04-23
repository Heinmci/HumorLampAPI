pub mod mood;

use std::collections::HashMap;
use self::mood::{MoodLocation, MoodHistory};

pub struct SharedData {
    pub geo_moods: HashMap<MoodLocation, MoodHistory>
}

impl SharedData {
    pub fn new() -> SharedData {
        let mut geo_moods = HashMap::new();
        geo_moods.insert(MoodLocation::French, MoodHistory::new(8));
        geo_moods.insert(MoodLocation::English, MoodHistory::new(8));

        SharedData {
            geo_moods
        }
    }

    pub fn get_geo_mood_mut(&mut self, location: &MoodLocation) -> &mut MoodHistory {
        self.geo_moods.get_mut(&location).unwrap()
    }

    pub fn get_geo_mood(&self, location: &MoodLocation) -> &MoodHistory {
        self.geo_moods.get(&location).unwrap()
    }
}