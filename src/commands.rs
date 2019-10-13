extern crate clap;

use std::net;
use crate::ipparser;


pub enum Set {
    DropVotes(u8),
    DropVerification(bool),
    ListSize(u16),
    Capacity(u16),
    Password(String),
    Key(String)
}

pub enum Get {
    Mac(ipparser::MacAddress),
    Username { pattern: String, start_index: usize },
    Index { start_index: usize, end_index: usize },
    RunningConfiguration
}

pub enum Drop {
    Ip(net::Ipv4Addr)
}

pub enum Command {
    Set(Set),
    Get(Get),
    Drop(Drop)
}

pub struct BaseCommand {
    password: String,
    address: net::SocketAddrV4,
    command: Command
}

fn password_and_address(matches:: &clap::ArgMatches) -> Option<(String, net::SocketAddrV4)> {

}

impl BaseCommand {
    pub fn from_clap_matches(matches: &clap::ArgMatches) /*-> Option<BaseCommand>*/ {
        if let Some(subcommand_matches) = matches.subcommand_matches("running-config") {
            if let Some((password, address)) = password_and_address(subcommand_matches) {

            }
        } else if let Some(subcommand_matches) = matches.subcommand_matches("set-dropvotes") {
            
        }

        if let Some(subcommand_matches) = matches.subcommand_matches("set-dropverification") {
            
        }

        if let Some(subcommand_matches) = matches.subcommand_matches("set-listsize") {
            
        }

        if let Some(subcommand_matches) = matches.subcommand_matches("set-capacity") {
            
        }

        if let Some(subcommand_matches) = matches.subcommand_matches("set-key") {
            
        }

        if let Some(subcommand_matches) = matches.subcommand_matches("drop") {
            
        }

        if let Some(subcommand_matches) = matches.subcommand_matches("get-mac") {
            
        }

        if let Some(subcommand_matches) = matches.subcommand_matches("get-username") {
            
        }

        if let Some(subcommand_matches) = matches.subcommand_matches("get-index") {
            
        }    
    }
}
