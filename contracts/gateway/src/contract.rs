#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, to_json_binary};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::{execute, query};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:gateway";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(execute::init(_deps, _msg)?)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateProjectMsg {
            owner,
            github_addr,
            description,
        } => execute::create_project(deps, env, info, owner, github_addr, description),
        ExecuteMsg::SaveResultMsg {
            user,
            project_id,
            req_id,
            request,
            result,
            ..
        } => execute::save_exec_result(deps, env, info, user, project_id, req_id, request, result),
        ExecuteMsg::ResultRequestMsg { user, id, input } => {
            execute::result_request(deps, env, info, user, id, input)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query::config(deps)?),
        QueryMsg::RequestIDInfo { id } => to_json_binary(&query::request_id(deps, id)?),
        QueryMsg::ProjectInfo { id } => to_json_binary(&query::project_info(deps, id)?),
    }
}

#[cfg(test)]
mod tests {}
