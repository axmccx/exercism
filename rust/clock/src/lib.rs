use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minute_sum = self.minutes + minutes;
        let extra_hour = minute_sum / 60;
        let new_minutes = minute_sum % 60;
        let new_hour = (self.hours + extra_hour) % 24;

        Clock {
            hours: new_hour,
            minutes: new_minutes
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_hours = match self.hours  {
            n if n < 9 => format!("0{n}"),
           _ => self.hours.to_string()
        };

        let display_minutes = match self.minutes {
            n if n < 9 => format!("0{n}"),
            _ => self.minutes.to_string()
        };

        write!(f, "{}:{}", display_hours, display_minutes)
    }
}