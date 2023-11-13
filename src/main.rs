use std::collections::VecDeque;

use interp_test::movement::Movement;
use interp_test::time::Timer;
use interp_test::{dbg_arrow, Player, DBG_INTERP, DBG_NOW, DBG_PREV};
use macroquad::prelude::*;
use macroquad::window::{screen_height, screen_width};

// const UPDATES_PER_SECOND: f32 = 10.0;
// const UPDATES_PER_SECOND: f32 = 15.0;
// const TICK_LEN_SECS: f32 = 1.0 / UPDATES_PER_SECOND * 10.0;
const UPDATES_PER_SECOND: f32 = 30.0;
const TICK_LEN_SECS: f32 = 1.0 / UPDATES_PER_SECOND * 5.0;
const TICK_BUFFER_SECS: f32 = 0.25;
const TICK_BUFFER_LEN: usize = (TICK_BUFFER_SECS * UPDATES_PER_SECOND + 0.5) as usize; // + 0.5 as ceil is not const (wtf)

/// TODO:DOCS:
const SPEED_FACTOR: f32 = 60.0 / UPDATES_PER_SECOND;
const PLAYER_MAX_SPEED: f32 = 20.0 * SPEED_FACTOR * 1.0;
const PLAYER_ACCEL: f32 = 5.0 * SPEED_FACTOR * SPEED_FACTOR;

#[derive(Clone, Debug, PartialEq, Default)]
struct GameState {
    // /// Tick index, icreases every tick
    tick_number: usize,
    /// older ticks in front, newer in back
    tick_state: VecDeque<TickState>,
    // no global_state here because of (re)borrow issues
    // cant do self.prev_tick_mut() and &self.global_state
    // global_state: GlobalState,
}

// #[allow(dead_code)]
impl GameState {
    fn new() -> Self {
        // Self::default()
        Self {
            tick_state: VecDeque::with_capacity(TICK_BUFFER_LEN),
            ..Self::default()
        }
    }

    fn init(&mut self) -> &mut Self {
        // let player = self.tick_state.front_mut().unwrap();
        dbg!(self.tick_state.len());
        let mut player = Player::default();
        player.movement.pos.x = screen_width() / 2.0;
        player.movement.pos.y = screen_height() / 2.0;

        let first_tick = TickState { player };
        for _ in 0..self.tick_state.capacity() {
            self.tick_state.push_back(first_tick.clone());
            // *state = first_tick.clone()
        }

        self
    }

    fn current_tick(&self) -> &TickState {
        self.tick_state.back().unwrap()
    }
    fn current_tick_mut(&mut self) -> &mut TickState {
        self.tick_state.back_mut().unwrap()
    }

    fn prev_tick(&self) -> &TickState {
        self.get_prev_tick(1).unwrap()
    }
    fn prev_tick_mut(&mut self) -> &TickState {
        self.get_prev_tick_mut(1).unwrap()
    }

    /// Get the tick `tick` ticks in the past. 0 is current tick, 1 is previous tick.
    fn get_prev_tick(&self, tick: usize) -> Option<&TickState> {
        // VecDeque::back(): self.get(self.len.wrapping_sub(1))
        self.tick_state
            .get(self.tick_state.len().wrapping_sub(1 + tick))
    }
    /// Get the tick `tick` ticks in the past. 0 is current tick, 1 is previous tick.
    fn get_prev_tick_mut(&mut self, tick: usize) -> Option<&mut TickState> {
        // VecDeque::back_mut(): self.get_mut(self.len.wrapping_sub(1))
        self.tick_state
            .get_mut(self.tick_state.len().wrapping_sub(1 + tick))
    }

    fn get_tick(&self, tick_number: usize) -> Option<&TickState> {
        let current = self.tick_number;
        // None if tried get future tick. NOTE:CHEATS: should never happen
        let prev = current.checked_sub(tick_number)?;
        self.get_prev_tick(prev) // None if tried to get too old
    }
    fn get_tick_mut(&mut self, tick_number: usize) -> Option<&mut TickState> {
        let current = self.tick_number;
        // None if tried get future tick. NOTE:CHEATS: should never happen
        let prev = current.checked_sub(tick_number)?;
        self.get_prev_tick_mut(prev) // None if tried to get too old
    }

    /// Remove oldest tick, copy latest tick to current. Returns the now current tick
    /// (unmodified from the now previous tick)
    fn advance_tick(&mut self) -> &mut TickState {
        // NOTE:PANIC: only panics when not `init()`ed

        self.tick_number += 1;
        let latest_tick = self.tick_state.back_mut().unwrap().clone();
        self.tick_state.pop_front(); // remove oldest
        self.tick_state.push_back(latest_tick); // copy latest
        self.tick_state.back_mut().unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
struct TickState {
    player: Player,
}

#[derive(Clone, Debug, PartialEq, Default)]
struct GlobalState {
    /// Store input as fast as possible here until `update()`
    input_buffer: Vec<Vec2>,
}

// #[derive(Clone, Debug, PartialEq, Default)]
// struct ServerState {

// }

#[macroquad::main("interp test")]
async fn main() {
    let mut game_state = GameState::new();
    game_state.init();
    dbg!(game_state.tick_state.len());
    let mut global_state = GlobalState::default();
    // let mut prev_game_state = game_state.clone();
    // let game = &mut game_state;
    // let mut prev_game = &mut prev_game_state;
    let mut update_timer = Timer::new(TICK_LEN_SECS);
    let mut dont_interpolate = false;
    dbg!(SPEED_FACTOR);
    loop {
        // game_state.player.handle_movement(PLAYER_MAX_SPEED);

        let delta_time = get_frame_time();
        let ready_to_update = update_timer.decrement(delta_time);

        // Input and Update
        handle_inputs(&mut global_state.input_buffer); // as close to update as possible
        if ready_to_update {
            // prev_game_state = game_state.clone();
            game_state.advance_tick();
            update(&mut game_state, &mut global_state);
            global_state.input_buffer.clear()
        }

        if is_key_pressed(KeyCode::Q) {
            dont_interpolate = !dont_interpolate;
        }

        // Drawing
        let mut tick_progress = 1.0 - update_timer.time / TICK_LEN_SECS;
        if dont_interpolate {
            tick_progress = 1.0;
        }
        draw(&game_state, &global_state, tick_progress);

        next_frame().await; // forced vsync :/
    }
}

fn update(game: &mut GameState, global_state: &mut GlobalState) {
    let player = &mut game.current_tick_mut().player;
    let center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
    player
        .handle_movement(&global_state.input_buffer, PLAYER_ACCEL)
        // .movement
        // .limit_speed(PLAYER_MAX_SPEED)
        ;

    if is_key_down(KeyCode::Space) {
        player.movement = Movement::default();
        player.movement.pos = center;
    }

    player.movement.step(PLAYER_MAX_SPEED);
}

// global state only needed for debug stuff rn
fn draw(game: &GameState, global_state: &GlobalState, t: f32) {
    let current = game.current_tick();
    let prev = game.prev_tick();
    clear_background(GRAY);
    current.player.draw(&prev.player, t);
    // game.player.draw(&prev.player, 0.25);
    // game.player.draw(&prev.player, 0.5);
    // game.player.draw(&prev.player, 0.75);
    current.player.draw_dbg_prev(&prev.player, t);
    current.player.draw_dbg(&prev.player, t);
    let real_time_avg_dir = interp_test::average_input(&global_state.input_buffer);
    let interped_pos = prev.player.movement.interp(&current.player.movement, t);
    dbg_arrow(interped_pos, real_time_avg_dir * 50.0, DBG_INTERP);
    dbg_arrow(interped_pos, current.player.movement.accel, DBG_PREV);
    dbg_arrow(interped_pos, current.player.movement.vel, DBG_NOW);

    // dbg tick buffer
    for i in (0..TICK_BUFFER_LEN - 1).rev() {
        let prev = &game.get_prev_tick(i + 1).unwrap().player;
        let next = &game.get_prev_tick(i).unwrap().player;
        next.draw(prev, t);
    }
}

fn handle_inputs(input_buffer: &mut Vec<Vec2>) {
    // TODO: possibly limit to like 10,000 or 80,000 or smth
    // incase mega input
    input_buffer.push(Player::desired_dir());
}
