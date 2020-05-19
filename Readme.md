## TDACLI

This is just a very simple CLI wrapper around my tdameritradeclient library.

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