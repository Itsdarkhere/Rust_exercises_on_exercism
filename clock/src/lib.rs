#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours: (hours + minutes.div_euclid(60)).rem_euclid(24), minutes: minutes.rem_euclid(60) }
    }

    pub fn add_minutes(&self, minutes: i32) -> Clock {
        Clock::new(self.hours, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{:0>#2}:{:0>#2}", self.hours, self.minutes)
    }
}
