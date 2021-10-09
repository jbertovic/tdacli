use clap::{App, AppSettings, Arg, ArgGroup, ArgMatches, SubCommand};

pub fn cli_matches<'a>() -> ArgMatches<'a> {
    App::new("TDAmeritrade API CLI")
        .version(crate_version!())
        .setting(AppSettings::VersionlessSubcommands)
        .about("Command Line Interface into tdameritradeclient rust library")
        .subcommand(
            SubCommand::with_name("refresh")
            .about("Fetch valid token or renew refresh_token using a refresh_token grant type")
            .arg(
                Arg::with_name("updaterefresh")
                .short("u")
                .help("Update refresh_token.  Otherwise only token vill be updated.")
            )
            .arg(
                Arg::with_name("clientid")
                .takes_value(true)
                .help("Also known as consumer key as registered at developer.tdameritrade.com")
            )
            .after_help("NOTE: refresh_token must be set in env variable: TDREFRESHTOKEN.")
        )
        .subcommand(
            SubCommand::with_name("auth")
            .arg(
                Arg::with_name("decoded")
                .short("d")
                .help("Code is decoded and NOT url encoded. Do NOT use this if copied from browser window.")
            )
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
            .about("Retrieves refresh_token using authorization_code grant type")
            .after_help("NOTE: code must be set in env variable: TDCODE. See 'weblink' subcommand to retrieve code.")
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
        .subcommand(SubCommand::with_name("userprincipals")
            .about("Retrieves User Principals")
            .after_help("NOTE: Token must be set in env variable: TDAUTHTOKEN.")
        )
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
                .after_help("NOTE: Token must be set in env variable: TDAUTHTOKEN.")
        )
        .subcommand(
            SubCommand::with_name("quote")
                .about("Retrieve quotes for requested symbols")
                .arg(Arg::with_name("symbols")
                    .required(true)
                    .help("Retrieves quotes of supplied <symbols> in format \"sym1,sym2,sym3\""
                ))
                .after_help("NOTE: Token must be set in env variable: TDAUTHTOKEN.")
        )
        .subcommand(
            SubCommand::with_name("transaction")
                .about("Retrieve transaction history")
                .arg(
                    Arg::with_name("account_id")
                        .takes_value(true)
                        .required(true)
                        .help("Retrieves transactions for linked account_id")
                )
                .arg( // symbol shouldn't be an argument with value but ONLY the value
                    Arg::with_name("transaction_type")
                        .long("type")
                        .takes_value(true)
                        .help("Specify Transaction Type otherwise ALL (See below)")
                )
                .arg( // symbol shouldn't be an argument with value but ONLY the value
                    Arg::with_name("trans_id")
                        .help("(OPTIONAL) Grab one transaction by ID. Ignores all other options.")
                )
                .arg( // symbol shouldn't be an argument with value but ONLY the value
                    Arg::with_name("symbol")
                        .long("symbol")
                        .takes_value(true)
                        .help("Specify symbol, otherwise all symbols")
                )
                .arg( // symbol shouldn't be an argument with value but ONLY the value
                    Arg::with_name("start_date")
                        .long("sdate")
                        .takes_value(true)
                        .help("Start date in yyyy-mm-dd. Max range is 1 year")
                )
                .arg( // symbol shouldn't be an argument with value but ONLY the value
                    Arg::with_name("end_date")
                        .long("edate")
                        .takes_value(true)
                        .help("End date in yyyy-mm-dd. Max range is 1 year")
                )
                .after_help("NOTE: Token must be set in env variable: TDAUTHTOKEN. \n\n\r\
                Transaction Types: ALL, TRADE, BUY_ONLY, SELL_ONLY, CASH_IN_OR_CASH_OUT, CHECKING, \n\
                DIVIDEND, INTEREST, OTHER, ADVISOR_FEES")
        )
        .subcommand(
            SubCommand::with_name("instrument")
                .about("Retrieve instrument information or search for instrument")
                .arg( // symbol shouldn't be an argument with value but ONLY the value
                    Arg::with_name("search")
                        .required(true)
                        .takes_value(true)
                        .help("Symbol of instrument or search parameter")
                )
                .arg( // symbol shouldn't be an argument with value but ONLY the value
                    Arg::with_name("search_type")
                        .required(true)
                        .takes_value(true)
                        .help("Specifies type of request")
                )
                .after_help("NOTE: Token must be set in env variable: TDAUTHTOKEN. \n\r\n\r\
                Type of Request \n\r\
                - symbol-search: Retrieve instrument data of a specific symbol or cusip \n\r\
                - symbol-regex: Retrieve instrument data for all symbols matching regex. \n\r\
                Example: search = XYZ.* will return all symbols beginning with XYZ \n\r\
                - desc-search: Retrieve instrument data for instruments whose description \n\r\
                contains the word supplied. Example: search = FakeCompany will return \n\r\
                all instruments with FakeCompany in the description. \n\r\
                - desc-regex: Search description with full regex support. \n\r\
                Example: search = XYZ.[A-C] returns all instruments whose descriptions \n\r\
                contain a word beginning with XYZ followed by a character A through C. \n\r\
                - fundamental: Returns fundamental data for a single instrument specified by exact symbol.\n\r\
                - cusip: use the supplied cusip to look up details of instrument.")
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
                .after_help("NOTE: Token must be set in env variable: TDAUTHTOKEN. \n\r\
                                Think of the frequency as the size of a candle on the chart or how to divide the ticks. \n\
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
                .after_help("NOTE: Token must be set in env variable: TDAUTHTOKEN.\n\r\
                                    '*' indicates default value.")
            )
        .subcommand(
            SubCommand::with_name("watchlist")
                .about("Retrieve Watchlist")
                .arg(
                    Arg::with_name("account_id")
                        .takes_value(true)
                        .required(true)
                        .help("Retrieves watchlist for specified account <account_id>")
                )
                .arg( 
                    Arg::with_name("watchlist_id")
                        //.long("watchlist_id")
                        .takes_value(true)
                        .help("Specify <watchlist_id> or retrieve all watchlists")
                )
                .after_help("NOTE: Token must be set in env variable: TDAUTHTOKEN.")
        )
        .after_help("Check env variable requirements for each subcommand.\r\n\
            Token can be retrieved using 'refresh' subcommand if you have a valid refresh_token.\r\n\
            Token can also be issued by using 'weblink' subcommand first to retrieve 'authorization_code'\r\n\
             and 'auth' subcommand to issue new token and refresh_token.\r\n\
            '*' indicates default value in subcommand help information.")
        .get_matches()
}
