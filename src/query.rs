use cosmwasm_std::{to_binary, Deps, StdError};
use cosmwasm_std::{ QueryResponse};

use crate::state::{TokenInfo, TOKENINFO};
use crate::msg::{TokenInfoResponse};


pub fn query_token_information(deps: Deps) -> Result<QueryResponse, StdError> {
    let token_info = TOKENINFO.load(deps.storage)?;
    Ok(to_binary(&TokenInfoResponse {
        name: token_info.token_name.clone(),
        symbol: token_info.token_symbol.clone(),
        decimal: token_info.token_decimal,
        description: token_info.token_description.clone(),
        logo: token_info.token_logo.clone(),
        creator: token_info.token_creator.clone(),
    })?)
}