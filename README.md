# XRPL-DEX-API
An API that provides detailed information and metrics of the XRP Ledger's decentralized exchange.

Depending on how big the database would be the API would provide a certain history of [Offer](https://xrpl.org/offer.html#offer) objects.

### The API would provide basic information like:<br>
- get_order_book -> get an order book by passing the currency pair
- get_exchange_rate -> get the current exchange rate of an order book
- get_exchange_rate_history -> get the price history of an order book considering a period
- get_current_market_order_taker_offer -> get the offer a taker had to consume, to buy/sell at market price
- get_market_volume -> get a market's volume considering a period
- get_circulating_supply ->
- normalize -> converts a currency amount into an other, using the order book's exchange rate<br>
### The API could also provide advanced information like:<br>
- method to get a markets liquidity (bid-ask spread, percentage spread, volume-based)
- method to get a markets efficiency
- method to get a markets share
- method to get amount an issuer has made with transfer fees during a time period
- method to get the average trade cost of a market considering a period
- method to get a market's growth considering a period
- method to get a market's free float
- method to get a market's turnover value (also a liquidity measurment)
