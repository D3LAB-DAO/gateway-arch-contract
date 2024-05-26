use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{ExecResult, ResultRequest};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Addr,
    pub count: i32,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateProjectMsg {
        owner: Addr,
        github_addr: String,
        description: String,
    },

    ResultRequestMsg {
        user: Addr,
        id: i32,
        input: String,
    },

    SaveResultMsg {
        project_id: i32,
        user: Addr,
        request: String,
        req_id: i32,
        result: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
    #[returns(RequestIDResponse)]
    RequestIDInfo { id: i32 },
    #[returns(ProjectResponse)]
    ProjectInfo { id: i32 },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub count: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProjectResponse {
    pub id: i32,
    pub github_addr: String,
    pub description: String,
    pub owner: Addr,
    pub request: Vec<ResultRequest>,
    pub result: Vec<ExecResult>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ResultRequestMsg {
    pub id: i32,
    pub user_addr: Addr,
    pub input: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SaveResultMsg {
    pub id: i32,
    pub result: String,
    pub user_addr: Addr,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RequestIDResponse {
    pub project_id: i32,
    pub req_id: i32,
}
