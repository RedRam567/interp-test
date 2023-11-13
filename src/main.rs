use interp_test::time::Timer;
use interp_test::{Player, DBG_INTERP, DBG_PREV, DBG_NOW, dbg_arrow};
use interp_test::movement::Movement;
use macroquad::prelude::*;
use macroquad::window::{screen_height, screen_width};

// const UPDATES_PER_SECOND: f32 = 10.0;
// const UPDATES_PER_SECOND: f32 = 15.0;
// const TICK_LEN_SECS: f32 = 1.0 / UPDATES_PER_SECOND * 10.0;
const UPDATES_PER_SECOND: f32 = 30.0;
const TICK_LEN_SECS: f32 = 1.0 / UPDATES_PER_SECOND * 10.5;

/// TODO:DOCS:
const SPEED_FACTOR: f32 = 60.0 / UPDATES_PER_SECOND;
const PLAYER_MAX_SPEED: f32 = 20.0 * SPEED_FACTOR * 1.0;
const PLAYER_ACCEL: f32 = 5.0 * SPEED_FACTOR * SPEED_FACTOR;

#[derive(Clone, Debug, PartialEq, Default)]
struct GameState {
    player: Player,
    /// Store input as fast as possible here until `update()`
    input_buffer: Vec<Vec2>,
}

impl GameState {
    fn new() -> Self {
        Self::default()
    }
    fn init(&mut self) -> &mut Self {
        self.player.movement.pos.x = screen_width() / 2.0;
        self.player.movement.pos.y = screen_height() / 2.0;
        self
    }
}

#[macroquad::main("interp test")]
async fn main() {
    let mut game_state = GameState::new();
    game_state.init();
    let mut prev_game_state = game_state.clone();
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
        handle_inputs(&mut game_state.input_buffer); // as close to update as possible
        if ready_to_update {
            prev_game_state = game_state.clone();
            update(&mut game_state);
            game_state.input_buffer.clear()
        }

        if is_key_pressed(KeyCode::Q) {
            dont_interpolate = !dont_interpolate;
        }

        // Drawing
        let mut tick_progress = 1.0 - update_timer.time / TICK_LEN_SECS;
        if dont_interpolate {
            tick_progress = 1.0;
        }
        draw(&game_state, &prev_game_state, tick_progress);

        next_frame().await; // forced vsync :/
    }
}

fn update(game: &mut GameState) {
    let player = &mut game.player;
    let center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
    player
        .handle_movement(&game.input_buffer, PLAYER_ACCEL)
        // .movement
        // .limit_speed(PLAYER_MAX_SPEED)
        ;

    dbg!(player.movement.vel.length());
    dbg!(player.movement.accel.length());

    if is_key_down(KeyCode::Space) {
        player.movement = Movement::default();
        player.movement.pos = center;
    }

    player.movement.step(PLAYER_MAX_SPEED);
}

fn draw(game: &GameState, prev: &GameState, t: f32) {
    clear_background(GRAY);
    game.player.draw(&prev.player, t);
    // game.player.draw(&prev.player, 0.25);
    // game.player.draw(&prev.player, 0.5);
    // game.player.draw(&prev.player, 0.75);
    game.player.draw_dbg_prev(&prev.player, t);
    game.player.draw_dbg(&prev.player, t);
    let real_time_avg_dir = interp_test::average_input(&game.input_buffer);
    let interpolated_pos = prev.player.movement.interp(&game.player.movement, t);
    dbg_arrow(interpolated_pos, real_time_avg_dir * 50.0, DBG_INTERP);
    dbg_arrow(interpolated_pos, game.player.movement.accel, DBG_PREV);
    dbg_arrow(interpolated_pos, game.player.movement.vel, DBG_NOW);
}

fn handle_inputs(input_buffer: &mut Vec<Vec2>) {
    // TODO: possibly limit to like 10,000 or 80,000 or smth
    // incase mega input
    input_buffer.push(Player::desired_dir());
}