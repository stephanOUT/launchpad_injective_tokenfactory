use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw20_base::msg::{
    QueryMsg as cw20BaseQueryMsg
};
use cw20_base::msg::{
    ExecuteMsg as cw20BaseExecuteMsg
};
use cosmwasm_std::{Uint128, Binary};
use cw_utils::Expiration;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub token_name: String,
    pub token_symbol: String,
    pub token_description: String,
    pub token_decimal: u8,
    pub token_logo: String,
    pub token_admin: String,
    pub token_creator: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Transfer {
        recipient: String,
        amount: Uint128,
    },
    Burn {
        amount: Uint128,
    },
    Send {
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    IncreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    DecreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    TransferFrom {
        owner: String,
        recipient: String,
        amount: Uint128,
    },
    SendFrom {
        owner: String,
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    BurnFrom {
        owner: String,
        amount: Uint128,
    },
    Mint {
        recipient: String,
        amount: Uint128,
    },
    // UpdateMinter {
    //     new_minter: Option<String>,
    // },
    UpdateMarketing {
        project: Option<String>,
        description: Option<String>,
        marketing: Option<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryResponses)]
    base_query_msg(cw20BaseQueryMsg),
    #[returns(TokenInfoResponse)]
    QueryTokenInformation {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfoResponse {
    pub name: String,
    pub symbol: String,
    pub decimal: u8,
    pub description: String,
    pub logo: String,
    pub creator: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}