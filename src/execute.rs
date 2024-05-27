use cw20_base::ContractError;

use cosmwasm_std::{
    DepsMut, Env, MessageInfo, Response, Uint128,
    StdResult, Storage, Binary
};
use cw_utils::Expiration;
use cw20_base::msg::{
    ExecuteMsg as cw20BaseExecuteMsg
};

use crate::state::{ TOKENINFO, TokenInfo };

use cw20_base::contract::{
    execute_mint, execute_send, execute_transfer, execute_update_marketing,
    execute_upload_logo, query_balance, query_token_info, query_minter, query_download_logo, query_marketing_info, execute_burn,
};

use cw20_base::allowances::{
    execute_decrease_allowance, execute_increase_allowance, execute_send_from,
    execute_transfer_from, query_allowance, execute_burn_from,
};

pub fn check_admin_only(info: MessageInfo, store: &dyn Storage) -> Result<bool, ContractError> {
    let token_state = TOKENINFO.load(store)?;
    if token_state.token_admin == "default" {
        return Ok(true);
    } else if token_state.token_admin != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }
    return Ok(true);
}

pub fn transfer(
    mut deps: DepsMut, _env: Env, info: MessageInfo, recipient: String, amount: Uint128
    ) -> Result<Response, ContractError> {
    let store = deps.branch().storage;
    check_admin_only(info.clone(), store)?;
    Ok(execute_transfer(deps, _env, info, recipient, amount)?)
}

pub fn burn(
    mut deps: DepsMut, _env: Env, info: MessageInfo, amount: Uint128
    ) -> Result<Response, ContractError> {
        let store = deps.branch().storage;
        check_admin_only(info.clone(), store);
        Ok(execute_burn(deps, _env, info, amount)?)
    }

pub fn send(
    mut deps: DepsMut, _env: Env, info: MessageInfo,contract: String, amount: Uint128, msg: Binary
    ) -> Result<Response, ContractError> {
        let store = deps.branch().storage;
        check_admin_only(info.clone(), store);
        Ok(execute_send(deps, _env, info,contract, amount, msg)?)
    }

pub fn mint(
    mut deps: DepsMut, _env: Env, info: MessageInfo,recipient: String, amount: Uint128
    ) -> Result<Response, ContractError> {
        let store = deps.branch().storage;
        check_admin_only(info.clone(), store);
        Ok(execute_mint(deps, _env, info, recipient, amount)?)
    }
    
pub fn increase_allowance(
    mut deps: DepsMut, _env: Env, info: MessageInfo,spender: String, amount: Uint128, expires: Option<Expiration>
    ) -> Result<Response, ContractError> {
        let store = deps.branch().storage;
        check_admin_only(info.clone(), store);
        Ok(execute_increase_allowance(deps, _env, info, spender, amount, expires)?)
    }

pub fn decrease_allowance(
    mut deps: DepsMut, _env: Env, info: MessageInfo,spender: String, amount: Uint128, expires: Option<Expiration>
    ) -> Result<Response, ContractError> {
        let store = deps.branch().storage;
        check_admin_only(info.clone(), store);
        Ok(execute_decrease_allowance(deps, _env, info, spender, amount, expires)?)
    }


pub fn transfer_from(
    mut deps: DepsMut, _env: Env, info: MessageInfo,owner: String, recipient: String, amount: Uint128
    ) -> Result<Response, ContractError> {
        let store = deps.branch().storage;
        check_admin_only(info.clone(), store);
        Ok(execute_transfer_from(deps, _env, info, owner, recipient, amount)?)
    }

pub fn burn_from(
    mut deps: DepsMut, _env: Env, info: MessageInfo,owner: String, amount: Uint128
    ) -> Result<Response, ContractError> {
        let store = deps.branch().storage;
        check_admin_only(info.clone(), store);
        Ok(execute_burn_from(deps, _env, info, owner, amount)?)
    }

pub fn send_from(
    mut deps: DepsMut, _env: Env, info: MessageInfo,owner: String, contract: String, amount: Uint128, msg: Binary
    ) -> Result<Response, ContractError> {
        let store = deps.branch().storage;
        check_admin_only(info.clone(), store);
        Ok(execute_send_from(deps, _env, info, owner, contract, amount, msg)?)
    }

pub fn update_marketing(
    mut deps: DepsMut, _env: Env, info: MessageInfo,project: Option<String>, description: Option<String>, marketing: Option<String>
    ) -> Result<Response, ContractError> {
        let store = deps.branch().storage;
        check_admin_only(info.clone(), store);
        Ok(execute_update_marketing(deps, _env, info, project, description, marketing)?)
    }

pub fn upload_logo(
    mut deps: DepsMut, _env: Env, info: MessageInfo, logo: cw20::Logo
    ) -> Result<Response, ContractError> {
        let store = deps.branch().storage;
        check_admin_only(info.clone(), store);
        Ok(execute_upload_logo(deps, _env, info, logo)?)
    }
    