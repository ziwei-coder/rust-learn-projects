use std::time::{Duration, Instant};

pub trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    }
}

pub struct Timer {
    period: Duration,
    pub delta: Duration,
    pub last_instant: Instant,
    pub countdown: Duration,
    pub ready: bool,
}

impl Timer {
    pub fn new() -> Self {
        let now = Instant::now();

        Self {
            last_instant: now,
            delta: Duration::default(),
            period: Duration::from_millis(1000),
            countdown: Duration::default(),
            ready: true,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;
        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        });
    }
}

#[cfg(test)]
mod tests {
    use super::TimeOutput;

    #[test]
    fn test_as_time() {
        let pairs = vec![
            (5_u64, "0:00:05"),
            (60_u64, "0:01:00"),
            (154_u64, "0:02:34"),
            (3603_u64, "1:00:03"),
        ];

        for (input, output) in pairs {
            assert_eq!(input.as_time(), output);
        }
    }
}
