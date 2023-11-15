use crate::state::{GameState, GlobalState, TickSettings};
use macroquad::prelude::*;
use std::fmt::{Display, Error as FmtError, Write};

const TYPEFACE_SIZE: f32 = 15.0;
use const_format::formatcp;

// type Writer<'a> = &'a mut dyn Write;

// TODO: wine host os detection (for analytics)

/// Convert an allocation-less formater fn to a string, ignoring all errors.
fn to_string<F, E>(mut f: F) -> String
where
    F: FnMut(&mut dyn Write) -> Result<(), E>,
{
    let mut buffer = String::new();
    _ = f(&mut buffer);
    buffer
}

/// Convert an allocation-less formater fn to a string.
fn try_to_string<F, E>(mut f: F) -> Result<String, E>
where
    F: FnMut(&mut dyn Write) -> Result<(), E>,
{
    let mut buffer = String::new();
    f(&mut buffer).map(|_| buffer)
}

// NOTE: see build.rs for these cfgs and env vars
// formatcp and w.write_str reduced asm instructions from 44 to 4
fn dbg_version(w: &mut dyn Write) -> Result<(), FmtError> {
    // Hide git tag on release
    #[cfg(build = "release")] // "0.1.0"
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    #[cfg(build = "debug")] // "0.1.0-e0a5e97-dirty"
    const VERSION: &str =
        formatcp!("{}-{}", env!("CARGO_PKG_VERSION"), git_tag::git_tag!("--dirty=-dirty"));

    const NAME: &str = env!("CARGO_CRATE_NAME");
    #[cfg(all(build = "release", debug_assertions))]
    const PROFILE: &str = formatcp!("{}+debug assertions", env!("PROFILE"));
    #[cfg(not(all(build = "release", debug_assertions)))]
    const PROFILE: &str = env!("PROFILE");

    const TARGET: &str = env!("TARGET");
    const DATETIME: &str = env!("BUILD_DATETIME");

    w.write_str(formatcp!("{} {} {}, {}, compiled {}", NAME, VERSION, PROFILE, TARGET, DATETIME))
}

#[cfg(build = "debug")]
fn dbg_version_line2(w: &mut dyn Write) -> Result<(), FmtError> {
    const HOST: &str = env!("HOST");
    const RUSTC_VERSION: &str = env!("BUILD_RUSTC_VERSION");

    w.write_str(formatcp!("build: {} {}", HOST, RUSTC_VERSION))
}

fn dbg_resolution(w: &mut dyn Write) -> Result<(), FmtError> {
    let width = screen_width() as i32;
    let height = screen_height() as i32;
    write!(w, "{}x{}", width, height)
}

fn dbg_fps(w: &mut dyn Write) -> Result<(), FmtError> {
    write!(w, "{:03} fps {:.8}ms", get_fps(), get_frame_time() * 1000.0)
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
struct DaysHms {
    pub secs: usize,
}
impl DaysHms {
    fn new(secs: usize) -> Self {
        Self { secs }
    }
}
impl Display for DaysHms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let secs = self.secs;
        let days = secs / 86400;
        let hours = secs / 3600 % 24;
        let min = secs / 60 % 60;
        let secs = secs % 60;

        if days != 0 {
            write!(f, "{} days ", days)?
        }
        write!(f, "{:02}:{:02}:{:02}", hours, min, secs)
    }
}

// ping
// lerp
#[rustfmt::skip]
pub fn dbg_info(game: &GameState, global_state: &GlobalState, t: f32) {
    // version
    // WINDOW RES inner resolution REFRESH LOGICAL RESOLUTION (scaling)
    // fps, frametime MIN MAX AVG VARIENCE
    // GRAPHS
    // tick, gametime
    // LOOP TIME, UPDATE TIME, DRAW TIME
    // net stuff:
    // tps, tick len, T
    // buffer len, time, time
    // timescale, speed factor, reference tps

    
    // fps limit
    // tick time, draw time, loop time
    // pos, dir, vel
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

    let mut y = 0.0;
    
    let tick_settings = &global_state.tick_settings;
    let TickSettings { tps, tick_len_secs, buffer_secs, buffer_len, speed_factor } = tick_settings;
    let tps = *tps;
    
    // dbg_version(&mut y);
    let mut next_line = || { y += TYPEFACE_SIZE; y };

    // let width = ref
    // draw_text(&format!("{}x{}", width, height), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    // draw_text(&format!("{:0>3} fps {:.8}ms", get_fps(), get_frame_time() * 1000.0), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    draw_text(&to_string(dbg_version), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    #[cfg(build = "debug")]
    draw_text(&to_string(dbg_version_line2), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    draw_text(&to_string(dbg_resolution), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    draw_text(&to_string(dbg_fps), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    
    let dbg_tick = |w: &mut dyn Write| {
        let tick = game.tick_number;
        let (gametime_s, gametime_ticks) = game.gametime_passed(tps);
        write!(w, "tick: {:06}, gametime: {}+{:02}", tick, DaysHms::new(gametime_s), gametime_ticks)
    };
    draw_text(&to_string(dbg_tick), 0.0, next_line(), TYPEFACE_SIZE, WHITE);

    // net stuff
    draw_text(&format!("set: tps: {}, tick time: {}s", tps, tick_len_secs), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    draw_text(&format!(" buffer: len: {}, time: {}s, actual time: {}s", buffer_len, buffer_secs, tick_len_secs * *buffer_len as f32), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    let time_scale = tick_settings.timescale();
    draw_text(&format!(" timescale: {}, speed factor: {}, reference tps: {}", time_scale, speed_factor, TickSettings::REFERENCE_TPS), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    // draw_text(&format!(""), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    // draw_text(&format!("version: {}", env!("CARGO_PKG_VERSION")), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
    // draw_text(&format!(" timescale: {}, speed factor: {}, reference tps: {}", time_scale, speed_factor, TickSettings::REFERENCE_TPS), 0.0, next_line(), TYPEFACE_SIZE, WHITE);
}
