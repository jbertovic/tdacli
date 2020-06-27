use clap::{App, Arg, SubCommand, ArgMatches, AppSettings, ArgGroup};

pub fn cli_matches<'a>() -> ArgMatches<'a> {
    App::new("TDAmeritrade API CLI")
        .version(crate_version!())
        .setting(AppSettings::VersionlessSubcommands)
        .about("Command Line Interface into tdameritradeclient rust library")
        .arg(
            Arg::with_name("refresh")
            .short("r")
            .help("Use env variable: TDREFRESHTOKEN as a refresh token.")
        )
        .arg(
            Arg::with_name("printrefresh")
            .short("p")
            .help("Print current token")
        )
        .subcommand(
            SubCommand::with_name("auth")
            .arg(
                Arg::with_name("clientid")
                .takes_value(true)
                .required(true)
                .help("Also known as consumer key as registered at developer.tdameritrade.com")
            )
            .arg(
                Arg::with_name("redirect")
                .takes_value(true)
                .required(true)
                .help("Redirect URI as registered at developer.tdameritrade.com")
            )
            .about("Refresh token from refresh_token or authorization_code\r\n\
            NOTE: code must be set in env variable: TDCODE")
        )
        .subcommand(
            SubCommand::with_name("weblink")
            .arg(
                Arg::with_name("clientid")
                .takes_value(true)
                .required(true)
                .help("Also known as consumer key as registered at developer.tdameritrade.com")
            )
            .arg(
                Arg::with_name("redirect")
                .takes_value(true)
                .required(true)
                .help("Redirect URI as registered at developer.tdameritrade.com")
            )
            .about("Gives you the url to get authorization code from TDAmeritrade")
        )
        .subcommand(SubCommand::with_name("userprincipals").about("Retrieves User Principals"))
        .subcommand(
            SubCommand::with_name("account")
                .about("Retrieve account information for <account_id>")
                .arg(
                    Arg::with_name("account_id")
                        .takes_value(true)
                        .required(true)
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
                .about("Retrieve quotes for requested symbols")
                .arg(Arg::with_name("symbols")
                    .required(true)
                    .help("Retrieves quotes of supplied <symbols> in format \"sym1,sym2,sym3\""
                ))
        )
        .subcommand(
            SubCommand::with_name("history")
                .about("Retrieve history for one <symbol>.")
                .arg( // symbol shouldn't be an argument with value but ONLY the value
                    Arg::with_name("symbol")
                        //.takes_value(true)
                        .required(true)
                        .help("Symbol of instrument.")
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
                .after_help("Think of the frequency as the size of a candle on the chart or how to divide the ticks. \n\
                                and the period as the term or total length of the history. \n\
                                '*' indicates default value.")
            )
        .subcommand(
            SubCommand::with_name("optionchain")
                .about("Retrieve option chain for one <symbol>")
                .arg( // symbol shouldn't be an argument with value but ONLY the value
                    Arg::with_name("symbol")
                        .required(true)
                        .help("Symbol of underlying instrument.")
                )
                .arg(
                    Arg::with_name("call")
                        .short("c")
                        .help("Retrieve CALL contract types only.")
                )
                .arg(
                    Arg::with_name("put")
                        .short("p")
                        .help("Retrieve PUT contract types only.")
                )
                .group(ArgGroup::with_name("contract_type")
                    .args(&["call", "put"])
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
                        .help("Include underlying instrument quote.")
                )
                .arg(
                    Arg::with_name("strategy")
                        .long("strategy")
                        .takes_value(true)
                        .help("Returns strategy chain. Values: <SINGLE*, COVERED, VERTICAL, CALENDAR, STRANGLE, STRADDLE, \
                                BUTTERFLY, CONDOR, DIAGONAL, COLLAR, or ROLL>")
                )
                .arg(
                    Arg::with_name("interval")
                        .long("interval")
                        .takes_value(true)
                        .help("Strike interval for spread strategy chains. To used with <strategy> argument.")
                )
                .arg(
                    Arg::with_name("strike")
                        .long("strike")
                        .short("x")
                        .takes_value(true)
                        .help("Return options only at specified strike price")
                )
                .arg(
                    Arg::with_name("range")
                        .long("range")
                        .takes_value(true)
                        .help("Specify range: <ALL*, ITM, NTM, OTM, SAK, SBK, or SNK>")
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
                        .short("m")
                        .takes_value(true)
                        .help("Return only options expiring in given month <JAN, FEB..>. Default is ALL.")
                )
                .arg(
                    Arg::with_name("typeS")
                        .short("s")
                        .help("Standard contracts only.")
                )
                .arg(
                    Arg::with_name("typeNS")
                        .short("n")
                        .help("Non-Standard contracts only.")
                )
                .group(ArgGroup::with_name("option_type")
                    .args(&["typeS", "typeNS"])
                )
                .after_help("'*' indicates default value.")
            )
        .after_help("A valid token must be set in env variable: TDAUTHTOKEN.\r\n\
            If '-r' flag is used than env variable: TDREFRESHTOKEN can be used instead.\r\n\
            '*' indicates default value in subcommand help information.")
        .get_matches()
}
