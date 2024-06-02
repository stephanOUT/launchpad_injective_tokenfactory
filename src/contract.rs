#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128
};
use cw20_base::ContractError;
use cw20_base::enumerable::{query_owner_allowances, query_spender_allowances, query_all_accounts};
use crate::msg::QueryMsg;

use cw20_base::contract::{query as cw20BaseQuery};
use crate::msg::MigrateMsg;
use cw2::set_contract_version;

use crate::msg::InstantiateMsg;
use crate::state::{ TokenInfo, TOKENINFO};
use cw20_base::msg::{ InstantiateMsg as cw20BaseInstantiateMsg };
use cw20_base::msg::{ QueryMsg as cw20BaseQueryMsg };
use cw20_base::allowances::{
    query_allowance
};
use cw20_base::contract:: {
    query_balance, query_token_info, query_minter, query_marketing_info, query_download_logo
};

use crate::msg::ExecuteMsg;
use crate::msg::QueryMsg::{ QueryTokenInformation };
use crate::query::{ query_token_information };

use crate::execute::{
    transfer, burn, send, mint, increase_allowance, decrease_allowance,
    transfer_from, burn_from, send_from, update_marketing, upload_logo,
    send_sell_fee, send_sell
};

// version info for migration info
const CONTRACT_NAME: &str = "Out-Token";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let token_info: TokenInfo = TokenInfo {
        token_name: msg.token_name.clone(),
        token_symbol: msg.token_symbol.clone(),
        token_description: msg.token_description.clone(),
        token_decimal: msg.token_decimal.clone(),
        token_logo: msg.token_logo.clone(),
        token_admin: msg.token_admin.clone(),
        token_creator: msg.token_creator.clone()
    };

    TOKENINFO.save(deps.storage, &token_info)?;

    /* Execute the instantiate method from cw_20_base as the code from that
    library is already battle tested we do not have to re-write the full
    functionality: https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw20-base*/
    Ok(cw20_base::contract::instantiate(deps, env, info.clone(), cw20BaseInstantiateMsg {
        name: msg.token_name.clone(),
        symbol: msg.token_symbol.clone(),
        decimals: msg.token_decimal,
        initial_balances: [
            cw20::Cw20Coin {
                address: info.sender.clone().to_string(),
                amount: msg.token_initialmint
            }
        ].to_vec(),
        mint: None,
        marketing: None
    })?)
}
#[allow(unreachable_patterns)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer { recipient, amount } => transfer(deps, env, info, recipient, amount),
        ExecuteMsg::Burn { amount } => burn(deps, env, info, amount),
        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => send(deps, env, info, contract, amount, msg),
        ExecuteMsg::Mint { recipient, amount } => mint(deps, env, info, recipient, amount),
        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires,
        } => increase_allowance(deps, env, info, spender, amount, expires),
        ExecuteMsg::DecreaseAllowance {
            spender,
            amount,
            expires,
        } => decrease_allowance(deps, env, info, spender, amount, expires),
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => transfer_from(deps, env, info, owner, recipient, amount),
        ExecuteMsg::BurnFrom { owner, amount } => burn_from(deps, env, info, owner, amount),
        ExecuteMsg::SendFrom {
            owner,
            contract,
            amount,
            msg,
        } => send_from(deps, env, info, owner, contract, amount, msg),
        ExecuteMsg::UpdateMarketing {
            project,
            description,
            marketing,
        } => update_marketing(deps, env, info, project, description, marketing),
        ExecuteMsg::SendSellFee {
            recipient,
            amount
        } => send_sell_fee(deps, env, info, recipient, amount),
        ExecuteMsg::SellToken {
            recipient,
            amount
        } => send_sell(deps, env, info, recipient, amount)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // QueryMsg::base_query_msg(cw20_base_msg) => cw20BaseQuery(deps, _env, cw20_base_msg),
        QueryMsg::QueryTokenInformation {} => query_token_information(deps),
        /* Default methods from CW20 Standard with no modifications:
        https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw20-base */
        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::TokenInfo {} => to_binary(&query_token_info(deps)?),
        QueryMsg::Minter {} => to_binary(&query_minter(deps)?),
        QueryMsg::Allowance { owner, spender } => {
            to_binary(&query_allowance(deps, owner, spender)?)
        }
        // cw20BaseQueryMsg::AllAllowances {
        //     owner,
        //     start_after,
        //     limit,
        // } => to_binary(&query_all_allowances(deps, owner, start_after, limit)?),
        QueryMsg::AllAccounts { start_after, limit } => {
            to_binary(&query_all_accounts(deps, start_after, limit)?)
        }
        QueryMsg::MarketingInfo {} => to_binary(&query_marketing_info(deps)?),
        QueryMsg::DownloadLogo {} => to_binary(&query_download_logo(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
