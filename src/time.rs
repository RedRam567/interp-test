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
}
