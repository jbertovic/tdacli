use clap::{App, Arg, SubCommand, ArgMatches, AppSettings};

pub fn cli_matches<'a>() -> ArgMatches<'a> {
    App::new("TDAmeritrade API CLI")
        .version(crate_version!())
        .setting(AppSettings::VersionlessSubcommands)
        .about("CLI Interface into tdameritradeclient rust library")
        .subcommand(SubCommand::with_name("userprincipals").about("Retrieves User Principals"))
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
                .about("Retrieve history for one <symbol>.")
                .arg( // symbol shouldn't be an argument with value but ONLY the value
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
                        .help("Defines start date in epoch format. <period> should not be provided")
                )
                .arg(
                    Arg::with_name("enddate")
                        .takes_value(true)
                        .help("Defines end date epoch format. Default is previous trading day.")
                )
            )
        .subcommand(
            SubCommand::with_name("optionchain")
                .about("Retrieve option chain for one <symbol>")
                .arg( // symbol shouldn't be an argument with value but ONLY the value
                    Arg::with_name("symbol")
                        //.takes_value(true)
                        .required(true)
                        .help("Retrieves history of supplied [symbol]")
                )
                .arg(
                    Arg::with_name("contract_type")
                        .long("ctype")
                        .takes_value(true)
                        .help("Type of contract to return in chain. <CALL, PUT or ALL (default)>")
                )
                .arg(
                    Arg::with_name("strike_count")
                        .long("xcount")
                        .takes_value(true)
                        .help("Number of strikes to return above and below at-the-money price")
                )
                .arg(
                    Arg::with_name("quotes")
                        .short("q")
                        .help("Include quotes for options in the option chain. Default is NOT included")
                )
                .arg(
                    Arg::with_name("strategy")
                        .long("strategy")
                        .takes_value(true)
                        .help("Returns strategy chain. Values: <SINGLE (default), COVERED, VERTICAL, CALENDAR, STRANGLE, STRADDLE, \
                                BUTTERFLY, CONDOR, DIAGONAL, COLLAR, or ROLL>")
                )
                .arg(
                    Arg::with_name("interval")
                        .takes_value(true)
                        .help("Strike interval for spread strategy chains. To used with <strategy> argument.")
                )
                .arg(
                    Arg::with_name("strike")
                        .short("x")
                        .takes_value(true)
                        .help("Return options only at specified strike price")
                )
                .arg(
                    Arg::with_name("range")
                        .takes_value(true)
                        .help("Specify range: <ALL (default), ITM, NTM, OTM, SAK, SBK, or SNK>")
                )
                .arg(
                    Arg::with_name("from")
                        .takes_value(true)
                        .help("Specify from date 'yyyy-MM-dd' or 'yyyy-mm-dd'T'HH:mm:ssz'")
                )
                .arg(
                    Arg::with_name("to")
                        .takes_value(true)
                        .help("Specify to date 'yyyy-MM-dd' or 'yyyy-mm-dd'T'HH:mm:ssz'")
                )
                .arg(
                    Arg::with_name("exp_month")
                        .long("expm")
                        .takes_value(true)
                        .help("Return only options expiring in given month <JAN, FEB..>. Default is ALL.")
                )
                .arg(
                    Arg::with_name("option_type")
                        .long("otype")
                        .takes_value(true)
                        .help("Standard or Non-standard contracts: <S, NS, ALL (default)>")
                )
            )
        .after_help("A valid token must be set in env variable: TDAUTHTOKEN.\r\n'*' indicates default value in subcommand help information.")
        .get_matches()
}
