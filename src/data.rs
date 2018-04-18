use std::collections::VecDeque;
use chrono_time::PreciseTime;

pub struct SharedData {
    pub paris_mood: MoodInTime,
    pub french_mood: MoodInTime,
    pub english_mood: MoodInTime
}

impl SharedData {
    pub fn new() -> SharedData {
        SharedData {
            paris_mood: MoodInTime::new(),
            french_mood: MoodInTime::new(),
            english_mood: MoodInTime::new()
        }
    }
}


pub struct MoodInTime {
    mood_last_8_mniutes: VecDeque<Mood>,
    pub seconds_since_switch: PreciseTime
}

impl MoodInTime {
    pub fn new() -> MoodInTime {
        let mut vd = VecDeque::with_capacity(8);
        for _ in 0..8 {
            vd.push_back(Mood::new());
        }
        MoodInTime {
            mood_last_8_mniutes: vd,
            seconds_since_switch: PreciseTime::now()
        }
    }

    fn get_rgb(mood: &Mood) -> String {
        if mood.happy > mood.sad && mood.happy > mood.scared {
            String::from("0,0,255|") // HAPPY
        } else if mood.happy > mood.sad && mood.happy < mood.scared {
             String::from("0,255,0|") // SCARED
        } else if mood.sad > mood.scared {
            String::from("255,0,0|") // SAD
        } else {
            String::from("255,255,255|") // EVERYTHING IS 0
        }
    }

    pub fn get_mood_last_8_minutes(&self) -> String {
        let mut result = self.mood_last_8_mniutes.iter().map(|x| MoodInTime::get_rgb(x)).collect::<String>();
        result.pop(); // Remove last | char
        result

    }
    
    pub fn incr_happy(&mut self) {
        let time_now = PreciseTime::now();
        let diff = self.seconds_since_switch.to(time_now).num_seconds();

        if diff > 60 {
            self.mood_last_8_mniutes.pop_back();
            self.mood_last_8_mniutes.push_front(Mood::new());
            self.mood_last_8_mniutes[0].incr_happy();
            self.seconds_since_switch = time_now;
        } else {
            self.mood_last_8_mniutes[0].incr_happy();
        }
    }

    pub fn incr_scared(&mut self) {
        let time_now = PreciseTime::now();
        let diff = self.seconds_since_switch.to(time_now).num_seconds();

        if diff > 60 {
            self.mood_last_8_mniutes.pop_back();
            self.mood_last_8_mniutes.push_front(Mood::new());
            self.mood_last_8_mniutes[0].incr_scared();
            self.seconds_since_switch = time_now;
        } else {
            self.mood_last_8_mniutes[0].incr_scared();
        }
    }

    pub fn incr_sad(&mut self) {
        let time_now = PreciseTime::now();
        let diff = self.seconds_since_switch.to(time_now).num_seconds();

        if diff > 60 {
            self.mood_last_8_mniutes.pop_back();
            self.mood_last_8_mniutes.push_front(Mood::new());
            self.mood_last_8_mniutes[0].incr_sad();
            self.seconds_since_switch = time_now;
        } else {
            self.mood_last_8_mniutes[0].incr_sad();
        }
    }
}

#[derive(Debug)]
pub struct Mood {
    pub happy: u32,
    pub scared: u32,
    pub sad: u32
}

impl Mood {
    pub fn new() -> Mood {
        Mood {
            happy: 0,
            scared: 0,
            sad: 0
        }
    }

    pub fn incr_happy(&mut self) {
        self.happy += 1;
    }

    pub fn incr_scared(&mut self) {
        self.scared += 1;
    }

    pub fn incr_sad(&mut self) {
        self.sad += 1;
    }
}