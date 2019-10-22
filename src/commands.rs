extern crate serde_json;
extern crate log;
extern crate clap;
use std::net;
use crate::ipparser;
use crate::clients;
use std::time;
use std::io::{
    Read,
    Write
};

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

impl BaseCommand {
    pub fn from_clap_matches(matches: &clap::ArgMatches) -> Option<BaseCommand> {
        if let Some(subcommand_matches) = matches.subcommand_matches("running-config") {
            if let Some((password, address)) = password_and_address(subcommand_matches) {
                return Some( BaseCommand { password, address, command: Command::Get(Get::RunningConfiguration) } );
            }
        } else if let Some(subcommand_matches) = matches.subcommand_matches("set-dropvotes") {
            if let Some((password, address)) = password_and_address(subcommand_matches) {
                if let Some(drop_votes) = subcommand_matches.value_of("drop-votes") {
                    if let Ok(drop_votes) = drop_votes.parse::<u8>() {
                        if drop_votes > 0 {
                            return Some( BaseCommand { password, address, command: Command::Set(Set::DropVotes(drop_votes)) } );
                        }
                    }
                }
            }            
        } else if let Some(subcommand_matches) = matches.subcommand_matches("set-dropverification") {
            if let Some((password, address)) = password_and_address(subcommand_matches) {
                if let Some(state) = subcommand_matches.value_of("state") {
                    if let Ok(state) = state.parse::<bool>() {
                        return Some( BaseCommand { password, address, command: Command::Set(Set::DropVerification(state)) } );
                    }
                }
            }
        } else if let Some(subcommand_matches) = matches.subcommand_matches("set-listsize") {
            if let Some((password, address)) = password_and_address(subcommand_matches) {
                if let Some(list_size) = subcommand_matches.value_of("list-size") {
                    if let Ok(list_size) = list_size.parse::<u16>() {
                        return Some( BaseCommand { password, address, command: Command::Set(Set::ListSize(list_size)) } );
                    }
                }
            }
        } else if let Some(subcommand_matches) = matches.subcommand_matches("set-capacity") {
            if let Some((password, address)) = password_and_address(subcommand_matches) {
                if let Some(capacity) = subcommand_matches.value_of("capacity") {
                    if let Ok(capacity) = capacity.parse::<u16>() {
                        if capacity > 1 {
                            return Some( BaseCommand { password, address, command: Command::Set(Set::Capacity(capacity)) } );
                        }
                    }
                }
            }            
        } else if let Some(subcommand_matches) = matches.subcommand_matches("set-key") {
            if let Some((password, address)) = password_and_address(subcommand_matches) {
                if let Some(key) = subcommand_matches.value_of("key") {
                    if key.is_ascii() && key.len() < 33 {
                        return Some( BaseCommand { password, address, command: Command::Set(Set::Key(key.to_string())) } );
                    }
                }
            }
        } else if let Some(subcommand_matches) = matches.subcommand_matches("set-password") {
            if let Some((password, address)) = password_and_address(subcommand_matches) {
                if let Some(new_password) = subcommand_matches.value_of("password") {
                    if new_password.is_ascii() && new_password.len() < 33 {
                        return Some( BaseCommand { password, address, command: Command::Set(Set::Password(new_password.to_string())) } );
                    }
                }
            }
        } else if let Some(subcommand_matches) = matches.subcommand_matches("drop") {
            if let Some((password, address)) = password_and_address(subcommand_matches) {
                if let Some(ip) = subcommand_matches.value_of("ip") {
                    if let Some(ip) = ipparser::str_to_ipv4addr(ip) {
                        return Some( BaseCommand { password, address, command: Command::Drop(Drop::Ip(ip)) } );
                    }
                }
            }
        } else if let Some(subcommand_matches) = matches.subcommand_matches("get-mac") {
            if let Some((password, address)) = password_and_address(subcommand_matches) {
                if let Some(mac) = subcommand_matches.value_of("mac") {
                    if let Some(mac) = ipparser::MacAddress::new_from_str(mac) {
                        return Some( BaseCommand { password, address, command: Command::Get(Get::Mac(mac)) } );
                    }
                }
            }
        } else if let Some(subcommand_matches) = matches.subcommand_matches("get-username") {
            if let Some((password, address)) = password_and_address(subcommand_matches) {
                if let Some(pattern) = subcommand_matches.value_of("pattern") {
                    if pattern.is_ascii() {
                        if let Some(start_index) = subcommand_matches.value_of("start") {
                            if let Ok(start_index) = start_index.parse::<usize>() {
                                return Some( BaseCommand { password, address, command: Command::Get(Get::Username { pattern: pattern.to_string(), start_index } ) } );
                            }
                        }
                    }
                }
            }
        } else if let Some(subcommand_matches) = matches.subcommand_matches("get-index") {
            if let Some((password, address)) = password_and_address(subcommand_matches) {
                if let Some(start_index) = subcommand_matches.value_of("start") {
                    if let Ok(start_index) = start_index.parse::<usize>() {
                        if let Some(end_index) = subcommand_matches.value_of("end") {
                            if let Ok(end_index) = end_index.parse::<usize>() {
                                return Some( BaseCommand { password, address, command: Command::Get(Get::Index { start_index, end_index }) } );
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn to_json_string(&self) -> String {
        let mut json_string = format!("{{\"user\":\"admin\",\"password\":\"{}\",", self.password);
        match &self.command {
            Command::Set(set) => {
                json_string.push_str("\"method\":\"set\",");
                match set {
                    Set::Capacity(capacity) => {
                        json_string.push_str("\"what\":\"capacity\",");
                        json_string.push_str(&format!("\"capacity\":{}", capacity));
                    },
                    Set::DropVerification(dv) => {
                        json_string.push_str("\"what\":\"drop_verification\",");
                        json_string.push_str(&format!("\"drop_verification\":{}", dv));
                    },
                    Set::DropVotes(dv) => {
                        json_string.push_str("\"what\":\"drop_votes\",");
                        json_string.push_str(&format!("\"drop_votes\":{}", dv));
                    },
                    Set::Key(key) => {
                        json_string.push_str("\"what\":\"key\",");
                        json_string.push_str(&format!("\"key\":\"{}\"", key));
                    },
                    Set::ListSize(ls) => {
                        json_string.push_str("\"what\":\"list_size\",");
                        json_string.push_str(&format!("\"list_size\":{}", ls));
                    },
                    Set::Password(password) => {
                        json_string.push_str("\"what\":\"password\",");
                        json_string.push_str(&format!("\"new_password\":\"{}\"", password));
                    }
                }
            },
            Command::Get(get) => {
                json_string.push_str("\"method\":\"get\",");
                match get {
                    Get::Index { start_index, end_index } => {
                        json_string.push_str("\"how\":\"index\",");
                        json_string.push_str(&format!("\"start_index\":{},\"end_index\":{}", start_index, end_index));
                    },
                    Get::Mac(mac) => {
                        json_string.push_str("\"how\":\"mac\",");
                        json_string.push_str(&format!("\"mac\":\"{}\"", mac));
                    },
                    Get::RunningConfiguration => {
                        json_string.push_str("\"how\":\"running_configuration\"");
                    },
                    Get::Username { pattern, start_index } => {
                        json_string.push_str("\"how\":\"username\",");
                        json_string.push_str(&format!("\"username\":\"{}\",\"start_index\":{}", pattern, start_index));
                    }
                }
            },
            Command::Drop(drop) => {
                json_string.push_str("\"method\":\"drop\",");
                match drop {
                    Drop::Ip(ip) => {
                        json_string.push_str(&format!("\"ip\":\"{}\"", ip));
                    }
                }
            }
        }
        json_string.push('}');
        return json_string;
    }

    pub fn send_and_interpret(&self) -> String {
        let mut string = String::default();
        if let Some(answer) = self.send() {
            if let Ok(answer) = serde_json::from_str::<serde_json::Value>(&answer) {
                if let Some(result) = answer.get("result") {
                    if let Some(result) = result.as_str() {
                        string.push_str(&format!("result: {}", result));
                        match &self.command {
                            Command::Set(set) => {                                
                                match set {
                                    Set::Capacity(_) => { // Just returns result
                                    },
                                    Set::DropVerification(_) => { // Just returns result
                                    },
                                    Set::DropVotes(_) => {
                                        if let Some(dropped_clients) = answer.get("dropped_clients") {
                                            if let Some(dropped_clients) = dropped_clients.as_array() {
                                                string.push_str(&format!("\n{} dropped client(s):", dropped_clients.len()));
                                                for (index, dropped_client) in dropped_clients.iter().enumerate() {
                                                    if let Some(dropped_client) = clients::Client::from_json_value(dropped_client) {
                                                        string.push_str(&format!("\n[{}] {}", index, dropped_client));
                                                    } else {
                                                        string.push_str(&format!("\n[{}] {}", index, dropped_client));
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    Set::Key(_) => { // Just returns result                                    
                                    },
                                    Set::ListSize(_) => { // Just returns result
                                    },
                                    Set::Password(_) => { // Just returns result
                                    }
                                }
                            },
                            Command::Get(get) => {  
                                match get {
                                    Get::Index { start_index, end_index: _e } => {
                                        if let Some(clients) = answer.get("clients") {
                                            if let Some(clients) = clients.as_array() {
                                                string.push_str(&format!("\n{} client(s):", clients.len()));
                                                for (index, client) in clients.iter().enumerate() {
                                                    if let Some(client) = clients::Client::from_json_value(client) {
                                                        string.push_str(&format!("\n[{}] {}", start_index + index, client));
                                                    } else {
                                                        string.push_str(&format!("\n[{}] {}", start_index + index, client));
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    Get::Mac(_) => {
                                        if let Some(client) = answer.get("client") {
                                            if let Some(client) = clients::Client::from_json_value(client) {
                                                string.push_str(&format!("\n{}", client));
                                            } else {
                                                string.push_str(&format!("\n{}", client));
                                            }
                                        }
                                    },
                                    Get::RunningConfiguration => {
                                        if let Some(run_conf) = answer.get("running_config") {
                                            string.push_str(&format!("\n{}", run_conf.as_str().unwrap()));
                                        }
                                    },
                                    Get::Username { pattern: _p, start_index: _s } => {
                                        if let Some(clients) = answer.get("clients") {
                                            if let Some(clients) = clients.as_array() {
                                                string.push_str(&format!("\n{} client(s):", clients.len()));
                                                for (index, client) in clients.iter().enumerate() {
                                                    if let Some(client) = clients::Client::from_json_value(client) {
                                                        string.push_str(&format!("\n[{}] {}", index, client));
                                                    } else {
                                                        string.push_str(&format!("\n[{}] {}", index, client));
                                                    }
                                                }
                                            }
                                        }
                                        if let Some(end_index) = answer.get("end_index") {
                                            string.push_str(&format!("\nend index: {}", end_index));
                                        }
                                    }
                                }
                            },
                            Command::Drop(drop) => {
                                match drop {
                                    Drop::Ip(_) => {
                                    }
                                }
                            }
                        }                        
                    } else {
                        return format!("Could not parse the reply of {} as a valid MINT server reply, raw reply:\n{}", self.address, answer);
                    }
                } else if let Some(error) = answer.get("error") {
                    if let Some(error) = error.as_u64() {
                        string.push_str(&format!("error code: {}", error));
                        if let Some(name) = answer.get("name") {
                            if let Some(name) = name.as_str() {
                                string.push_str(&format!("\nname: {}", name));
                            } else {
                                string.push_str("\nUnparsable name");
                            }
                        } else {
                            string.push_str("\nUnparsable name");
                        }
                    } else {
                        string.push_str(&format!("Could not parse the reply of {} as a valid MINT server reply, raw reply:\n{}", self.address, answer));
                    }
                }
            } else {
                string.push_str(&format!("Could not parse the reply of {}, raw reply:\n{}", self.address, answer));
            }
        } else {
            string.push_str(&format!("No answer from {}", self.address));
        }
        return string;
    }

    fn send(&self) -> Option<String> {
        log::debug!("Connecting with {} ...", self.address);
        if let Ok(mut client) = net::TcpStream::connect(self.address) {
            log::debug!("Connection established with {}", self.address);
            let request = self.to_json_string();
            log::debug!("request:\n{}", request);
            if let Ok(bytes_written) = client.write(request.as_bytes()) {
                if bytes_written == request.as_bytes().len() {
                    log::info!("The request was sent succesfully [{} byte(s)]", bytes_written);
                    let mut buffer = [0; 65535];
                    if let Ok(()) = client.set_read_timeout(Some(time::Duration::from_secs(10))) {
                        log::info!("Read timeout was set to 10 seconds");
                    } else {
                        log::warn!("Could not set read timeout to 10 seconds");
                    }
                    if let Ok(bytes_received) = client.read(&mut buffer) {
                        log::info!("{} byte(s) received", bytes_received);
                        let reply = String::from_utf8_lossy(&buffer[..bytes_received]);
                        log::debug!("raw reply received:\n{}", reply);
                        return Some(reply.to_string());
                    } else {
                        log::error!("{} didn't reply anything", self.address);
                    }
                } else {
                    log::error!("{} of {} byte(s) were sent to {}", bytes_written, request.as_bytes().len(), self.address);
                }
            } else {
                log::error!("No bytes were sent to {}", self.address);
            }
            
        } else {
            log::error!("Could not connect to {}", self.address);
        }
        None
    }
}

fn password_and_address(matches: &clap::ArgMatches) -> Option<(String, net::SocketAddrV4)> {
    if let Some(password) = matches.value_of("admin-password") {
        if let Some(address) = matches.value_of("server-address") {
            if let Some(address) = ipparser::sockaddrv4str_to_sockaddrv4(address) {
                return Some((password.to_string(), address));
            }
        }
    }
    None
}
