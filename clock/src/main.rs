#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct Clock {
    hours: i16,
    minutes: i16,
}

impl Clock {
    fn new(hours: i16, minutes: i16) -> Self {
        Self { hours, minutes }.normalize()
    }

    fn normalize(self) -> Self {
        let mut hours = (self.hours + self.minutes / 60) % 24;
        let mut minutes = self.minutes % 60;

        if minutes < 0 {
            minutes += 60;
            hours -= 1;
        }

        if hours < 0 {
            hours += 24;
        }

        Self { hours, minutes }
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl From<i16> for Clock {
    fn from(val: i16) -> Self {
        Clock::new(0, val)
    }
}

impl From<u8> for Clock {
    fn from(val: u8) -> Self {
        Clock::new(0, val.into())
    }
}

fn main() {
    let clock = Clock::new(-9, 124);
    println!("{clock}");
    let clock = Clock::default();
    println!("{clock}");
    let str_rep = Clock::default().to_string();
    println!("{str_rep}");
    let clock = Clock::from(1000_i16);
    println!("{clock}");
    let clock: Clock = 1000_i16.into();
    println!("{clock}");
    let clock_1 = Clock::new(8, 10);
    let clock_2 = Clock::new(10, 8);
    println!("{}", clock_1 < clock_2);
}
