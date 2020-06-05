use clap::{App, Arg, SubCommand, ArgMatches};

pub fn cli_matches<'a>() -> ArgMatches<'a> {
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
        .get_matches()
}
