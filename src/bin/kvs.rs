use clap::{App, AppSettings, SubCommand, Arg};
use std::process::exit;
use kvs::KvStore;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .setting(AppSettings::DisableHelpFlags)
        .subcommand(
            clap::SubCommand::with_name("set")
                .about("set hhhhh")
                .arg(Arg::with_name("key").help("A string key").required(true))
                .arg(Arg::with_name("value").help("A string value").required(true)),
        )
        .subcommand(
            SubCommand::with_name("get")
            .arg(Arg::with_name("key").help("key").required(true)),)
        .subcommand(
                SubCommand::with_name("rm")
                    .about("Remove a given key")
                    .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .get_matches();

    let mut kv = KvStore::new();


    match matches.subcommand() {
        ("set", Some(_matches)) => {
            kv.set(matches.value_of("key").unwrap_or("wdw").to_owned(), matches.value_of("value").unwrap_or("ccpp").to_owned())
        }
        ("get", Some(_matches)) => {

            let val = kv.get(matches.value_of("key").unwrap_or("test").to_owned());
            println!("{}", val.unwrap_or(String::new()));
        }
        ("rm", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }

}