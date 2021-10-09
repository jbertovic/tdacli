use clap::ArgMatches;
use tdameritradeclient::{TDAClient, Endpoint, param};

/// Grabs the quote for symbols supplied xxx,yyy,zzz
pub fn quote(c: &TDAClient, args: &ArgMatches) {
    match args.value_of("symbols") {
        Some(symbols) => {
            let resp: String = c.get(&Endpoint::Quotes, &[param::Quotes::Symbol(symbols)]);
            println!("{}", resp)
        }
        None => missing_symbol(),
    }
}
/// Grabs the instrument information or search
pub fn instrument(c: &TDAClient, args: &ArgMatches) {
    match args.value_of("search") {
        Some(search) => {
            let mut param: Vec<param::Instruments> = Vec::new();
            param.push(param::Instruments::Symbol(search));
            // if cusip is supplied than use invoke tdameritradeclient::getinstrument
            // otherwise use tdameritradeclient::getinstruments <- ending in 's'
            let stype = args.value_of("search_type").unwrap();
            let resp: String;
            if stype.contains("cusip") {
                resp = c.get(&Endpoint::Instrument(search), &[param::Empty]);
            } else {
                param.push(param::Instruments::SearchType(
                    args.value_of("search_type").unwrap(),
                ));
                resp = c.get(&Endpoint::Instruments, &param);
            }
            println!("{}", resp);
        }
        None => missing_symbol(),
    }
}

/// Grabs the quote history for a symbol using param defining the query
pub fn history(c: &TDAClient, args: &ArgMatches) {
    match args.value_of("symbol") {
        Some(symbol) => {
            let mut param: Vec<param::History> = Vec::new();
            // determine query parameters
            if args.is_present("period") {
                param.push(param::History::Period(
                    args.value_of("period")
                        .unwrap()
                        .parse()
                        .expect("period should be a number. Check --help"),
                ));
            }
            if args.is_present("period_type") {
                param.push(param::History::PeriodType(args.value_of("period_type").unwrap()));
            }
            if args.is_present("freq") {
                param.push(param::History::Frequency(
                    args.value_of("freq")
                        .unwrap()
                        .parse()
                        .expect("freq should be a number. Check --help"),
                ));
            }
            if args.is_present("freq_type") {
                param.push(param::History::FrequencyType(args.value_of("freq_type").unwrap()));
            }
            if args.is_present("startdate") {
                param.push(param::History::StartDate(
                    args.value_of("startdate")
                        .unwrap()
                        .parse()
                        .expect("start date should be a number specifying epoch type"),
                ));
            }
            if args.is_present("enddate") {
                param.push(param::History::EndDate(
                    args.value_of("enddate")
                        .unwrap()
                        .parse()
                        .expect("end date should be a number specifying epoch type"),
                ));
            }
            let response: String = c.get(&Endpoint::History(&symbol), &param);
            println!("{}", response)
        }
        None => missing_symbol(),
    }
}

/// Grabs the quote history for a symbol using param defining the query
pub fn optionchain(c: &TDAClient, args: &ArgMatches) {
    match args.value_of("symbol") {
        Some(symbol) => {
            let mut param: Vec<param::OptionChain> = Vec::new();
            // determine query parameters
            param.push(param::OptionChain::Symbol(symbol));
            if args.is_present("call") {
                param.push(param::OptionChain::ContractType("CALL"));
            }
            if args.is_present("put") {
                param.push(param::OptionChain::ContractType("PUT"));
            }
            if args.is_present("strike_count") {
                param.push(param::OptionChain::StrikeCount(
                    args.value_of("srike_count")
                        .unwrap()
                        .parse()
                        .expect("strike count should be a positive integer"),
                ));
            }
            if args.is_present("quotes") {
                param.push(param::OptionChain::IncludeQuotes(true));
            }
            if args.is_present("strategy") {
                param.push(param::OptionChain::Strategy(args.value_of("strategy").unwrap()));
            }
            if args.is_present("interval") {
                param.push(param::OptionChain::Interval(
                    args.value_of("interval")
                        .unwrap()
                        .parse()
                        .expect("strike interval should be a number"),
                ));
            }
            if args.is_present("strike") {
                param.push(param::OptionChain::Strike(
                    args.value_of("strike")
                        .unwrap()
                        .parse()
                        .expect("specified strike price should be a number"),
                ));
            }
            if args.is_present("range") {
                param.push(param::OptionChain::Range(args.value_of("range").unwrap()));
            }
            if args.is_present("from") {
                param.push(param::OptionChain::FromDate(args.value_of("from").unwrap()));
            }
            if args.is_present("to") {
                param.push(param::OptionChain::ToDate(args.value_of("to").unwrap()));
            }
            if args.is_present("exp_month") {
                param.push(param::OptionChain::ExpireMonth(
                    args.value_of("exp_month").unwrap(),
                ));
            }
            if args.is_present("typeS") {
                param.push(param::OptionChain::OptionType("S"));
            }
            if args.is_present("typeNS") {
                param.push(param::OptionChain::OptionType("NS"));
            }
            let response: String = c.get(&Endpoint::OptionChain, &param);
            println!("{}", response)
        }
        None => missing_symbol(),
    }
}

fn missing_symbol() {
    println!("{{ \"error\": \"Missing symbol\"}}");
}
