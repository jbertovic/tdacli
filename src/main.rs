//! # tdacli
//! Command line client for the tdameritradeclient
//! subcommands split out as follows into separate modules and corresponding to the tdameritradeclient
//! each subcommand will need to borrow the tdameritradeclient through a passed reference
//!
//! `cli` module contains the clap configuration for arguments
//!
//! module -> subcommands that match tdameritrade::TDAClient
//!
//! `account -> userprinicipals(), account(account_id)`
//! Defines items that deal with account and user information
//!
//! `quote -> quote(symbols), history(symbol, History param), optionchain(symbol, OptionChain param)`
//! Defines items that deal with quotes either live or historical and expanded option quotes
//!
//! `orders` -> getsavedorders(accoundid), getorders(accountid)`
//! Defines items that deal with orders live or saved
//!
#[macro_use(crate_version)]
extern crate clap;

pub mod cli;
pub mod account;
pub mod quote;
pub mod auth;

use std::env;
use tdameritradeclient::TDAClient;

fn main() {
    let matches = cli::cli_matches();
    
    //TODO: add refresh subcommand
    //TODO: refresh: option only to print refresh or token but NOT both
    //TODO: refresh: able to update either token or both tokens
    //TODO: refresh: able to assign to env variables
    //TODO: add orders subcommand
    //TODO: orders: output filled, working, all

    let c = TDAClient::new(env::var("TDAUTHTOKEN")
    .expect("Token is missing inside env variable TDAUTHTOKEN"));

    match matches.subcommand() {
        ("weblink", Some(sub_m)) => auth::weblink(&sub_m),
        ("userprincipals", Some(_)) => account::userprincipals(&c),
        ("account", Some(sub_m)) => account::account(&c, &sub_m),
        ("quote", Some(sub_m)) => quote::quote(&c, sub_m),
        ("history", Some(sub_m)) => quote::history(&c, sub_m),
        ("optionchain", Some(sub_m)) => quote::optionchain(&c, sub_m),
        ("auth", Some(sub_m)) => auth::auth(sub_m),
        ("refresh", Some(sub_m)) => auth::refresh(sub_m),
        //order
        _ => {}
    }
}

