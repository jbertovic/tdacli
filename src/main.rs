#[macro_use(crate_version)]
extern crate clap;
use clap::{App, Arg, SubCommand};
use std::env;
use tdameritradeclient::{Execute, TDAClient, Account, History};

//use subcommands: userprincipals (DONE), account (DONE), quote (DONE), history, optionchain

//TODO: Split up subcommands into different files to clean it up
//TODO: Add chrono to deal with timestamp - think how to modify incoming json or will i need to parse into objects?

fn main() {
    let matches =
        App::new("TDAmeritrade API CLI")
            .version(crate_version!())
            .about("CLI Interface into tdameritradeclient rust library")
            .subcommand(SubCommand::with_name("userprincipals").about("Fetches User Principals"))
            .subcommand(
                SubCommand::with_name("account")
                    .about("Retrieve account information for <account_id>")
                    .arg(
                        Arg::with_name("account_id")
                            //.takes_value(true)
                            //.index(1)
                            .help("Retrieves account information for linked account_id")
                    )
                    .arg(
                        Arg::with_name("positions")
                            .short("p")
                            .help("includes account positions")
                    )
                    .arg(
                        Arg::with_name("orders")
                            .short("o")
                            .help("includes account orders")
                    )
            )
            .subcommand(
                SubCommand::with_name("quote")
                    .about("Retrieve quotes for [symbols]")
                    .arg(Arg::with_name("symbols")
                        .help("Retrieves quotes of supplied [symbols] in format \"sym1,sym2,sym3\""
                    ))
            )
            .subcommand(
                SubCommand::with_name("history")
                    .about("Retrieve history for one <symbol>. \r\n'*' indicates default value in argument values.")
                    .arg( //TODO: symbol shouldn't be an argument with value but ONLY the value
                        Arg::with_name("symbol")
                            //.takes_value(true)
                            .required(true)
                            .help("Retrieves history of supplied [symbol]")
                    )
                    .arg(
                        Arg::with_name("period_type")
                            .long("ptype")
                            .takes_value(true)
                            .help("Defines period type: day, month, year, ytd")
                    )
                    .arg(
                        Arg::with_name("period")
                            .long("period")
                            .takes_value(true)
                            .help("Defines number of periods to show; day: 1,2,3,4,5,10* | month: 1*,2,3,6 | year: 1*,2,3,5,10,15,20 | ytd: 1*")
                    )
                    .arg(
                        Arg::with_name("freq_type")
                            .long("ftype")
                            .takes_value(true)
                            .help("Defines freq of new candle by period_type: day: minute* | month: daily, weekly* | year: daily, weekly, monthly* | ytd: daily, weekly*")
                    )
                    .arg(
                        Arg::with_name("freq")
                            .long("freq")
                            .takes_value(true)
                            .help("Defines number of freq_types to show: minute: 1*,5,10,15,30 | daily: 1* | weekly: 1* | monthly: 1*")
                    )
                    .arg(
                        Arg::with_name("startdate")
                            .takes_value(true)
                            .help("Defines start date <yyyy-mm-dd>. <period> should not be provided")
                    )
                    .arg(
                        Arg::with_name("enddate")
                            .takes_value(true)
                            .help("Defines end date <yyyy-mm-dd>. Default is previous trading day.")
                    )
                )
            .subcommand(
                SubCommand::with_name("optionchain")
                    .about("Retrieve history for one <symbol>")
                    .arg(
                        Arg::with_name("symbol")
                            .takes_value(true)
                            .help("Retrieves Option Chain of supplied <symbol>")
                    )
            )
            .after_help("A valid token must be set in env variable: TDAUTHTOKEN")
            .get_matches();

    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());

    match matches.subcommand() {
        ("userprincipals", Some(_)) => {
            let resp: String = c.getuserprincipals().execute();
            println!("{}", &resp);
        }
        ("account", Some(sub_m)) => match sub_m.value_of("account_id") {
            Some(account) => {
                let resp: String;
                if sub_m.is_present("positions")&&sub_m.is_present("orders") {
                    resp = c.getaccount(&account)
                        .params(&[Account::PositionsAndOrders.into()])
                        .execute();
                }
                else if sub_m.is_present("positions") {
                    resp = c.getaccount(&account)
                        .params(&[Account::Positions.into()])
                        .execute();
                }
                else if sub_m.is_present("orders") {
                    resp = c.getaccount(&account)
                        .params(&[Account::Orders.into()])
                        .execute();
                }
                else {
                    resp = c.getaccount(&account).execute();
                }
                println!("{}", resp)
            }
            None => println!("Missing account_id"),
        },
        ("quote", Some(sub_m)) => match sub_m.value_of("symbols") {
            Some(symbols) => {
                let resp: String = c.getquotes(&symbols).execute();
                println!("{}", resp)
            }
            None => println!("Missing symbols"),
        },
        ("history", Some(sub_m)) => match sub_m.value_of("symbol") {
            Some(symbol) => {

                let mut param: Vec<(&str, String)> = Vec::new();
                // determine query parameters
                if sub_m.is_present("period") {
                    param.push(History::Period(sub_m.value_of("period").unwrap().parse().unwrap()).into());
                }
                if sub_m.is_present("period_type") {
                    param.push(History::PeriodType(sub_m.value_of("period_type").unwrap()).into());
                }
                if sub_m.is_present("freq") {
                    param.push(History::Frequency(sub_m.value_of("freq").unwrap().parse().unwrap()).into());
                }
                if sub_m.is_present("freq_type") {
                    param.push(History::FrequencyType(sub_m.value_of("freq_type").unwrap()).into());
                }
                let request = c.gethistory(&symbol);
                let response: String = 
                    if !param.is_empty() { request.params(&param).execute() }
                    else { request.execute() };
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
