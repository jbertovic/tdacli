use clap::ArgMatches;
use tdameritradeclient::{Account, TDAClient, Transactions};

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
            if args.is_present("positions") && args.is_present("orders") {
                resp = c.getaccount(&account, &[Account::PositionsAndOrders]);
            } else if args.is_present("positions") {
                resp = c.getaccount(&account, &[Account::Positions]);
            } else if args.is_present("orders") {
                resp = c.getaccount(&account, &[Account::Orders]);
            } else {
                resp = c.getaccount(&account, &[]);
            }
            println!("{}", resp)
        }
        None => println!("{{ \"error\": \"Missing account_id\" }}"),
    }
}
/// Grabs transaction detail with options for date range, or one transaction id
/// or restricted to symbol or type
/// calls `tdameritradeclient::transactions(account_id, transaction param)
/// or calls `tdameritradeclient::transaction(account_id, transaction_id)
pub fn transaction(c: &TDAClient, args: &ArgMatches) {
    match args.value_of("account_id") {
        Some(account) => {
            let resp: String;
            // if transaction_id is supplied than just grab that transaction and 
            // ignore all other options
            if args.is_present("trans_id") {
                resp = c.gettransaction(&account, args.value_of("trans_id").unwrap());
            } else {
                let mut param: Vec<Transactions> = Vec::new();
                if args.is_present("transaction_type") {
                    param.push(Transactions::TransactionType(
                        args.value_of("transaction_type").unwrap(),
                    ));
                }
                if args.is_present("symbol") {
                    param.push(Transactions::Symbol(args.value_of("symbol").unwrap()));
                }
                if args.is_present("start_date") {
                    param.push(Transactions::StartDate(
                        args.value_of("start_date").unwrap(),
                    ));
                }
                if args.is_present("end_date") {
                    param.push(Transactions::EndDate(args.value_of("end_date").unwrap()));
                }
                resp = c.gettransactions(&account, &param);
            }
            println!("{}", resp)
        }
        None => println!("{{ \"error\": \"Missing account_id\" }}"),
    }
}
