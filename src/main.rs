use clap::{App, Arg, SubCommand};
use std::env;
use tdameritradeclient::{Execute, TDAClient};

//use subcommands: userprincipals (DONE), quote (DONE), history, optionchain

fn main() {
    let matches = App::new("TDAmeritrade API CLI")
        .version("0.1.0")
        .about("CLI Interface into tdameritradeclient rust library")
        .subcommand(
            SubCommand::with_name("userprincipals")
            .about("Fetches User Principals")
        )
        .subcommand(
            SubCommand::with_name("quote")
                .about("Retrieve quotes for <symbols>")
                .arg(
                    Arg::with_name("symbols")
                        .takes_value(true)
                        .help("Retrieves quotes of supplied symbols"),
                ),
        )
        .after_help("A valid token must be set in env variable: TDAUTHTOKEN")
        .get_matches();

    let c = initialize_client();

    match matches.subcommand() {
        ("userprincipals", Some(_)) => {
            let resp: String = c.getuserprincipals().execute();
            println!("{}", &resp);
        }
        ("quote", Some(sub_m)) => match sub_m.value_of("symbol") {
            Some(symbols) => {
                let resp: String = c.getquotes(&symbols).execute();
                println!("{}", resp)
            }
            None => println!("Missing symbols"),
        },
        _ => {}
    }
}

fn initialize_client() -> TDAClient {
    let token = env::var("TDAUTHTOKEN").unwrap();
    let c = TDAClient::new(token);
    return c;
}
