mod dbg;

use std::time::Instant;

use interp_test::movement::Movement;
use interp_test::state::{GameState, GlobalState};
use interp_test::time::Timings;
use interp_test::{dbg_arrow, player::Player, DBG_INTERP, DBG_NOW, DBG_PREV};
use macroquad::prelude::*;
use macroquad::window::{screen_height, screen_width};

#[macroquad::main("interp test")]
async fn main() {
    let mut global_state = GlobalState::new(30.0).unwrap();
    let mut game_state = GameState::new(global_state.tick_settings.buffer_len);
    game_state.init();

    #[allow(clippy::field_reassign_with_default)] // to match other all other uses if Timings
    loop {
        // need a local version else timings after draw are all wrong, because async?
        let mut current_timings = Timings::default();
        current_timings.start = Some(Instant::now());

        let delta_time = get_frame_time();
        let ready_to_update = global_state.update_timer.decrement(delta_time);
        // HACK: prevents mega extrapolating when tps > fps
        // currently: if fps > tps, tps = fps
        // TODO: render thread
        global_state.update_timer.decrement(0.0);
        global_state.update_timer.decrement(0.0);
        global_state.update_timer.decrement(0.0);
        global_state.update_timer.decrement(0.0);
        global_state.update_timer.decrement(0.0);
        global_state.update_timer.decrement(0.0);
        global_state.update_timer.decrement(0.0);

        // Input handling
        // HACK: ugly bool
        let close = pre_update(&mut game_state, &mut global_state);
        if close {
            break;
        }
        current_timings.pre_update = Some(Instant::now());

        // Update
        if ready_to_update {
            update(&mut game_state, &global_state);
            global_state.input_buffer.clear()
        }
        current_timings.update = Some(Instant::now());

        // Drawing
        let mut tick_progress = global_state.tick_progress();
        if global_state.dont_interpolate {
            tick_progress = 1.0;
        }
        draw(&game_state, &global_state, tick_progress);
        current_timings.draw = Some(Instant::now());

        // has forced vsync :/ disable on Linux with `vblank_mode=0 cargo run`
        next_frame().await;
        current_timings.waiting = Some(Instant::now());
        global_state.timings = current_timings;
    }
}

fn pre_update(game: &mut GameState, global_state: &mut GlobalState) -> bool {
    // close game
    if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl))
        && (is_key_down(KeyCode::C) || is_key_down(KeyCode::Q))
        || is_key_down(KeyCode::Escape)
    {
        return true;
    }

    // fullscreening
    // NOTE: unfullscreening is broken on x11 in macroquad (wtf?), also fullscreening loses
    // focus because window hides for 1 micro second
    // NOTE: fullscreen keybind in Kwin works to unfullscreen tho
    if (is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt))
        && is_key_pressed(KeyCode::Enter)
    {
        global_state.is_fullscreen = !global_state.is_fullscreen;
        set_fullscreen(global_state.is_fullscreen);
    }

    // Modify timescale
    if is_key_pressed(KeyCode::F1) || is_key_pressed(KeyCode::KpSubtract) {
        let mut timescale = global_state.tick_settings.timescale() / 1.25;
        if (timescale - 1.0).abs() < 0.05 {
            timescale = 1.0;
        }
        global_state.set_timescale(timescale);
    }

    if is_key_pressed(KeyCode::F2) || is_key_pressed(KeyCode::KpAdd) {
        let mut timescale = global_state.tick_settings.timescale() * 1.25;
        if (timescale - 1.0).abs() < 0.05 {
            timescale = 1.0;
        }
        global_state.set_timescale(timescale);
    }

    // Modify tps
    if is_key_pressed(KeyCode::F3) {
        _ = global_state.set_tps(game, global_state.tick_settings.tps - 5.0);
    }

    if is_key_pressed(KeyCode::F4) {
        _ = global_state.set_tps(game, global_state.tick_settings.tps + 5.0);
    }

    if is_key_pressed(KeyCode::F5) {
        _ = global_state.set_buffer_secs(game, global_state.tick_settings.buffer_secs - 0.05);
    }

    if is_key_pressed(KeyCode::F6) {
        _ = global_state.set_buffer_secs(game, global_state.tick_settings.buffer_secs + 0.05);
    }

    // FIXME: spamming eats all ram (how??)
    // if is_key_pressed(KeyCode::T) {
    //     // *global_state = GlobalState::new(30.0).unwrap();
    //     game.init();
    // }

    // Interp stuff
    if is_key_pressed(KeyCode::R) {
        // reset
        *global_state = GlobalState::new(30.0).unwrap();
    }
    if is_key_pressed(KeyCode::U) {
        global_state.dbg_hide_interp_info = !global_state.dbg_hide_interp_info;
    }
    if is_key_pressed(KeyCode::I) {
        global_state.dont_interpolate = !global_state.dont_interpolate;
    }
    if is_key_pressed(KeyCode::O) {
        global_state.dbg_buffer = !global_state.dbg_buffer;
    }
    if is_key_pressed(KeyCode::P) {
        use interp_test::player::AveragingStrategy;
        global_state.avg_strategy = match global_state.avg_strategy {
            AveragingStrategy::Oldest => AveragingStrategy::Newest,
            AveragingStrategy::Newest => AveragingStrategy::Mean,
            AveragingStrategy::Mean => AveragingStrategy::MeanIgnoreZero,
            AveragingStrategy::MeanIgnoreZero => AveragingStrategy::MeanNormalized,
            AveragingStrategy::MeanNormalized => AveragingStrategy::Oldest,
            _ => AveragingStrategy::Oldest,
        }
    }

    handle_inputs(&mut global_state.input_buffer); // as close to update as possible

    false
}

fn update(game: &mut GameState, global_state: &GlobalState) {
    game.advance_tick();
    let speed_factor = global_state.tick_settings.speed_factor;
    let player = &mut game.current_tick_mut().player;
    let center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
    let wish_dir = global_state.avg_strategy.average(&global_state.input_buffer);
    player.handle_movement(wish_dir, Player::accel(speed_factor));

    if is_key_down(KeyCode::Space) {
        player.movement = Movement::default();
        player.movement.pos = center;
    }

    player.movement.step(
        Player::max_speed(speed_factor),
        Player::base_friction(speed_factor),
        Player::scaling_friction(speed_factor),
    );
}

// global state only needed for debug stuff rn
fn draw(game: &GameState, global_state: &GlobalState, t: f32) {
    let current = game.current_tick();
    let prev = game.prev_tick();
    clear_background(GRAY);
    current.player.draw(&prev.player, t);

    if !global_state.dbg_hide_interp_info {
        current.player.draw_dbg_prev(&prev.player, t);
        current.player.draw_dbg(&prev.player, t);

        let realtime_wish_dir = global_state.avg_strategy.average(&global_state.input_buffer);

        let interped_pos = prev.player.movement.interp(&current.player.movement, t);
        dbg_arrow(interped_pos, realtime_wish_dir * 50.0, DBG_INTERP);
        dbg_arrow(interped_pos, current.player.movement.accel, DBG_PREV);
        dbg_arrow(interped_pos, current.player.movement.vel, DBG_NOW);
    }

    // FIXME: increasing tps during runtime past initial value crashes
    // dbg tick buffer
    if global_state.dbg_buffer {
        for i in (0..global_state.tick_settings.buffer_len - 1).rev() {
            let buffer = &game.buffer;
            let prev = &buffer.get_back(i + 1).unwrap().player;
            let next = &buffer.get_back(i).unwrap().player;
            next.draw(prev, t);
        }
    }

    dbg::dbg_info(game, global_state, t);
}

fn handle_inputs(input_buffer: &mut Vec<Vec2>) {
    // TODO: possibly limit to like 10,000 or 80,000 or smth
    // incase lag or mega input
    input_buffer.push(Player::desired_dir());
}
