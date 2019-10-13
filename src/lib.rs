extern crate clap;
extern crate log;
pub mod config;
pub mod ipparser;
pub mod commands;
use std::process;

pub fn run(command: commands::BaseCommand) {
    if let Ok(()) = config::setup_logging(&log::LevelFilter::Debug) {
        log::debug!("Logging was set up");
        println!("{}", command.send_and_interpret());
    } else {
        eprintln!("Could not set up logging");
        process::exit(1);
    }
}
