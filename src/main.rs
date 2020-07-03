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
    
    //TODO: add orders subcommand
    //TODO: orders: output filled, working, all
    //TODO: orders: add (need to determine how to specify order desc json)
    //TODO: orders: delete

    match matches.subcommand() {
        (cmd, Some(sub_m)) => {
            match cmd {

                // relies on NO Env Variables
                "weblink" => auth::weblink(&sub_m),

                // relies on env variable TDREFRESHTOKEN
                "refresh" => {
                    let refresh = env::var("TDREFRESHTOKEN")
                        .expect("Token is missing inside env variable TDREFRESHTOKEN");
                    auth::refresh(sub_m, refresh);
                    }

                // relies on env variable TDCODE
                "auth" => {
                    let code = env::var("TDCODE")
                        .expect("Code is missing inside env variable TDCODE");
                    auth::auth(sub_m, code);
                    }

                // relies on env variable TDAUTHTOKEN and tdameritradeclient::TDClient
                _ => {
                    let c = TDAClient::new(env::var("TDAUTHTOKEN")
                        .expect("Token is missing inside env variable TDAUTHTOKEN"));
                    match cmd {
                        "userprincipals" => account::userprincipals(&c),
                        "account" => account::account(&c, &sub_m),
                        "quote" => quote::quote(&c, sub_m),
                        "history" => quote::history(&c, sub_m),
                        "optionchain" => quote::optionchain(&c, sub_m),
                        _ => {},
                    }
                }
            }
        }
        _ => {println!("Subcommand must be specified.  For more information try --help")}
    }
}

