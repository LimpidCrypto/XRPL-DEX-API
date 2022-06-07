use std::string::String;

pub struct NormalizedOffer {
    diff_type: String,
    identifiers: (String, String),
    account: String,
    book_directory: String,
    book_node: String,
    flags: u32,
    owner_node: String,
    previous_txn_id: String,
    previous_txn_lgr_seq: u32,
    sequence: u32,
    taker_gets: CurrencyAmount,
    taker_pays: CurrencyAmount,
    fee: String,
    index: String,
    quality: String,
    ledger_entry_type: String,
    owner_funds: String,
    taker_gets_funded: String,
    taker_pays_funded: String,
}

struct CurrencyAmount {
    currency: String,
    issuer: String,
    amount: String,
}
