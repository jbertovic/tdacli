use clap::ArgMatches;
use tdameritradeclient::{TDAClient, Account};

/// Grabs the userprincipals data from tdameritrade
/// calls `tdameritradeclient::getuserprincipals()`
pub fn userprincipals(c: &TDAClient) {
    let resp: String = c.getuserprincipals();
    println!("{}", &resp);
}

/// Grabs the account information with options for Positions, Orders or both
/// calls `tdameritradeclient::getaccount(account_id, Account param)
pub fn account(c: &TDAClient, args: &ArgMatches) {
    match args.value_of("account_id") {
        Some(account) => {
            let resp: String;
            if args.is_present("positions")&&args.is_present("orders") {
                resp = c.getaccount(&account, &[Account::PositionsAndOrders]);
            }
            else if args.is_present("positions") {
                resp = c.getaccount(&account, &[Account::Positions]);
            }
            else if args.is_present("orders") {
                resp = c.getaccount(&account, &[Account::Orders]);
            }
            else {
                resp = c.getaccount(&account, &[]);
            }
            println!("{}", resp)
        }
        None => println!("{{ \"error\": \"Missing account_id\" }}"),
    }
}
