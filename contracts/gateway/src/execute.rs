use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, StdResult};

use msg::InstantiateMsg;
use state::{Config, ExecResult, Project, RequestID, ResultRequest, CONFIG, PROJECT, REQUEST};

use crate::{msg, state};

pub fn init(deps: DepsMut, msg: InstantiateMsg) -> StdResult<Response> {
    let config = Config {
        admin: msg.admin,
        cnt: msg.count,
    };

    let req_id = RequestID {
        request_id: vec![0],
    };

    CONFIG.save(deps.storage, &config)?;
    REQUEST.save(deps.storage, &req_id)?;

    Ok(Response::default())
}

pub fn create_project(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    owner: Addr,
    github_addr: String,
    des: String,
) -> StdResult<Response> {
    let mut config = CONFIG.load(deps.storage)?;
    let mut request_id = REQUEST.load(deps.storage)?;

    let project = Project {
        id: config.cnt + 1,
        owner,
        github_addr,
        description: des,
        request: vec![],
        result: vec![],
    };
    PROJECT.save(deps.storage, project.id, &project)?;

    config.cnt = config.cnt + 1;
    request_id.request_id.push(0);

    CONFIG.save(deps.storage, &config)?;
    REQUEST.save(deps.storage, &request_id)?;

    Ok(Response::new().add_attribute("id", project.id.to_string()))
}

pub fn result_request(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    user: Addr,
    id: i32,
    input: String,
) -> StdResult<Response> {
    let mut project = PROJECT.load(deps.storage, id)?;
    let mut request = REQUEST.load(deps.storage)?;

    let idx = id as usize;
    let req_id = request.request_id[idx] as i32;
    let result = ResultRequest {
        req_id: req_id + 1,
        user,
        input,
    };

    project.request.push(result);
    request.request_id[idx] = req_id + 1;

    PROJECT.save(deps.storage, id, &project)?;
    REQUEST.save(deps.storage, &request)?;

    Ok(Response::default())
}

pub fn save_exec_result(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    user: Addr,
    project_id: i32,
    req_id: i32,
    request: String,
    result: String,
) -> StdResult<Response> {
    let mut project = PROJECT.load(deps.storage, project_id)?;

    let result = ExecResult {
        req_id,
        user,
        request,
        result,
    };

    project.result.push(result);

    if let Some(pos) = project.request.iter().position(|x| x.req_id == req_id) {
        project.request.remove(pos);
    }

    PROJECT.save(deps.storage, project_id, &project)?;
    Ok(Response::default())
}
