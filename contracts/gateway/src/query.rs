use cosmwasm_std::{Deps, StdResult};

use state::CONFIG;

use crate::msg::{Config, ProjectResponse, RequestIDResponse};
use crate::state;
use crate::state::{PROJECT, REQUEST};

pub fn config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;
    let resp = Config {
        owner: (config.admin),
        count: config.cnt,
    };
    Ok(resp)
}

pub fn request_id(deps: Deps, id: i32) -> StdResult<RequestIDResponse> {
    let req = REQUEST.load(deps.storage)?;

    let idx = id as usize;
    let resp = RequestIDResponse {
        project_id: id,
        req_id: req.request_id[idx],
    };

    Ok(resp)
}

pub fn project_info(deps: Deps, id: i32) -> StdResult<ProjectResponse> {
    let project = PROJECT.load(deps.storage, id)?;

    let resp = ProjectResponse {
        id: project.id,
        github_addr: project.github_addr,
        description: project.description,
        owner: project.owner,
        request: project.request,
        result: project.result,
    };
    Ok(resp)
}
