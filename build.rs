//! Enables #[cfg(build = release)] for release and inheiriting profiles.
//! and #[cfg(build = debug)] for debug and inheiriting profiles.
//!
//! I wanted distinction over debug and debug assertions to give bad actors
//! as little info about the program as possible in release.
//!
//! Add this to your settings.json for better rust analyzer:
//! ```json
//! "rust-analyzer.cargo.cfgs": {
//!     "build": "debug"
//! }
//! ```
//!
//! See <https://users.rust-lang.org/t/conditional-compilation-for-debug-release/1098/6>
// TODO:DOCS: env vars

// TODO: where to even see this documentation? :p

use std::env;

fn main() {
    if let Ok(profile) = env::var("PROFILE") {
        if profile != "release" && profile != "debug" {
            eprintln!("Warning: weird value in profile");
        }
        println!("cargo:rustc-cfg=build={:?}", profile);
    } else {
        panic!("PROFILE env var not set");
    }

    // re-export build env vars to idk normal env vars
    println!("cargo:rustc-env=PROFILE={}", env::var("PROFILE").unwrap());
    println!("cargo:rustc-env=HOST={}", env::var("HOST").unwrap());
    // make a little bit prettier
    let target = env::var("TARGET").unwrap().to_string().replace("unknown-", "");
    println!("cargo:rustc-env=TARGET={}", target);

    // Chronos because Windows has the worst cli ever. Just run the `date` command and find out
    // NOTE: broo I just realized I had my Windows vm open and there was a freaking fullscreen
    // ad for edge. No respect.
    // Mon Y-M-D H:M:S -8:00 // as far as I can tell always English days of weeks
    let date_time = chrono::offset::Local::now().format("%a %Y-%m-%d %H:%M:%S %:z");
    println!("cargo:rustc-env=BUILD_DATETIME={}", date_time);
}

// fn cmd_output(mut command: Command) -> Result<String, &'static str> {
//     // let output = Command::new(cmd).args(args).output().map_err(|_| "Error launching command")?;
//     let output = command.output().map_err(|_| "Error launching command")?;
//     if !output.status.success() {
//         return Err("Command exited with error");
//     }
//     Ok(String::from_utf8_lossy(&output.stdout).into_owned())
// }
