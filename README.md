# XRPL-DEX-API
An API that provides detailed information and metrics of the XRP Ledger's decentralized exchange.
Depending on how big the database would be the API would provide a certain history of [Offer](https://xrpl.org/offer.html#offer) objects.
From those objects (may need to include some additional fields) you could for example derive the following information from them:<br>
### Basic information:<br>
- get_order_book -> get an order book by passing the currency pair
- get_exchange_rate -> get the current exchange rate of an order book
- get_exchange_rate_history -> get the price history of an order book considering some period
- get_current_market_order_taker_offer -> get the offer a taker had to consume, to buy/sell at market price
- get_market_volume -> get a market's volume considering some period
- get_circulating_supply -> amount of tokens that are not in the account of the currencies issuer
- normalize -> converts a currency amount into an other, using the order book's exchange rate<br>
### Advanced information:<br>
- method to get a markets liquidity (bid-ask spread, percentage spread, volume-based)
- method to get a markets efficiency
- method to get a markets share
- method to get amount an issuer has made with transfer fees during some period
- method to get the average trade cost of a market considering some period
- method to get a market's growth considering some period
- method to get a market's free float
- method to get a market's turnover value (also a liquidity measurment)

# Please suggest more potentially useful methods the API should provide and dicuss mine.
