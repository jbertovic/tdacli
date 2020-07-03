## TDACLI

A simple CLI wrapper around my tdameritradeclient library.  You will need to set the token in an environment variable called `TDAUTHTOKEN`.  You can go on [developer.tdameritrade.com](http://developer.tdameritrade.com) to see how to manually create a valid token.

For help enter `tdacli --help`.  This will give you access to help and further subcommands.

Current subcommands are: quote, history, optionchain, userprincipals, account.


## Example

an example usage piped into jq running on linux


```
> tdacli quote SPY,INTC,VNQ | jq '.[] | {symbol, mark, bidPrice, askPrice}'
{
  "symbol": "SPY",
  "mark": 294.43,
  "bidPrice": 294.41,
  "askPrice": 294.43
}
{
  "symbol": "INTC",
  "mark": 60.01,
  "bidPrice": 60.01,
  "askPrice": 60.13
}
{
  "symbol": "VNQ",
  "mark": 72.87,
  "bidPrice": 71.87,
  "askPrice": 72.88
}
```

## CLI commands

```
>tdacli --help
TDAmeritrade API CLI 0.1.0
Command Line Interface into tdameritradeclient rust library

USAGE:
    tdacli [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    account           Retrieve account information for <account_id>
    help              Prints this message or the help of the given subcommand(s)
    history           Retrieve history for one <symbol>.
    optionchain       Retrieve option chain for one <symbol>
    quote             Retrieve quotes for requested symbols
    userprincipals    Retrieves User Principals

A valid token must be set in env variable: TDAUTHTOKEN.
'*' indicates default value in subcommand help information.
```


### TODO

- [ ] add option for date calculations on History epoch data stamps
- [ ] Consider adding date conversion using chrono package
- [ ] orders - adding, deleting, listing
