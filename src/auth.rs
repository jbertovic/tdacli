use clap::ArgMatches;
use tdameritradeclient::auth::{
    get_code_weblink, get_refresh_from_refresh, get_token_from_refresh, TDauth,
};

/// Create a link to use to get an authorization_code from tdameritrade
/// Code can be used to fetch a token and refresh token in the `auth` subcommand
pub fn weblink(args: &ArgMatches) {
    println!(
        "{}",
        get_code_weblink(
            args.value_of("clientid").unwrap(),
            args.value_of("redirect").unwrap()
        )
    );
}
/// Fetch updated `refresh_token` from a current `authorization_code` as retrieved from `weblink`
pub fn auth(args: &ArgMatches, code: String) {
    match args.value_of("clientid") {
        Some(clientid) => match args.value_of("redirect") {
            Some(redirect) => {
                let decoded = !args.is_present("decoded");
                let tdauth = TDauth::new_from_code(&code, clientid, redirect, decoded);
                let (_t, refresh) = tdauth.get_tokens();
                println!("{}", refresh);
            }
            None => println!("{{ \"error\": \"Missing redirect\"}}"),
        },
        None => println!("{{ \"error\": \"Missing clientid\"}}"),
    }
}
/// Fetch updated `token` or `refresh_token` from a current `refresh_token`
pub fn refresh(args: &ArgMatches, rtoken: String) {
    match args.value_of("clientid") {
        Some(clientid) => {
            // do i need to renew refresh or only token?
            // need to update tdameritradeclient to include get_refresh_from_refresh()
            let token = if !args.is_present("updaterefresh") {
                get_token_from_refresh(&rtoken, clientid)
            } else {
                get_refresh_from_refresh(&rtoken, clientid)
            };
            println!("{}", token);
        }
        None => println!("{{ \"error\": \"Missing clientid\"}}"),
    }
}
