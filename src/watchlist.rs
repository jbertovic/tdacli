use clap::ArgMatches;
use tdameritradeclient::{TDAClient, Endpoint, param};

/// Grabs all the watchlists or a specified watchlist for supplied account id
pub fn watchlist(c: &TDAClient, args: &ArgMatches) {
    let account_id = args.value_of("account_id").unwrap();
    match args.value_of("watchlist_id") {
        Some(watchlist_id) => {
            let resp: String = c.get(&Endpoint::Watchlist((account_id, watchlist_id)), &[param::Empty]);
            println!("{}", resp)
        }
        None => {
            let resp: String = c.get(&Endpoint::Watchlists(account_id), &[param::Empty]);
            println!("{}", resp)
        },
    }
}

