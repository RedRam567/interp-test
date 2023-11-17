use std::collections::VecDeque;
use std::time::Instant;

/// gameloop render time timings
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Timings {
    pub total: VecDeque<Instant>,
    pub pre_update: VecDeque<Instant>,
    pub update: VecDeque<Instant>,
    pub draw: VecDeque<Instant>,
    pub draw_dbg: VecDeque<Instant>,
}

