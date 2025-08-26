use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    fn normalize_minutes(minutes: i32) -> (i32, i32) {
        let wrapped_minutes = (minutes % 1440 + 1440) % 1440;
        (wrapped_minutes / 60, wrapped_minutes % 60)
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let (h, m) = Self::normalize_minutes(hours * 60 + minutes);
        Clock { hours: h, minutes: m }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let current_minutes = self.hours * 60 + self.minutes;
        let (h, m) = Self::normalize_minutes(current_minutes + minutes);
        Clock { hours: h, minutes: m}
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}