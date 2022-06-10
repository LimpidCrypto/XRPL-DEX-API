use serde_json;

struct CurrencyAmount {
    currency: String,
    issuer: String,
    value: String,
}

pub struct NormalizedOffer {
    status: String,
    book_node: String,
    owner_node: String,
    prev_txn_id: String,
    prev_txn_lgr_seq: String,
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
        CurrencyAmount {
            currency: match currency_amount.get("currency") {
                None => "XRP".to_string(),
                _ => currency_amount["currency"].to_string(),
            },
            issuer: match currency_amount.get("issuer") {
                None => "".to_string(),
                _ => currency_amount["issuer"].to_string(),
            },
            value: match currency_amount.get("value") {
                None => currency_amount.to_string(),
                _ => currency_amount["value"].to_string(),
            },
        }
    }
}

fn get_offer_status(offer_object: &str, taker_gets: &CurrencyAmount) -> String {
    if offer_object.starts_with(r#"{"CreatedNode""#) {
        "created".to_string()
    } else if offer_object.starts_with(r#"{"ModifiedNode""#) {
        let taker_gets_value = taker_gets.value.parse::<f32>().unwrap();
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
    let taker_gets_value = taker_gets.value.parse::<f32>().unwrap();
    let taker_pays_value = taker_pays.value.parse::<f32>().unwrap();
    let quality = taker_pays_value / taker_gets_value;
    quality.to_string()
}

fn calculate_unfunded_amounts(owner_funds: &str, taker_gets: &CurrencyAmount, taker_pays: &CurrencyAmount, side: &str) -> String {
    let formatted_owner_funds = owner_funds.parse::<f32>().unwrap();
    let taker_gets_value = taker_gets.value.parse::<f32>().unwrap();
    if taker_gets_value < formatted_owner_funds {
        "".to_string()
    } else if side == "TakerGets" {
        formatted_owner_funds.to_string()
    } else {
        let quality = calculate_quality(taker_gets, taker_pays);
        let calc_taker_pays_value = formatted_owner_funds * quality.parse::<f32>().unwrap();
        calc_taker_pays_value.to_string()
    }
}

impl NormalizedOffer {
    pub fn new(offer_object: String, txn_index: u32, owner_funds: String) -> NormalizedOffer {
        let offer: serde_json::Value = serde_json::from_str(&offer_object).unwrap();
        let book_node = if offer.get("NewFields").is_some() {
            offer["NewFields"]["BookNode"].to_string()
        } else {offer["FinalFields"]["BookNode"].to_string()};
        let owner_node = if offer.get("NewFields").is_some() {
            offer["NewFields"]["OwnerNode"].to_string()
        } else {offer["FinalFields"]["OwnerNode"].to_string()};
        let prev_txn_id = if offer.get("PreviousTxnID").is_some() {
            offer["PreviousTxnID"].to_string()
        } else if offer.get("FinalFields").is_some() {
            offer["FinalFields"]["PreviousTxnID"].to_string()
        } else {
            "".to_string()
        };
        let prev_txn_lgr_seq = if offer.get("PreviousTxnLgrSeq").is_some() {
            offer["PreviousTxnLgrSeq"].to_string()
        } else if offer.get("FinalFields").is_some() {
            offer["FinalFields"]["PreviousTxnLgrSeq"].to_string()
        } else {
            "".to_string()
        };
        let taker_gets = if offer.get("NewFields").is_some() {
            CurrencyAmount::new(&offer["NewFields"]["TakerGets"])
        } else {CurrencyAmount::new(&offer["FinalFields"]["TakerGets"])};
        let taker_pays = if offer.get("NewFields").is_some() {
            CurrencyAmount::new(&offer["NewFields"]["TakerPays"])
        } else {CurrencyAmount::new(&offer["FinalFields"]["TakerPays"])};
        NormalizedOffer {
            status: get_offer_status(&offer_object, &taker_gets),
            book_node,
            owner_node,
            prev_txn_id,
            prev_txn_lgr_seq,
            txn_index: txn_index.to_string(),
            fee: "".to_string(),
            quality: calculate_quality(&taker_gets, &taker_pays),
            taker_gets_funded: calculate_unfunded_amounts(&owner_funds, &taker_gets, &taker_pays, "TakerGets"),
            taker_pays_funded: calculate_unfunded_amounts(&owner_funds, &taker_gets, &taker_pays, "TakerPays"),
            owner_funds,
            taker_gets,
            taker_pays,
        }
    }
}
