#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TimeControl {
    pub initial_seconds: u32,
    pub increment_seconds: u32,
}

impl TimeControl {
    pub const fn new(initial_seconds: u32, increment_seconds: u32) -> Self {
        Self {
            initial_seconds,
            increment_seconds,
        }
    }

    pub fn label(&self) -> String {
        format!("{} + {}", self.initial_seconds / 60, self.increment_seconds)
    }
}

pub const STANDARD_TIME_CONTROLS: [TimeControl; 10] = [
    TimeControl::new(60, 0),
    TimeControl::new(60, 1),
    TimeControl::new(120, 1),
    TimeControl::new(180, 0),
    TimeControl::new(180, 2),
    TimeControl::new(300, 0),
    TimeControl::new(300, 3),
    TimeControl::new(600, 0),
    TimeControl::new(600, 5),
    TimeControl::new(900, 10),
];
