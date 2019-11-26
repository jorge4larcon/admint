// Author: Jorge Alarcon Alvarez
// Email:  jorge4larcon@gmail.com
// This module is used to communicate the different modules of the program and to run the user commands.

extern crate clap;
extern crate log;
pub mod config;
pub mod ipparser;
pub mod commands;
pub mod clients;
use std::process;

pub fn run(command: commands::BaseCommand) {    
    if let Ok(()) = config::setup_logging(&log::LevelFilter::Warn) {
        log::debug!("Logging was set up");
        println!("{}", command.send_and_interpret());
    } else {
        eprintln!("Could not set up logging");
        process::exit(1);
    }
}
