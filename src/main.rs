mod state;

use crate::state::{GameState, GlobalState};
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
