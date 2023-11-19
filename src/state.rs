#![allow(clippy::result_unit_err)]

pub mod ring_buffer;

use crate::player::AveragingStrategy;
use crate::player::Player;
use crate::state::ring_buffer::RingBuffer;
use crate::time::Timer;
use crate::time::Timings;
use macroquad::math::Vec2;
use macroquad::window::screen_height;
use macroquad::window::screen_width;

/// State of the game and a buffer of previous ticks.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct GameState {
    // /// Tick index, icreases every tick
    pub tick_number: usize,
    /// older ticks in front, newer in back
    pub buffer: RingBuffer<TickState>,
    // NOTE: GlobalState isnt here because of (re)borrow issues
    // cant do self.prev_tick_mut() and &self.global_state
}

#[allow(dead_code)]
impl GameState {
    pub fn new(buffer_len: usize) -> Self {
        Self { buffer: RingBuffer::with_capacity(buffer_len), ..Self::default() }
    }

    pub fn init(&mut self) -> &mut Self {
        let mut player = Player::default();
        player.movement.pos.x = screen_width() / 2.0;
        player.movement.pos.y = screen_height() / 2.0;

        let first_tick = TickState { player };
        self.buffer.fill_to_capacity(&first_tick);

        self
    }

    /// Returns whole seconds and ticks remainder passed
    pub fn gametime_passed(&self, tps: f32) -> (usize, usize) {
        // premptive f64
        let tick = self.tick_number;
        // TODO: this much prec not needed
        let (gametime_secs, gametime_ticks) = if tps.fract() == 0.0 {
            let tps = tps as usize;
            let n = tick / tps;
            let rem = tick % tps;
            (n, rem)
        } else {
            let n = (tick as f64 / tps as f64) as usize;
            let rem = (tick as f64 % tps as f64) as usize;
            (n, rem)
        };
        (gametime_secs, gametime_ticks)
    }

    pub fn current_tick(&self) -> &TickState {
        self.buffer.back().unwrap()
    }
    pub fn current_tick_mut(&mut self) -> &mut TickState {
        self.buffer.back_mut().unwrap()
    }

    pub fn prev_tick(&self) -> &TickState {
        self.get_prev_tick(1).unwrap()
    }
    pub fn prev_tick_mut(&mut self) -> &TickState {
        self.get_prev_tick_mut(1).unwrap()
    }

    // TODO: decide on api, get tick n, or get tick that is n ticks in the past

    /// Get the tick `tick` ticks in the past. 0 is current tick, 1 is previous tick.
    pub(crate) fn get_prev_tick(&self, tick: usize) -> Option<&TickState> {
        // VecDeque::back(): self.get(self.len.wrapping_sub(1))
        self.buffer.get(self.buffer.len().wrapping_sub(1 + tick))
    }
    /// Get the tick `tick` ticks in the past. 0 is current tick, 1 is previous tick.
    pub(crate) fn get_prev_tick_mut(&mut self, tick: usize) -> Option<&mut TickState> {
        // VecDeque::back_mut(): self.get_mut(self.len.wrapping_sub(1))
        self.buffer.get_mut(self.buffer.len().wrapping_sub(1 + tick))
    }

    pub(crate) fn get_tick(&self, tick_number: usize) -> Option<&TickState> {
        let current = self.tick_number;
        // None if tried get future tick. NOTE:CHEATS: should never happen
        let prev = current.checked_sub(tick_number)?;
        self.get_prev_tick(prev) // None if tried to get too old
    }
    pub(crate) fn get_tick_mut(&mut self, tick_number: usize) -> Option<&mut TickState> {
        let current = self.tick_number;
        // None if tried get future tick. NOTE:CHEATS: should never happen
        let prev = current.checked_sub(tick_number)?;
        self.get_prev_tick_mut(prev) // None if tried to get too old
    }

    /// Remove oldest tick, copy latest tick to current. Returns the now current tick
    /// (unmodified from the now previous tick)
    pub fn advance_tick(&mut self) -> &mut TickState {
        // NOTE:PANIC: only panics when not `init()`ed

        self.tick_number += 1;
        let latest_tick = self.buffer.back_mut().unwrap().clone();
        self.buffer.pop_front(); // remove oldest
        self.buffer.push_back(latest_tick); // copy latest
        self.buffer.back_mut().unwrap()
    }
}

/// State of one tick
#[derive(Clone, Debug, PartialEq, Default)]
pub struct TickState {
    pub player: Player,
}

/// Player input, interp settings, dbg info settings.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct GlobalState {
    /// Store input as fast as possible here until `update()`
    pub input_buffer: Vec<Vec2>,
    pub tick_settings: TickSettings,
    pub is_fullscreen: bool,
    pub update_timer: Timer,
    pub avg_strategy: AveragingStrategy,

    pub timings: Timings,
    pub dont_interpolate: bool,
    pub dbg_buffer: bool,
    pub dbg_hide_interp_info: bool,
}

impl GlobalState {
    pub fn new(tps: f32) -> Result<Self, ()> {
        let tick_settings = TickSettings::new(tps)?;
        Ok(Self {
            update_timer: Timer::new(tick_settings.tick_len_secs),
            tick_settings,
            ..Default::default()
        })
    }

    pub fn tick_progress(&self) -> f32 {
        1.0 - self.update_timer.time / self.tick_settings.tick_len_secs
    }

    pub fn set_timescale(&mut self, timescale: f32) -> &mut Self {
        self.tick_settings.set_timescale(timescale);
        self.update_timer.update_from_tick_settings(&self.tick_settings);
        self
    }

    pub fn set_tps(&mut self, game_state: &mut GameState, tps: f32) -> Result<(), ()> {
        self.tick_settings = self.tick_settings.set_tps(tps)?;
        game_state.buffer.resize(self.tick_settings.buffer_len);
        self.update_timer.update_from_tick_settings(&self.tick_settings);
        Ok(())
    }

    pub fn set_buffer_secs(&mut self, game_state: &mut GameState, secs: f32) -> Result<(), ()> {
        self.tick_settings.set_buffer_secs(secs)?;
        game_state.buffer.resize(self.tick_settings.buffer_len);
        self.update_timer.update_from_tick_settings(&self.tick_settings);
        Ok(())
    }

    pub fn set_avg_strategy(&mut self, avg_strat: AveragingStrategy) {
        self.avg_strategy = avg_strat;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TickSettings {
    /// Ticks per second.
    pub tps: f32,
    /// How long to wait until advancing to the next tick.
    pub tick_len_secs: f32,
    // TODO: needed?
    /// How long to buffer ticks for.
    pub buffer_secs: f32,
    /// How many tics to buffer.
    pub buffer_len: usize,
    /// How much to scale speed / acceleration when the tps changes.
    pub speed_factor: f32,
}

impl TickSettings {
    /// Reference ticks per seconds,`speed_factor` will be scaled according to this.
    /// At 30 tps, `speed_factor` will be 2.0. At 120 tps, `speed_factor` will be 0.5
    pub const REFERENCE_TPS: f32 = 60.0; // TODO: replace with "1 tps"
    const DEFAULT_BUFFER: f32 = 0.25;

    /// Create and initialize
    pub fn new(tps: f32) -> Result<Self, ()> {
        Self {
            tps,
            buffer_secs: Self::DEFAULT_BUFFER,
            tick_len_secs: Default::default(),
            buffer_len: Default::default(),
            speed_factor: Default::default(),
        }
        .calculate()
    }

    pub fn is_sane(&self) -> bool {
        self.tps >= 10.0
            && self.buffer_secs > 0.0
            && self.buffer_len > 1
            && self.speed_factor.is_normal()
    }

    /// Calculate rest of values from `tps` and `buffer_secs` and `REFERENCE_TPS`
    pub fn calculate(&self) -> Result<Self, ()> {
        let tick_len_secs = self.tps.recip();
        let buffer_len = (self.buffer_secs * self.tps).ceil() as usize;
        let speed_factor = Self::REFERENCE_TPS / self.tps;

        let new = Self { tick_len_secs, buffer_len, speed_factor, ..*self };
        if new.is_sane() {
            Ok(new)
        } else {
            Err(())
        }
    }

    /// Sets the ticks per seconds. Recalculates, preserves timescale.
    pub fn set_tps(&self, tps: f32) -> Result<Self, ()> {
        let mut new = self.clone();
        let timescale = self.timescale();

        new.tps = tps;
        new = new.calculate()?;

        new.set_timescale(timescale);
        Ok(new)
    }

    pub fn timescale(&self) -> f32 {
        let ideal_tick_len = self.tps.recip();
        let actual = self.tick_len_secs;
        ideal_tick_len / actual
    }

    /// Scale the length of ticks for slow motion or fast forward.
    pub fn set_timescale(&mut self, timescale: f32) -> &mut Self {
        let ideal_tick_len = self.tps.recip();
        let scaled_tick_len = ideal_tick_len * timescale.recip();
        self.tick_len_secs = scaled_tick_len;
        self
    }

    pub fn set_buffer_secs(&mut self, secs: f32) -> Result<(), ()> {
        let mut new = self.clone();
        let timescale = self.timescale();

        new.buffer_secs = secs;
        new = new.calculate()?;
        new.set_timescale(timescale);

        *self = new;
        Ok(())
    }
}

impl Default for TickSettings {
    fn default() -> Self {
        Self::new(60.0).unwrap()
    }
}