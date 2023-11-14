//! abc

extern crate proc_macro;

use std::process::Command;
use proc_macro::TokenStream;

// TODO: generic compile time run command macro
// TODO: generic get env var macro (build env var)

/// Simply runs `git describe --always --dirty=-modified` to get the latest git tag,
/// and returns a string literal. Appends "-modified" to the end if there are uncommited changes.
/// Any arguments supplied will be appended to the command arguments.
#[proc_macro]
pub fn git_tag(token_stream: TokenStream) -> TokenStream {
    // "fn answer() -> u32 { 42 }".parse().unwrap()
    // let extra_args = _item.into_iter().map(|tt| ).collect::<Vec<_>>();
    let mut extra_args = Vec::new();
    for token_tree in token_stream {
        let str = token_tree.to_string();
        let str = str.strip_prefix('\"').expect("argument must be a string literal").strip_suffix('\"').expect("argument must be a string literal");
        extra_args.push(str.to_string());
    }

    let mut command = Command::new("git");
    let args = ["describe", "--always", "--dirty=-modified"];
    let mut all_args = args.to_vec();
    all_args.extend(extra_args.iter().map(|s| s.as_str()));
    command.args(&all_args);

    let args_str = all_args.join(" ");

    let output = command.output().unwrap_or_else(|_| panic!("Error running command: git {}", args_str));
    let exit_status = &output.status;
    if !exit_status.success() {
        panic!("git failed with error code: {}. command: git {}", exit_status, args_str)
    }
        let bytes = output.stdout;
        let stdout = String::from_utf8_lossy(&bytes);
        let stdout = stdout.trim(); // has to be here in particular or no effect for reasons???
        format!("\"{}\"", stdout).parse().unwrap()
}