use clap::ArgMatches;
use tdameritradeclient::{TDAClient, Endpoint, param};

/// Grabs the userprincipals data from tdameritrade
pub fn userprincipals(c: &TDAClient) {
    let resp: String = c.get(&Endpoint::UserPrincipals, &[param::Empty]);
    println!("{}", &resp);
}
/// Grabs the account information with options for Positions, Orders or both
pub fn account(c: &TDAClient, args: &ArgMatches) {
    match args.value_of("account_id") {
        Some(account) => {
            let resp: String;
            if args.is_present("positions") && args.is_present("orders") {
                resp = c.get(&Endpoint::Account(&account), &[param::Account::PositionsAndOrders]);
            } else if args.is_present("positions") {
                resp = c.get(&Endpoint::Account(&account), &[param::Account::Positions]);
            } else if args.is_present("orders") {
                resp = c.get(&Endpoint::Account(&account), &[param::Account::Orders]);
            } else {
                resp = c.get(&Endpoint::Account(&account), &[param::Empty]);
            }
            println!("{}", resp)
        }
        None => println!("{{ \"error\": \"Missing account_id\" }}"),
    }
}
/// Grabs transaction detail with options for date range, or one transaction id
/// or restricted to symbol or type
pub fn transaction(c: &TDAClient, args: &ArgMatches) {
    match args.value_of("account_id") {
        Some(account) => {
            let resp: String;
            // if transaction_id is supplied than just grab that transaction and 
            // ignore all other options
            if args.is_present("trans_id") {
                resp = c.get(&Endpoint::Transaction((&account, args.value_of("trans_id").unwrap())), &[param::Empty]);
            } else {
                let mut params: Vec<param::Transactions> = Vec::new();
                if args.is_present("transaction_type") {
                    params.push(param::Transactions::TransactionType(
                        args.value_of("transaction_type").unwrap(),
                    ));
                }
                if args.is_present("symbol") {
                    params.push(param::Transactions::Symbol(args.value_of("symbol").unwrap()));
                }
                if args.is_present("start_date") {
                    params.push(param::Transactions::StartDate(
                        args.value_of("start_date").unwrap(),
                    ));
                }
                if args.is_present("end_date") {
                    params.push(param::Transactions::EndDate(args.value_of("end_date").unwrap()));
                }
                resp = c.get(&Endpoint::Transactions(&account), &params);
            }
            println!("{}", resp)
        }
        None => println!("{{ \"error\": \"Missing account_id\" }}"),
    }
}
