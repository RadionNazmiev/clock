// Implement a clock that handles times without dates.
// You should be able to add and subtract minutes to it.
// Two clocks that represent the same time should be equal to each other.
// Rust Traits for .to_string()
// Did you implement .to_string() for the Clock struct?
// If so, try implementing the Display trait for Clock instead.
// Traits allow for a common way to implement functionality for various types.
// For additional learning, consider how you might implement String::from for the Clock type. You don't have to actually implement this—it's redundant with Display,
// which is generally the better choice when the destination type is String—but it's useful to have a few type-conversion traits in your toolkit.

use std::fmt;

const DAY: i64 = 24 * 60;
const HOUR: i64 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i64,
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Clock {
        Clock {
            minutes: (((hours * HOUR + minutes) % DAY) + DAY) % DAY
        }
    }

    pub fn add_minutes(self, minutes: i64) -> Clock {
        Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes/HOUR, self.minutes%HOUR)
    }
}