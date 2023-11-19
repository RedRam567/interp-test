use std::time::{Duration, Instant};

use crate::state::TickSettings;

// TODO: better default
/// Decreasing timer with automatic wrapping and looping.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Timer {
    pub time: f32,
    pub start_time: f32,
}

impl Timer {
    pub fn new(length: f32) -> Self {
        Self { time: 0.0, start_time: length }
    }

    pub fn decrement(&mut self, time: f32) -> bool {
        self.time -= time;

        if self.time <= 0.0 {
            self.time += self.start_time;
            return true;
        }

        false
    }

    pub fn update_from_tick_settings(&mut self, tick_settings: &TickSettings) -> &mut Self {
        self.start_time = tick_settings.tick_len_secs;
        self
    }
}

/// Timings of the game loop
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Timings {
    pub start: Option<Instant>,
    pub pre_update: Option<Instant>,
    pub update: Option<Instant>,
    // since drawing starts before draw is set
    // pub prev_start: Option<Instant>,
    pub draw: Option<Instant>,
    pub waiting: Option<Instant>,
}

impl Timings {
    fn duration_since_opt(comparee: Option<Instant>, since: Option<Instant>) -> Option<Duration> {
        Some(comparee?.duration_since(since?))
    }

    pub fn total_duration(&self) -> Duration {
        Self::duration_since_opt(self.waiting, self.start).unwrap_or_default()
    }
    pub fn total_no_wait_duration(&self) -> Duration {
        Self::duration_since_opt(self.draw, self.start).unwrap_or_default()
    }

    pub fn pre_update_duration(&self) -> Duration {
        Self::duration_since_opt(self.pre_update, self.start).unwrap_or_default()
    }
    pub fn update_duration(&self) -> Duration {
        Self::duration_since_opt(self.update, self.pre_update).unwrap_or_default()
    }
    pub fn draw_duration(&self) -> Duration {
        Self::duration_since_opt(self.draw, self.start).unwrap_or_default()
    }
    pub fn waiting_duration(&self) -> Duration {
        Self::duration_since_opt(self.waiting, self.start).unwrap_or_default()
    }
}
