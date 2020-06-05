//! # tdacli
//! Command line client for the tdameritradeclient
//! subcommands split out as follows into separate modules and corresponding to the tdameritradeclient
//!
//! `account -> getuserprinicipals(), getaccount(accountid)`
//! Defines items that deal with account and user information
//!
//! `quote -> getquote(symbols), gethistory(symbol, History param)`
//! Defines items that deal with quotes either live or historical
//!
//! `optionchain -> getoptionchain(symbol, OptionChain param)`
//! Returns option chain data for a symbol
//!
//! `orders` -> getsavedorders(accoundid), getorders(accountid)`
//! Defines items that deal with orders live or saved
//!
#[macro_use(crate_version)]
extern crate clap;

pub mod cli;
use cli::cli_matches;

use std::env;
use tdameritradeclient::{TDAClient, Account, History};

// TODO: Refactor subcommands to separate files

fn main() {
    let matches = cli_matches();
        
    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());

    match matches.subcommand() {
        ("userprincipals", Some(_)) => {
            let resp: String = c.getuserprincipals();
            println!("{}", &resp);
        }
        ("account", Some(sub_m)) => match sub_m.value_of("account_id") {
            Some(account) => {
                let resp: String;
                if sub_m.is_present("positions")&&sub_m.is_present("orders") {
                    resp = c.getaccount(&account, &[Account::PositionsAndOrders]);
                }
                else if sub_m.is_present("positions") {
                    resp = c.getaccount(&account, &[Account::Positions]);
                }
                else if sub_m.is_present("orders") {
                    resp = c.getaccount(&account, &[Account::Orders]);
                }
                else {
                    resp = c.getaccount(&account, &[]);
                }
                println!("{}", resp)
            }
            None => println!("Missing account_id"),
        },
        ("quote", Some(sub_m)) => match sub_m.value_of("symbols") {
            Some(symbols) => {
                let resp: String = c.getquotes(&symbols);
                println!("{}", resp)
            }
            None => println!("Missing symbols"),
        },
        ("history", Some(sub_m)) => match sub_m.value_of("symbol") {
            Some(symbol) => {

                let mut param: Vec<History> = Vec::new();
                // determine query parameters
                if sub_m.is_present("period") {
                    param.push(History::Period(sub_m.value_of("period").unwrap().parse().unwrap()));
                }
                if sub_m.is_present("period_type") {
                    param.push(History::PeriodType(sub_m.value_of("period_type").unwrap()));
                }
                if sub_m.is_present("freq") {
                    param.push(History::Frequency(sub_m.value_of("freq").unwrap().parse().unwrap()));
                }
                if sub_m.is_present("freq_type") {
                    param.push(History::FrequencyType(sub_m.value_of("freq_type").unwrap()));
                }
                let response: String = c.gethistory(&symbol, &param);
                println!("{}", response)
            }
            None => println!("Missing symbol"),
        },
        ("optionchain", Some(sub_m)) => match sub_m.value_of("symbol") {
            Some(symbol) => println!("Not Yet Implemented: Symbol {}", symbol),
            None => println!("Missing symbol"),
        },
        _ => {}
    }
}

