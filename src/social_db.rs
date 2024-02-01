use near_sdk::{ext_contract, AccountId, env};
use near_sdk::serde_json::Value;
use near_sdk::serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SetReturnType {
    pub block_height: near_sdk::json_types::U64,
}

#[ext_contract(ext_social_db)]
pub trait SocialDB {
    fn set(&mut self, data: Value) -> Value;
}

pub fn social_db_contract() -> ext_social_db::SocialDBExt {
    let social_db: AccountId = if env::current_account_id().to_string().ends_with("testnet") {
        "v1.social08.testnet"
    } else {
        "social.near"
    }
    .parse()
    .unwrap();
    ext_social_db::ext(social_db)
}
