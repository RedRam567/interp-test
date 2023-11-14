use macroquad::prelude::*;
use crate::state::{GameState, GlobalState, TickSettings};
use std::fmt::Write;

const TYPEFACE_SIZE: f32 = 15.0;

/// Convert an allocation-less formater to a string, ignoring all errors.
fn to_string<F, E>(mut f: F) -> String
where
    F: FnMut(&mut dyn Write) -> Result<(), E>,
{
    let mut buffer = String::new();
    _ = f(&mut buffer);
    buffer
}

/// Convert an allocation-less formater to a string.
fn try_to_string<F, E>(mut f: F) -> Result<String, E>
where
    F: FnMut(&mut dyn Write) -> Result<(), E>,
{
    let mut buffer = String::new();
    f(&mut buffer).map(|_| buffer)
}

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


// ping
// lerp
#[rustfmt::skip]
pub fn dbg_info(game: &GameState, global_state: &GlobalState, t: f32) {
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