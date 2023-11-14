mod state;

use crate::state::{GameState, GlobalState, TickSettings};
use interp_test::movement::Movement;
use interp_test::time::Timer;
use interp_test::{dbg_arrow, Player, DBG_INTERP, DBG_NOW, DBG_PREV};
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use macroquad::window::{screen_height, screen_width};

// const UPDATES_PER_SECOND: f32 = 10.0;
// const UPDATES_PER_SECOND: f32 = 15.0;
// const TICK_LEN_SECS: f32 = 1.0 / UPDATES_PER_SECOND * 10.0;
// const UPDATES_PER_SECOND: f32 = 30.0;
// const TICK_LEN_SECS: f32 = 1.0 / UPDATES_PER_SECOND * 5.0;
// const TICK_BUFFER_SECS: f32 = 0.25;
// const TICK_BUFFER_LEN: usize = (TICK_BUFFER_SECS * UPDATES_PER_SECOND + 0.5) as usize; // + 0.5 as ceil is not const (wtf)

// /// TODO:DOCS:
// const SPEED_FACTOR: f32 = 60.0 / UPDATES_PER_SECOND;
// const PLAYER_MAX_SPEED: f32 = 20.0 * SPEED_FACTOR * 1.0;
// const PLAYER_ACCEL: f32 = 5.0 * SPEED_FACTOR * SPEED_FACTOR;

#[macroquad::main("interp test")]
async fn main() {
    // let mut tick_settings = TickSettings::default().tps(30.0).calculate().unwrap();

    let mut global_state = GlobalState::new(30.0).expect("tps too low");
    let mut game_state = GameState::new(global_state.tick_settings.buffer_len);
    // game_state.tick_number = 2usize.pow(48);
    // game_state.tick_number = (2i128.pow(64) - 100) as usize;
    // game_state.tick_number = 0;
    game_state.init();
    dbg!(game_state.tick_state.len());
    // let mut prev_game_state = game_state.clone();
    // let game = &mut game_state;
    // let mut prev_game = &mut prev_game_state;
    let mut update_timer = Timer::new(global_state.tick_settings.tick_len_secs);
    let mut dont_interpolate = false;
    let mut is_fullscreen = false;
    // dbg!(SPEED_FACTOR);
    loop {
        // game_state.player.handle_movement(PLAYER_MAX_SPEED);

        let delta_time = get_frame_time();
        let ready_to_update = update_timer.decrement(delta_time);
        // HACK: prevent mega extrapolating when tps > fps
        // currently: if fps > tps, tps = fps
        // TODO: render thread
        update_timer.decrement(0.0);
        update_timer.decrement(0.0);
        update_timer.decrement(0.0);
        update_timer.decrement(0.0);
        update_timer.decrement(0.0);
        update_timer.decrement(0.0);
        update_timer.decrement(0.0);

        // close game
        if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl))
            && (is_key_down(KeyCode::C) || is_key_down(KeyCode::Q))
            || is_key_down(KeyCode::Escape)
        {
            break;
        }

        // NOTE: unfullscreening is broken on x11 in macroquad (wtf?), also fullscreening loses
        // focus because window hides for 1 micro second
        // NOTE: fullscreen keybind in Kwin works to unfullscreen tho
        if (is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt))
            && is_key_pressed(KeyCode::Enter)
        {
            is_fullscreen = !is_fullscreen;
            set_fullscreen(is_fullscreen);
        }

        if is_key_pressed(KeyCode::F3) || is_key_pressed(KeyCode::KpSubtract) {
            let tick_settings = &mut global_state.tick_settings;
            let mut timescale = tick_settings.timescale() / 1.1;
            if (timescale - 1.0).abs() < 0.05 {
                timescale = 1.0;
            }
            tick_settings.set_timescale(timescale);
            update_timer.start_time = tick_settings.tick_len_secs;
        }

        if is_key_pressed(KeyCode::F4) || is_key_pressed(KeyCode::KpAdd) {
            let tick_settings = &mut global_state.tick_settings;
            let mut timescale = tick_settings.timescale() * 1.1;
            if (timescale - 1.0).abs() < 0.05 {
                timescale = 1.0;
            }
            tick_settings.set_timescale(timescale);
            update_timer.start_time = tick_settings.tick_len_secs;
        }

        // Input and Update
        handle_inputs(&mut global_state.input_buffer); // as close to update as possible
        if ready_to_update {
            // prev_game_state = game_state.clone();
            game_state.advance_tick();
            update(&mut game_state, &mut global_state);
            global_state.input_buffer.clear()
        }

        if is_key_pressed(KeyCode::I) {
            dont_interpolate = !dont_interpolate;
        }

        // Drawing
        let mut tick_progress = 1.0 - update_timer.time / global_state.tick_settings.tick_len_secs;
        if dont_interpolate {
            tick_progress = 1.0;
        }

        draw(&game_state, &global_state, tick_progress);

        next_frame().await; // forced vsync :/ disable on Linux with `vblank_mode=0 cargo run`
    }
}

fn update(game: &mut GameState, global_state: &mut GlobalState) {
    let speed_factor = global_state.tick_settings.speed_factor;
    let player = &mut game.current_tick_mut().player;
    let center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
    player
        .handle_movement(&global_state.input_buffer, Player::accel(speed_factor))
        // .movement
        // .limit_speed(PLAYER_MAX_SPEED)
        ;

    if is_key_down(KeyCode::Space) {
        player.movement = Movement::default();
        player.movement.pos = center;
    }

    player.movement.step(Player::max_speed(speed_factor));
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
    for i in (0..global_state.tick_settings.buffer_len - 1).rev() {
        let prev = &game.get_prev_tick(i + 1).unwrap().player;
        let next = &game.get_prev_tick(i).unwrap().player;
        next.draw(prev, t);
    }

    dbg_info(game, global_state, t);
}

fn handle_inputs(input_buffer: &mut Vec<Vec2>) {
    // TODO: possibly limit to like 10,000 or 80,000 or smth
    // incase lag or mega input
    input_buffer.push(Player::desired_dir());
}

// ping
// lerp
// #[no_mangle]
const TYPEFACE_SIZE: f32 = 15.0;

#[rustfmt::skip]
fn dbg_info(game: &GameState, global_state: &GlobalState, t: f32) {
    // fps, frametime
    // tick, gametime
    // tps, tick time
    // buffer len, time, time
    // timescale, speed factor, reference tps

    
    // t
    // fps limit
    // tick time, draw time, loop time
    // pos, dir
    // ping, lerp

    // fps, avg min max last, graphs
    // resolution, zoom, vram, driver, gpu, cpu name, ram usage, vram usage
    // game version
    // rust version // maybe only for debug builds, want to give hackers as little info as possible
    // profile
    // opt-level
    // debug-assertions
    // DEBUG =?
    // host
    // target
    // features -> hard
    // comp
    
    fn format_days_hms(seconds: usize) -> String {
        let days = seconds / 86400;
        let hours = seconds / 3600 % 24;
        let min = seconds / 60 % 60;
        let secs = seconds % 60;
        if days != 0 {
            format!("{} days {}:{}:{}", days, hours, min, secs)
        } else {
            format!("{}:{}:{}", hours, min, secs)
        }
    }

    let mut y = 0.0;
    
    let tick_settings = &global_state.tick_settings;
    let TickSettings { tps, tick_len_secs, buffer_secs, buffer_len, speed_factor } = tick_settings;
    let tps = *tps;
    
    dbg_info_version(&mut y);
    let mut next_line = || { y += TYPEFACE_SIZE; y };

    let width = screen_width() as i32;
    let height = screen_height() as i32;
    // let width = ref
    // draw_text(&format!("{}x{}", width, height), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    draw_text(&to_string(dbg_resolution), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    draw_text(&format!("{:0>3} fps {:.8}ms", get_fps(), get_frame_time() * 1000.0), 0.0, next_line(), TYPEFACE_SIZE, WHITE);


    let tick = game.tick_number;
    let (gametime_s, gametime_ticks) = game.gametime_passed(tps);
    let game_time_frac = gametime_ticks as f64 / tps as f64;
    let with_zero = &format!("{:.15}", game_time_frac); // NOTE: 15 is f64::DIGITS
    let gametime_frac = with_zero.strip_prefix('0').unwrap_or_default();
    let gametime_whole = format_days_hms(gametime_s); // correct
    // draw_text(&format!("tick: {:06}, gametime: {}+{:02} or {}{}s", tick, gametime_whole, gametime_ticks, gametime_s, gametime_frac), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    draw_text(&format!("tick: {:06}, gametime: {}+{:02}", tick, gametime_whole, gametime_ticks), 0.0, next_line(), TYPEFACE_SIZE, WHITE);

    draw_text(&format!("set: tps: {}, tick time: {}s", tps, tick_len_secs), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    draw_text(&format!(" buffer: len: {}, time: {}s, actual time: {}s", buffer_len, buffer_secs, tick_len_secs * *buffer_len as f32), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    let time_scale = tick_settings.timescale();
    draw_text(&format!(" timescale: {}, speed factor: {}, reference tps: {}", time_scale, speed_factor, TickSettings::REFERENCE_TPS), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    // draw_text(&format!(""), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    // draw_text(&format!("version: {}", env!("CARGO_PKG_VERSION")), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    // draw_text(&format!(" timescale: {}, speed factor: {}, reference tps: {}", time_scale, speed_factor, TickSettings::REFERENCE_TPS), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
}

/// Convert an allocationless formater to a string, ignoring all errors.
fn to_string<F, E>(mut f: F) -> String
where
    F: FnMut(&mut dyn Write) -> Result<(), E>,
{
    let mut buffer = String::new();
    _ = f(&mut buffer);
    buffer
}

/// Convert an allocationless formater to a string.
fn try_to_string<F, E>(mut f: F) -> Result<String, E>
where
    F: FnMut(&mut dyn Write) -> Result<(), E>,
{
    let mut buffer = String::new();
    f(&mut buffer).map(|_| buffer)
}

use std::fmt::Write;
fn dbg_resolution(write: &mut dyn Write) -> Result<(), std::fmt::Error> {
    let width = screen_width() as i32;
    let height = screen_height() as i32;
    writeln!(write, "{}x{}", width, height)
}

fn dbg_info_version(y: &mut f32) {
    // see build.rs for these cfgs
    // something like: "0.1.0"
    #[cfg(build = "release")]
    let version = concat!(env!("CARGO_PKG_VERSION"));
    // something like: "0.1.0-e0a5e97-dirty"
    #[cfg(build = "debug")]
    let version = concat!(
        env!("CARGO_PKG_VERSION"),
        "-",
        git_tag::git_tag!("--dirty=-dirty")
    );
    let name = env!("CARGO_CRATE_NAME");
    let profile = env!("PROFILE");
    // let host = env!("HOST"); // no one cares
    let target = env!("TARGET");
    let datetime = env!("BUILD_DATETIME");
    // dbg!(std::env!("TARGET"));
    // dbg!(std::env::var("PROFILE"));
    // dbg!(std::env::var("TARGET"));
    // game version
    // rust version // maybe only for debug builds, want to give hackers as little info as possible
    // profile
    // opt-level
    // debug-assertions
    // DEBUG =?
    // host
    // target
    // features -> hard
    // comp
    let mut next_line = || {
        *y += TYPEFACE_SIZE;
        *y
    };

    // let version = git_tag::git_tag!(); // actually works and puts "-modified" at the end
    draw_text(
        &format!("{name} {version} {profile} {target}, compiled {datetime}",),
        0.0,
        next_line(),
        TYPEFACE_SIZE,
        WHITE,
    );
}
