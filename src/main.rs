extern crate clap;
extern crate regex;

use clap::AppSettings;
use clap::SubCommand;
use clap::App;
use clap::Arg;
use regex::Regex;


fn bool_validator(b: String) -> Result<(), String> {
    if let Ok(_b) = b.parse::<bool>() {
        Ok(())
    }
    format!("{} is not a valid boolean value")
}

fn list_size_validator(ll: String) -> Result<(), String> {
    if let Ok(_ll) = ll.parse::<u16>() {        
        Ok(())
    }
    format!("{} is not a valid list size number, this value must be between [1,65535]")
}

fn usize_validator(num: String) -> Result<(), String> {
    if let Ok(_n) = num.parse::<usize>() {
        Ok(())
    }
    format!("{} is not a valid unsigned number")
}

fn mac_validator(mac: String) -> Result<(), String> {
    if mac.is_ascii() {
        let mac_re = Regex::new(r"^((([a-fA-F0-9][a-fA-F0-9]+[-]){5}|([a-fA-F0-9][a-fA-F0-9]+[:]){5})([a-fA-F0-9][a-fA-F0-9])$)|(^([a-fA-F0-9][a-fA-F0-9][a-fA-F0-9][a-fA-F0-9]+[.]){2}([a-fA-F0-9][a-fA-F0-9][a-fA-F0-9][a-fA-F0-9]))$").unwrap();
        Ok(())
    }
    format!("{} is not a valid mac address")
}

fn username_pattern_validator(pattern: String) -> Result<(), String> {
    if pattern.is_ascii() {
        Ok(())
    }
    format!("{} is not a valid pattern", pattern)
}

fn key_password_validator(key: String) -> Result<(), String> {
    key.is_ascii() && key.len() < 33
}

fn capacity_validator(c: String) -> Result<(), String> {
    if let Ok(v) = c.parse::<u16>() {
        if v < 2 {
            return Err(format!("The capacity must be between [2,65535]"));
        } else {
            return Ok(());
        }
    }
}

fn main() {
    let matches = App::new("ADMINT")
                          .version("1.0")
                          .author("Jorge A. <jorge4larcon@gmail.com>")
                          .about("ADministration tool for MINT server")
                          .setting(AppSettings::ArgRequiredElseHelp)
                          .subcommand(SubCommand::with_name("set-dropvotes")
                                       .about("Set the drop votes of the server, this command can drop users that are logged in the server")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("drop-votes")
                                            .short("d")
                                            .long("drop-votes")
                                            .value_name("DROP VOTES")
                                            .help("The new drop votes value for the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(list_size_validator)))
                          .subcommand(SubCommand::with_name("set-dropverification")
                                       .about("Enable/Disable the drop verification in the server")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("state")
                                            .short("s")
                                            .long("state")
                                            .value_name("STATE")
                                            .help("The new state of the drop verification in the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(list_size_validator)))
                          .subcommand(SubCommand::with_name("set-listsize")
                                       .about("Set the list size of the server")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("list-size")                                       
                                            .short("l")
                                            .long("list-size")
                                            .value_name("LIST SIZE")
                                            .help("The new list size of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(list_size_validator)))
                          .subcommand(SubCommand::with_name("set-capacity")
                                       .about("Set the capacity of the server")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("capacity")                                      
                                            .short("c")
                                            .long("capacity")
                                            .value_name("CAPACITY")
                                            .help("The new capacity of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(capacity_validator)))
                          .subcommand(SubCommand::with_name("set-password")
                                       .about("Set the password for the normal users")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("password")
                                            .short("p")
                                            .long("password")
                                            .value_name("PASSWORD")
                                            .help("The new password for the clients")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(key_password_validator)))
                          .subcommand(SubCommand::with_name("set-key")
                                       .about("Set the password for the admin user")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("key")
                                            .short("k")
                                            .long("key")
                                            .value_name("KEY")
                                            .help("The new password for the admin")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(key_password_validator)))
                          .subcommand(SubCommand::with_name("drop")
                                       .about("Drop a client from the server with an specific ip address")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("ip")
                                            .short("i")
                                            .long("ip")
                                            .value_name("IP ADDRESS")
                                            .help("The IPv4 address of the client to drop (can be more than one)")
                                            .takes_value(true)
                                            .required(true)
                                            .multiple(true)))
                          .subcommand(SubCommand::with_name("get-mac")
                                       .about("Get a client from the server with an specific mac address")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("mac")
                                            .short("m")
                                            .long("mac")
                                            .value_name("MAC")
                                            .long_help("The mac to search, it could be aaaa.bbbb.cccc, aa:aa:bb:bb:cc:cc or aa-aa-bb-bb-cc-cc")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(mac_validator)))
                          .subcommand(SubCommand::with_name("get-username")
                                       .about("Get a list of clients from the server with an specific pattern in their usernames")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("pattern")
                                            .short("p")
                                            .long("pattern")
                                            .value_name("PATTERN")
                                            .help("The pattern to search")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(username_pattern_validator)))
                          .subcommand(SubCommand::with_name("get-index")
                                       .about("Get a list of clients from the server")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("start")
                                            .short("s")
                                            .long("start")
                                            .value_name("START_INDEX")
                                            .help("The start index of the list")
                                            .default_value("0")
                                            .takes_value(true)
                                            .required(false)
                                            .number_of_values(1)
                                            .validator(usize_validator)))
                                        .arg(Arg::with_name("end")
                                            .short("e")
                                            .long("end")
                                            .value_name("END_INDEX")
                                            .help("The end index of the list")
                                            .default_value("10")
                                            .takes_value(true)
                                            .required(false)
                                            .number_of_values(1)
                                            .validator(usize_validator)))
                          .get_matches();
}
