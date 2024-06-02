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
    pub token_initialmint: Uint128,
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
    SendSellFee {
        recipient: String,
        amount: Uint128
    },
    SellToken {
        recipient: String,
        amount: Uint128
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TokenInfoResponse)]

    QueryTokenInformation {},
    #[returns(cw20::BalanceResponse)]
    Balance { address: String },
    /// Returns metadata on the contract - name, decimals, supply, etc.
    #[returns(cw20::TokenInfoResponse)]
    TokenInfo {},
    /// Only with "mintable" extension.
    /// Returns who can mint and the hard cap on maximum tokens after minting.
    #[returns(cw20::MinterResponse)]
    Minter {},
    /// Only with "allowance" extension.
    /// Returns how much spender can use from owner account, 0 if unset.
    #[returns(cw20::AllowanceResponse)]
    Allowance { owner: String, spender: String },
    /// Only with "enumerable" extension (and "allowances")
    /// Returns all allowances this owner has approved. Supports pagination.
    // #[returns(cw20::AllAllowancesResponse)]
    // AllAllowances {
    //     owner: String,
    //     start_after: Option<String>,
    //     limit: Option<u32>,
    // },
    // /// Only with "enumerable" extension (and "allowances")
    // /// Returns all allowances this spender has been granted. Supports pagination.
    // #[returns(cw20::AllSpenderAllowancesResponse)]
    // AllSpenderAllowances {
    //     spender: String,
    //     start_after: Option<String>,
    //     limit: Option<u32>,
    // },
    /// Only with "enumerable" extension
    /// Returns all accounts that have balances. Supports pagination.
    #[returns(cw20::AllAccountsResponse)]
    AllAccounts {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Only with "marketing" extension
    /// Returns more metadata on the contract to display in the client:
    /// - description, logo, project url, etc.
    #[returns(cw20::MarketingInfoResponse)]
    MarketingInfo {},
    /// Only with "marketing" extension
    /// Downloads the embedded logo data (if stored on chain). Errors if no logo data is stored for this
    /// contract.
    #[returns(cw20::DownloadLogoResponse)]
    DownloadLogo {},
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