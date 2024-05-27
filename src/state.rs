use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo {
    pub token_name: String, // nft contract address
    pub token_symbol: String,
    pub token_description: String,
    pub token_decimal: u8,
    pub token_logo: String,
    pub token_admin: String,
    pub token_creator: String,
}

pub const TOKENINFO_KEY: &str = "Token_Info";
pub const TOKENINFO: Item<TokenInfo> = Item::new(TOKENINFO_KEY);