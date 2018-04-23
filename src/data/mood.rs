use std::collections::{VecDeque};
use chrono_time::PreciseTime;
use std::cmp::Ordering;

pub struct MoodHistory {
    mood_history: VecDeque<Moods>,
    moods_of_interest: Vec<(MoodType, MoodColour)>,
    pub seconds_since_switch: PreciseTime
}

impl MoodHistory {
    pub fn new(nb_minutes: usize) -> MoodHistory {
        let mut history = VecDeque::with_capacity(nb_minutes);
        let moods_of_interest = vec![(MoodType::Happy, MoodColour::Blue), (MoodType::Sad, MoodColour::Green), (MoodType::Scared, MoodColour::Red)];
        let moods = Moods::new(&moods_of_interest);
        for _ in 0..8 {
            history.push_back(moods.clone());
        }

        MoodHistory {
            mood_history: history,
            moods_of_interest,
            seconds_since_switch: PreciseTime::now()
        }
    }

    pub fn get_current_mood(&self) -> String {
        self.mood_history[0].get_rgb()
    }
    
    pub fn increment_mood(&mut self, mood_type: MoodType) {
        let time_now = PreciseTime::now();
        let diff = self.seconds_since_switch.to(time_now).num_seconds();

        if diff > 60 {
            self.mood_history.pop_back();
            self.mood_history.push_front(Moods::new(&self.moods_of_interest));
            self.seconds_since_switch = time_now;
        }

        self.mood_history[0].increment_mood(mood_type);

    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum MoodLocation {
    French,
    English
}

#[derive(Debug, Clone)]
pub struct Moods {
    moods: Vec<Mood>
}

impl Moods {
    pub fn new(types: &Vec<(MoodType, MoodColour)>) -> Moods {
        Moods {
            moods: types.iter().map(|&(tp, colour)| Mood::new(tp, colour)).collect::<Vec<Mood>>()
        }
    }
    
    // TODO: Solve problem when counter is the same for multiple moods, right now only last in iterator is the max if counter is the same
    pub fn get_rgb(&self) -> String {
        let top_mood = self.moods.iter().max();
        match top_mood {
            Some(value) => MoodColour::colour_to_rgb_string(value.colour()),
            None => String::from("000,000,000")
        }
    }

    pub fn increment_mood(&mut self, mood_type: MoodType) {
        self.moods.iter_mut().find(|x| x.mood_type() == mood_type).unwrap().increment_mood();
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Mood {
    mood_type: MoodType,
    counter: u32,
    colour: MoodColour
}

impl Mood {
    pub fn new(mood_type: MoodType, colour: MoodColour) -> Mood {
        Mood {
            mood_type,
            counter: 0,
            colour
        }
    }

    pub fn increment_mood(&mut self) {
        self.counter += 1;
    }

    pub fn colour(&self) -> MoodColour {
        self.colour
    }

    pub fn mood_type(&self) -> MoodType {
        self.mood_type
    }
}

impl Ord for Mood {
    fn cmp(&self, other: &Mood) -> Ordering {
        self.counter.cmp(&other.counter)
    }
}

impl PartialOrd for Mood {
    fn partial_cmp(&self, other: &Mood) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Mood {
    fn eq(&self, other: &Mood) -> bool {
        self.counter == other.counter
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MoodType {
    Happy,
    Scared,
    Sad 
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MoodColour {
    Green,
    Blue,
    Red
}

impl MoodColour {
    pub fn colour_to_rgb_string(colour: MoodColour) -> String {
        match colour {
            MoodColour::Green => String::from("000,255,000"),
            MoodColour::Blue => String::from("000,000,255"),
            MoodColour::Red => String::from("255,000,000")
        }
    }
}