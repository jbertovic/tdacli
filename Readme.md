## TDACLI

A simple CLI wrapper around my [tdameritradeclient](https://github.com/jbertovic/tdameritradeclient) library. For help enter `tdacli --help`.  This will give you access to help and further subcommand help categories.

Current subcommands are: userprincipals, account, quote, history, optionchain, instrument, transaction, auth, refresh, weblink. See below for description on output of `tdaci --help` in the CLI Commands section

Environmental Variable Requirements:
- `TDAUTHTOKEN` on subcommands: account, history, optionchain, quote, userprincipals, watchlist
- `TDREFRESHTOKEN` on subcommands: refresh
- `TDCODE` on subcommand: auth
- No Env Variable on subcommand: weblink

## Example

an example usage piped into jq running on linux.  Assumes `TDAUTHTOKEN` env variable holds valid token.

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

## Authorization from scratch

### 1) Fetch code using weblink
Use the `weblink` subcommand along with registered app on developer.tdameritrade.com to get an authorization link. When you register an app you will have a `consumer_key` (referred through out as clientid) and a `redirect_uri`. I recommend using a redirecturi that is your localhost (`https://127.0.0.1:8080/`).  This way you can copy the returned code directly from the query bar in the browser.  The below Consumer Key is only an example.  

```
> tdacli weblink J3ROAVSNNFTLC9RJE4BD2DO2WJABCDEF https://127.0.0.1:8080/
https://auth.tdameritrade.com/auth?response_type=code&redirect_uri=https%3A%2F%2F127.0.0.1%3A8080%2F&client_id=J3ROAVSNNFTLC9RJE4BD2DO2WJABCDEF%40AMER.OAUTHAP

```

Visit this link and authorize your access directly with TDameritrade.  You will receive the code in the query bar.  Only copy the encoded string after `code=`.  Assign this to the environment variable `TDCODE`.



### 2) Use code to retrieve token and refresh_token

From step 1 above with `TDCODE` env variable assigned.

```
> tdacli auth J3ROAVSNNFTLC9RJE4BD2DO2WJABCDEF https://127.0.0.1:8080/
{"Token": "UZfU1e9dhw9zdq26+vJa1tERNhfaa4lFXqgfzeVW+HOEdRKHSDvnq0lepMd.....long string truncated",
"Refresh": "a1tERNhfaa4lFXqgfzeVW+HOEdRKHSDvnq0lepMdvJa1tERNhfa....long string truncated"}
```

Assign the Refresh token to TDREFRESHTOKEN and the Token to TDAUTHTOKEN.  You can go ahead and run any of the other subcommands with TDAUTHTOKEN set.

A refresh token lasts for 90 days and can be used to get a valid token to access tdameritrade API.  The token is only valid for 30 min before expiring.  Reuse the same or new refresh token to renew.


### 3) Use refresh to maintain an active token

Assumes REFRESHTOKEN is set and valid.

```
> tdacli refresh J3ROAVSNNFTLC9RJE4BD2DO2WJABCDEF
zdq26+vJaNhfaa4lFXqgfzeVW+HOEdRKHSDzdq26+vJa1tERNhfaa4lFXqgfzeVW+HOEdRKHSDzdq26+vJa1tERNhfaa4...long string truncated
```

Use the returned value to set TDAUTHTOKEN.

In bash you can automate this with:
```
export TDAUTHTOKEN=$(tdacli refresh J3ROAVSNNFTLC9RJE4BD2DO2WJABCDEF)
```

Not sure how to do this in windows?  Please DM me if you know.

## CLI Commands

### Output of main help

```
> tdacli --help
TDAmeritrade API CLI 0.3.0
Command Line Interface into tdameritradeclient rust library

USAGE:
    tdacli [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    account           Retrieve account information for <account_id>
    auth              Retrieves refresh_token using authorization_code grant type
    help              Prints this message or the help of the given subcommand(s)
    history           Retrieve history for one <symbol>.
    instrument        Retrieve instrument information or search for instrument
    optionchain       Retrieve option chain for one <symbol>
    quote             Retrieve quotes for requested symbols
    refresh           Fetch valid token or renew refresh_token using a refresh_token grant type
    transaction       Retrieve transaction history
    userprincipals    Retrieves User Principals
    watchlist         Retrieve Watchlist
    weblink           Gives you the url to get authorization code from TDAmeritrade

Check env variable requirements for each subcommand.
Token can be retrieved using 'refresh' subcommand if you have a valid refresh_token.
Token can also be issued by using 'weblink' subcommand first to retrieve 'authorization_code'
and 'auth' subcommand to issue new token and refresh_token.
'*' indicates default value in subcommand help information.
```

### history help
Included one help output of subcommand as an example.  All subcommands have `--help` option to help understand usage.

```
> tdacli history --help
tdacli-history
Retrieve history for one <symbol>.

USAGE:
    tdacli history [OPTIONS] <symbol> [ARGS]

FLAGS:
    -h, --help    Prints help information

OPTIONS:
        --freq <freq>            Defines number of freq_types to show: minute: 1*,5,10,15,30 | daily: 1* | weekly: 1* |
                                 monthly: 1*
        --ftype <freq_type>      Defines freq of new candle by period_type: day: minute* | month: daily, weekly* | year:
                                 daily, weekly, monthly* | ytd: daily, weekly*
        --period <period>        Defines number of periods to show; day: 1,2,3,4,5,10* | month: 1*,2,3,6 | year:
                                 1*,2,3,5,10,15,20 | ytd: 1*
        --ptype <period_type>    Defines period type: day, month, year, ytd

ARGS:
    <symbol>       Symbol of instrument.
    <startdate>    Defines start date in epoch format. <period> should not be provided
    <enddate>      Defines end date epoch format. Default is previous trading day.

Think of the frequency as the size of a candle on the chart or how to divide the ticks.
and the period as the term or total length of the history.
'*' indicates default value.
```


### TODO

- [ ] add option for date calculations on History epoch data stamps
- [ ] Consider adding date conversion using chrono package
- [ ] orders - a way to handle orders with library adding and deleting
