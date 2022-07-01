use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
struct CurrencyAmount {
    currency: String,
    issuer: String,
    value: String,
}

#[derive(Debug)]
pub struct NormalizedOffer {
    status: String,
    account: String,
    book_node: String,
    owner_node: String,
    prev_txn_id: String,
    prev_txn_lgr_seq: String,
    date: String,
    expiration: String,
    txn_index: String,
    fee: String,
    quality: String,
    taker_gets_funded: String,
    taker_pays_funded: String,
    owner_funds: String,
    taker_gets: CurrencyAmount,
    taker_pays: CurrencyAmount,
}

impl CurrencyAmount {
    pub fn new(currency_amount: &serde_json::Value) -> CurrencyAmount {
        let currency = currency_amount.get("currency");
        let issuer = currency_amount.get("issuer");
        let value = currency_amount.get("value");
        CurrencyAmount {
            currency: if let Some(..) = currency {
                String::from(currency.unwrap().as_str().unwrap())
            } else {
                String::from("XRP")
            },
            issuer: if let Some(..) = issuer {
                String::from(issuer.unwrap().as_str().unwrap())
            } else {
                String::from("")
            },
            value: if let Some(..) = value {
                String::from(value.unwrap().as_str().unwrap())
            } else {
                String::from(currency_amount.as_str().unwrap())
            },
        }
    }
}

fn get_offer_status(offer_object: &Value, taker_gets: &CurrencyAmount) -> String {
    if offer_object.get("CreatedNode").is_some() {
        "created".to_string()
    } else if offer_object.get("ModifiedNode").is_some() {
        let taker_gets_value = taker_gets.value.as_str().parse::<f32>().unwrap();
        if taker_gets_value > 0.0 {
            "partially-filled".to_string()
        } else {
            "filled".to_string()
        }
    } else {
        "cancelled".to_string()
    }
}

fn calculate_quality(taker_gets: &CurrencyAmount, taker_pays: &CurrencyAmount) -> String {
    let taker_gets_value = String::from(&taker_gets.value).parse::<f32>().unwrap();
    let taker_pays_value = String::from(&taker_pays.value).parse::<f32>().unwrap();
    let quality = taker_pays_value / taker_gets_value;
    quality.to_string()
}

fn calculate_unfunded_amounts(
    owner_funds: &str,
    taker_gets: &CurrencyAmount,
    taker_pays: &CurrencyAmount,
    side: &str,
) -> String {
    let formatted_owner_funds = owner_funds.parse::<f32>().unwrap();
    let taker_gets_value = taker_gets.value.as_str().parse::<f32>().unwrap();
    if taker_gets_value < formatted_owner_funds {
        "".to_string()
    } else if side == "TakerGets" {
        formatted_owner_funds.to_string()
    } else {
        let quality = calculate_quality(taker_gets, taker_pays);
        let calc_taker_pays_value =
            formatted_owner_funds * quality.as_str().parse::<f32>().unwrap();
        calc_taker_pays_value.to_string()
    }
}

impl NormalizedOffer {
    pub fn new(
        offer: &Value,
        account: &str,
        date: &i64,
        node_type: &str,
        txn_index: u64,
        owner_funds: String,
        hash: String,
        ledger_index: &i64,
    ) -> NormalizedOffer {
        // let offer: serde_json::Value = serde_json::from_str(&offer_object).unwrap();
        let mut book_node = if offer[node_type].get("NewFields").is_some() {
            offer[node_type]["NewFields"]["BookNode"]
                .to_string()
                .replace('\"', "")
        } else {
            offer[node_type]["FinalFields"]["BookNode"]
                .to_string()
                .replace('\"', "")
        };
        if book_node == "null" {
            book_node = "0".to_string()
        }
        let mut owner_node = if offer[node_type].get("NewFields").is_some() {
            offer[node_type]["NewFields"]["OwnerNode"]
                .to_string()
                .replace('\"', "")
        } else {
            offer[node_type]["FinalFields"]["OwnerNode"]
                .to_string()
                .replace('\"', "")
        };
        if owner_node == "null" {
            owner_node = "0".to_string()
        }
        let prev_txn_id = if offer[node_type].get("PreviousTxnID").is_some() {
            offer[node_type]["PreviousTxnID"]
                .to_string()
                .replace('\"', "")
        } else if offer[node_type].get("FinalFields").is_some() {
            offer[node_type]["FinalFields"]["PreviousTxnID"]
                .to_string()
                .replace('\"', "")
        } else {
            hash
        };
        let prev_txn_lgr_seq = if offer[node_type].get("PreviousTxnLgrSeq").is_some() {
            offer[node_type]["PreviousTxnLgrSeq"]
                .to_string()
                .replace('\"', "")
        } else if offer[node_type].get("FinalFields").is_some() {
            offer[node_type]["FinalFields"]["PreviousTxnLgrSeq"]
                .to_string()
                .replace('\"', "")
        } else {
            ledger_index.to_string()
        };
        let mut expiration = if offer[node_type].get("NewFields").is_some() {
            offer[node_type]["NewFields"]["Expiration"]
                .to_string()
                .replace('\"', "")
        } else {
            offer[node_type]["FinalFields"]["Expiration"]
                .to_string()
                .replace('\"', "")
        };
        if expiration == "null" {
            expiration = "".to_string()
        }
        let taker_gets = if offer[node_type].get("NewFields").is_some() {
            CurrencyAmount::new(&offer[node_type]["NewFields"]["TakerGets"])
        } else {
            CurrencyAmount::new(&offer[node_type]["FinalFields"]["TakerGets"])
        };
        let taker_pays = if offer[node_type].get("NewFields").is_some() {
            CurrencyAmount::new(&offer[node_type]["NewFields"]["TakerPays"])
        } else {
            CurrencyAmount::new(&offer[node_type]["FinalFields"]["TakerPays"])
        };
        NormalizedOffer {
            status: get_offer_status(offer, &taker_gets),
            account: account.to_string(),
            book_node,
            owner_node,
            prev_txn_id,
            prev_txn_lgr_seq,
            date: date.to_string(),
            expiration,
            txn_index: txn_index.to_string(),
            fee: "".to_string(), // TODO: fn calculate_fee
            quality: calculate_quality(&taker_gets, &taker_pays),
            taker_gets_funded: if !owner_funds.is_empty() {
                calculate_unfunded_amounts(&owner_funds, &taker_gets, &taker_pays, "TakerGets")
            } else {
                "".to_string()
            },
            taker_pays_funded: if !owner_funds.is_empty() {
                calculate_unfunded_amounts(&owner_funds, &taker_gets, &taker_pays, "TakerPays")
            } else {
                "".to_string()
            },
            owner_funds,
            taker_gets,
            taker_pays,
        }
    }
}

pub fn normalize_offers(txn: HashMap<String, Value>) {
    if !txn.contains_key("meta") {
        return;
    }
    let meta = &txn["meta"];
    let transaction = &txn["transaction"];
    let txn_account = &transaction["Account"];
    let affected_nodes = &meta["AffectedNodes"];
    for node in affected_nodes.as_array().unwrap() {
        let mut node_type = String::new();
        if node.get("CreatedNode").is_some() {
            node_type.replace_range(.., "CreatedNode");
        } else if node.get("ModifiedNode").is_some() {
            node_type.replace_range(.., "ModifiedNode");
        } else {
            node_type.replace_range(.., "DeletedNode");
        }
        if node[&node_type]["LedgerEntryType"] == "Offer" {
            let owner_funds_opt = transaction.get("owner_funds");
            let mut owner_funds = String::new();
            let date = &transaction["date"].as_i64().unwrap();
            let hash = transaction["hash"].as_str().unwrap();
            let ledger_index = &txn["ledger_index"].as_i64().unwrap();
            let offer_account = if node[&node_type].get("NewFields").is_some() {
                node[&node_type]["NewFields"]["Account"].as_str().unwrap()
            } else {
                node[&node_type]["FinalFields"]["Account"].as_str().unwrap()
            };
            if owner_funds_opt.is_some() && txn_account == offer_account {
                owner_funds = String::from(owner_funds_opt.unwrap().as_str().unwrap());
            }
            let txn_index = meta["TransactionIndex"].as_u64().unwrap();
            let normalized_offer = NormalizedOffer::new(
                node,
                offer_account,
                date,
                &node_type,
                txn_index,
                owner_funds,
                hash.to_string(),
                ledger_index,
            );
            println!("{:?}\n", normalized_offer)
        }
    }
}
