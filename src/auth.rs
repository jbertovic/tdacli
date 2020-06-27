use clap::ArgMatches;
use tdameritradeclient::auth::getcodeweblink;

pub fn weblink(args: &ArgMatches) {
    println!("{}", 
        getcodeweblink(args.value_of("clientid").unwrap(), args.value_of("redirect").unwrap()));
}

pub fn auth(args: &ArgMatches) {
    println!("{}", 
        getcodeweblink(args.value_of("clientid").unwrap(), args.value_of("redirect").unwrap()));
}