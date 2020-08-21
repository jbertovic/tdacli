use clap::ArgMatches;
use tdameritradeclient::{History, Instruments, OptionChain, TDAClient};

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
/// Grabs the instrument information or search
/// calls `tdameritradeclient::getinstruments(Instrument param)
/// OR calls `tdameritradeclient::getinstrument(cusip)
pub fn instrument(c: &TDAClient, args: &ArgMatches) {
    match args.value_of("search") {
        Some(search) => {
            let mut param: Vec<Instruments> = Vec::new();
            param.push(Instruments::Symbol(search));
            // if cusip is supplied than use invoke tdameritradeclient::getinstrument
            // otherwise use tdameritradeclient::getinstruments <- ending in 's'
            let stype = args.value_of("search_type").unwrap();
            let resp: String;
            if stype.contains("cusip") {
                resp = c.getinstrument(search);
            } else {
                param.push(Instruments::SearchType(
                    args.value_of("search_type").unwrap(),
                ));
                resp = c.getinstruments(&param);
            }
            println!("{}", resp);
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
                    args.value_of("period")
                        .unwrap()
                        .parse()
                        .expect("period should be a number. Check --help"),
                ));
            }
            if args.is_present("period_type") {
                param.push(History::PeriodType(args.value_of("period_type").unwrap()));
            }
            if args.is_present("freq") {
                param.push(History::Frequency(
                    args.value_of("freq")
                        .unwrap()
                        .parse()
                        .expect("freq should be a number. Check --help"),
                ));
            }
            if args.is_present("freq_type") {
                param.push(History::FrequencyType(args.value_of("freq_type").unwrap()));
            }
            if args.is_present("startdate") {
                param.push(History::StartDate(
                    args.value_of("startdate")
                        .unwrap()
                        .parse()
                        .expect("start date should be a number specifying epoch type"),
                ));
            }
            if args.is_present("enddate") {
                param.push(History::EndDate(
                    args.value_of("enddate")
                        .unwrap()
                        .parse()
                        .expect("end date should be a number specifying epoch type"),
                ));
            }
            let response: String = c.gethistory(&symbol, &param);
            println!("{}", response)
        }
        None => missing_symbol(),
    }
}

/// Grabs the quote history for a symbol using param defining the query
/// calls `tdameritradeclient::getoptionchain(symbol, Option param)
pub fn optionchain(c: &TDAClient, args: &ArgMatches) {
    match args.value_of("symbol") {
        Some(symbol) => {
            let mut param: Vec<OptionChain> = Vec::new();
            // determine query parameters
            param.push(OptionChain::Symbol(symbol));
            if args.is_present("call") {
                param.push(OptionChain::ContractType("CALL"));
            }
            if args.is_present("put") {
                param.push(OptionChain::ContractType("PUT"));
            }
            if args.is_present("strike_count") {
                param.push(OptionChain::StrikeCount(
                    args.value_of("srike_count")
                        .unwrap()
                        .parse()
                        .expect("strike count should be a positive integer"),
                ));
            }
            if args.is_present("quotes") {
                param.push(OptionChain::IncludeQuotes(true));
            }
            if args.is_present("strategy") {
                param.push(OptionChain::Strategy(args.value_of("strategy").unwrap()));
            }
            if args.is_present("interval") {
                param.push(OptionChain::Interval(
                    args.value_of("interval")
                        .unwrap()
                        .parse()
                        .expect("strike interval should be a number"),
                ));
            }
            if args.is_present("strike") {
                param.push(OptionChain::Strike(
                    args.value_of("strike")
                        .unwrap()
                        .parse()
                        .expect("specified strike price should be a number"),
                ));
            }
            if args.is_present("range") {
                param.push(OptionChain::Range(args.value_of("range").unwrap()));
            }
            if args.is_present("from") {
                param.push(OptionChain::FromDate(args.value_of("from").unwrap()));
            }
            if args.is_present("to") {
                param.push(OptionChain::ToDate(args.value_of("to").unwrap()));
            }
            if args.is_present("exp_month") {
                param.push(OptionChain::ExpireMonth(
                    args.value_of("exp_month").unwrap(),
                ));
            }
            if args.is_present("typeS") {
                param.push(OptionChain::OptionType("S"));
            }
            if args.is_present("typeNS") {
                param.push(OptionChain::OptionType("NS"));
            }
            let response: String = c.getoptionchain(&param);
            println!("{}", response)
        }
        None => missing_symbol(),
    }
}

fn missing_symbol() {
    println!("{{ \"error\": \"Missing symbol\"}}");
}
