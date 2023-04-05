use std::fmt;

#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        if minutes >= 60 {
            Clock {
                hours: (hours + minutes / 60).rem_euclid(24),
                minutes: minutes.rem_euclid(60),
            }
        } else {
            Clock {
                hours: hours.rem_euclid(24),
                minutes,
            }
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        if &self.minutes + minutes >= 60 {
            Clock {
                hours: (self.hours + (&self.minutes + minutes) / 60).rem_euclid(24),
                minutes: (&self.minutes + minutes).rem_euclid(60),
            }
        } else if &self.minutes + minutes < 0 {
            Clock {
                hours: (self.hours + (&self.minutes + minutes) / 60).rem_euclid(24),
                minutes: (&self.minutes + minutes).rem_euclid(60),
            }
        } else {
            Clock {
                hours: self.hours.rem_euclid(24),
                minutes: self.minutes + minutes,
            }
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.hours < 10 {
            if self.minutes < 10 {
                return write!(f, "0{}:0{}", self.hours, self.minutes);
            }
            return write!(f, "0{}:{}", self.hours, self.minutes);
        } else {
            if self.minutes < 10 {
                return write!(f, "{}:0{}", self.hours, self.minutes);
            }
        }
        write!(f, "{}:{}", self.hours, self.minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Clock")
            .field("hours", &self.hours)
            .field("minutes", &self.minutes)
            .finish()
    }
}
