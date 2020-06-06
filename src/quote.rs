use clap::ArgMatches;
use tdameritradeclient::{TDAClient, History};

/// Grabs the quote for symbols supplied xxx,yyy,zzz
/// calls `tdameritradeclient::getquote(symbols)
pub fn quote(c: &TDAClient, args: &ArgMatches) {
    match args.value_of("symbols") {
        Some(symbols) => {
            let resp: String = c.getquotes(&symbols);
            println!("{}", resp)
        }
        None => missing_symbol(),
    }
}

/// Grabs the quote history for a symbol using param defining the query
/// calls `tdameritradeclient::gethistory(symbol, History param)
pub fn history(c: &TDAClient, args: &ArgMatches) {
    match args.value_of("symbol") {
        Some(symbol) => {
            let mut param: Vec<History> = Vec::new();
            // determine query parameters
            if args.is_present("period") {
                param.push(History::Period(
                    args.value_of("period").unwrap().parse().unwrap(),
                ));
            }
            if args.is_present("period_type") {
                param.push(History::PeriodType(args.value_of("period_type").unwrap()));
            }
            if args.is_present("freq") {
                param.push(History::Frequency(
                    args.value_of("freq").unwrap().parse().unwrap(),
                ));
            }
            if args.is_present("freq_type") {
                param.push(History::FrequencyType(args.value_of("freq_type").unwrap()));
            }
            let response: String = c.gethistory(&symbol, &param);
            println!("{}", response)
        }
        None => missing_symbol(),
    }
}


fn missing_symbol() {
    println!("{{ \"error\": \"Missing symbols\"}}");
}