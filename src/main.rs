use std::env;

mod config;

use config::Config;
use std::process::Command;

fn main() {
    let mut config = Config::default();
    let success;
    let mut status = "";
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.contains(&"new".to_string()) || args.contains(&"init".to_string()) {
        let mut command = Vec::new();
        for arg in &args {
            if arg != &"-f".to_string() && arg != &"-ff".to_string() && arg != &"-ffn".to_string() {
                command.push(arg.to_string());
            }
        }
        Command::new("cargo")
            .args(command)
            .output()
            .expect("Failed to execute command");
    }

    if args.contains(&"-h".to_string()) {
        println!("Truck --Version {}", env!("CARGO_PKG_VERSION"));
        println!("[Note] If you are a linux or mac user, please make sure zld(mac), lld(linux), clang and mold is installed.");
        println!("USAGE: truck [flags for truck].");
        println!("\tno flags: default configuration");
        println!("\t-f: Fast Linking");
        println!("\t-ff: Faster than -f");
        println!("\t-ffn: Fastest. Requires Rust Nightly");
    } else {
        if args.contains(&"-f".to_string()) {
            success = config.fast(args.get(2));
            status = "[Use with caution] Make sure you have lld(linux) or zld(mac) installed";
        } else if args.contains(&"-ff".to_owned()) {
            success = config.faster(args.get(2));
            status = "[Use with caution] Make sure You have mold(if linux or mac using package manager) installed";
        } else if args.contains(&"-ffn".to_string()) {
            success = config.faster_nightly(args.get(2));
            status = "[Use with caution] Make sure You have rust-nightly and mold(if linux or mac using package manager) installed";
        } else {
            success = config.def(args.get(2));
        }
        if success {
            println!("Initialised Successfully");
            println!("{}", status);
        } else {
            println!("You discovered a bug. Please Report");
        }
    }
}
