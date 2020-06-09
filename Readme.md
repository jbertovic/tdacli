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

### TODO

- [ ] finish off History and OptionChain subcommands
- [ ] add option for date calculations on History epoch data stamps
- [ ] Consider adding date conversion using chrono package
- [ ] once orders are complete on [TDAClient](https://github.com/jbertovic/tdameritradeclient) than to new subcommand
