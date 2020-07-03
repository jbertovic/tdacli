use clap::ArgMatches;
use tdameritradeclient::auth::{getcodeweblink, gettoken_fromrefresh, getrefresh_fromrefresh, TDauth};

/// Create a link to use to get an authorization_code from tdameritrade
/// Code can be used to fetch a token and refresh token in the `auth` subcommand
pub fn weblink(args: &ArgMatches) {
    println!("{}", 
        getcodeweblink(args.value_of("clientid").unwrap(), args.value_of("redirect").unwrap()));
}
/// Fetch updated `token` or `refresh_token` from a current `refresh_token`
pub fn auth(args: &ArgMatches, code: String) {
    match args.value_of("clientid") {
        Some(clientid) => {
            match args.value_of("redirect") {
                Some(redirect) => {
                    let decoded = !args.is_present("decoded");
                    let tdauth = TDauth::new_fromcode(&code, clientid, redirect, decoded);
                    let (t, r) = tdauth.gettokens();
                    println!("{{\"Token\": \"{}\", \"Refresh\": \"{}\"}}", t, r)
                },
                None => println!("{{ \"error\": \"Missing redirect\"}}"),
            }
        },
        None => println!("{{ \"error\": \"Missing clientid\"}}"),
    }
}
/// Fetch updated `token` or `refresh_token` from a current `refresh_token`
pub fn refresh(args: &ArgMatches, rtoken: String) {
    match args.value_of("clientid") {
        Some(clientid) => {
            // do i need to renew refresh or only token?
            // need to update tdameritradeclient to include getrefresh_fromrefresh()
            let token = if args.is_present("updaterefresh") {
                gettoken_fromrefresh(&rtoken, clientid)
            } else {
                getrefresh_fromrefresh(&rtoken, clientid)
            };
            println!("{}", token);
        }
        None => println!("{{ \"error\": \"Missing clientid\"}}"),
    }
}