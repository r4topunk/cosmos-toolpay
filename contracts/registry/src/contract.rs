use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ToolResponse};
use crate::state::{ToolMeta, TOOLS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:registry";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Default denomination for backward compatibility
const DEFAULT_DENOM: &str = "untrn";

/// Validates endpoint URL format and length
fn validate_endpoint(endpoint: &str) -> Result<(), ContractError> {
    // Check length constraint (≤ 512 characters)
    if endpoint.len() > 512 {
        return Err(ContractError::EndpointTooLong {});
    }
    
    // Check format constraint (must start with https://)
    if !endpoint.starts_with("https://") {
        return Err(ContractError::InvalidEndpointFormat {});
    }
    
    Ok(())
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Set contract version for migration info
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    // Return success response
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisterTool { tool_id, price, denom, description, endpoint } => 
            execute_register_tool(deps, info, tool_id, price, denom, description, endpoint),
        ExecuteMsg::UpdatePrice { tool_id, price } => 
            execute_update_price(deps, info, tool_id, price),
        ExecuteMsg::PauseTool { tool_id } => 
            execute_pause_tool(deps, info, tool_id),
        ExecuteMsg::ResumeTool { tool_id } => 
            execute_resume_tool(deps, info, tool_id),
        ExecuteMsg::UpdateDenom { tool_id, denom } => 
            execute_update_denom(deps, info, tool_id, denom),
        ExecuteMsg::UpdateEndpoint { tool_id, endpoint } => 
            execute_update_endpoint(deps, info, tool_id, endpoint),
    }
}

// RegisterTool handler implementation
pub fn execute_register_tool(
    deps: DepsMut,
    info: MessageInfo,
    tool_id: String,
    price: cosmwasm_std::Uint128,
    denom: Option<String>,
    description: String,
    endpoint: String,
) -> Result<Response, ContractError> {
    // Validate tool_id length ≤ 16 characters
    if tool_id.len() > 16 {
        return Err(ContractError::ToolIdTooLong {});
    }
    
    // Validate description length ≤ 256 characters
    if description.len() > 256 {
        return Err(ContractError::DescriptionTooLong {});
    }
    
    // Validate endpoint
    validate_endpoint(&endpoint)?;
    
    // Store provider address from info.sender
    let provider = info.sender;
    
    // Use provided denom or default to "untrn"
    let denom = denom.unwrap_or_else(|| DEFAULT_DENOM.to_string());
    
    // Store tool metadata in TOOLS map
    let tool = ToolMeta {
        provider: provider.clone(),
        price,
        denom: denom.clone(),
        is_active: true,
        description: description.clone(),
        endpoint: endpoint.clone(),
    };
    
    TOOLS.save(deps.storage, &tool_id, &tool)?;
    
    // Return success response with tool_id
    Ok(Response::new()
        .add_attribute("method", "register_tool")
        .add_attribute("tool_id", tool_id)
        .add_attribute("provider", provider.to_string())
        .add_attribute("price", price.to_string())
        .add_attribute("denom", denom)
        .add_attribute("is_active", "true")
        .add_attribute("description", description)
        .add_attribute("endpoint", endpoint))
}

// UpdatePrice handler implementation
pub fn execute_update_price(
    deps: DepsMut,
    info: MessageInfo,
    tool_id: String,
    price: cosmwasm_std::Uint128,
) -> Result<Response, ContractError> {
    // Load existing tool
    let mut tool = TOOLS.may_load(deps.storage, &tool_id)?
        .ok_or(ContractError::ToolNotFound {})?;
    
    // Verify sender is the provider
    if info.sender != tool.provider {
        return Err(ContractError::Unauthorized {});
    }
    
    // Update price and save
    tool.price = price;
    TOOLS.save(deps.storage, &tool_id, &tool)?;
    
    Ok(Response::new()
        .add_attribute("method", "update_price")
        .add_attribute("tool_id", tool_id)
        .add_attribute("new_price", price.to_string()))
}

// PauseTool handler implementation
pub fn execute_pause_tool(
    deps: DepsMut,
    info: MessageInfo,
    tool_id: String,
) -> Result<Response, ContractError> {
    // Load existing tool
    let mut tool = TOOLS.may_load(deps.storage, &tool_id)?
        .ok_or(ContractError::ToolNotFound {})?;
    
    // Verify sender is the provider
    if info.sender != tool.provider {
        return Err(ContractError::Unauthorized {});
    }
    
    // Set is_active to false
    tool.is_active = false;
    TOOLS.save(deps.storage, &tool_id, &tool)?;
    
    Ok(Response::new()
        .add_attribute("method", "pause_tool")
        .add_attribute("tool_id", tool_id))
}

// ResumeTool handler implementation
pub fn execute_resume_tool(
    deps: DepsMut,
    info: MessageInfo,
    tool_id: String,
) -> Result<Response, ContractError> {
    // Load existing tool
    let mut tool = TOOLS.may_load(deps.storage, &tool_id)?
        .ok_or(ContractError::ToolNotFound {})?;
    
    // Verify sender is the provider
    if info.sender != tool.provider {
        return Err(ContractError::Unauthorized {});
    }
    
    // Set is_active to true
    tool.is_active = true;
    TOOLS.save(deps.storage, &tool_id, &tool)?;
    
    Ok(Response::new()
        .add_attribute("method", "resume_tool")
        .add_attribute("tool_id", tool_id))
}

// UpdateDenom handler implementation
pub fn execute_update_denom(
    deps: DepsMut,
    info: MessageInfo,
    tool_id: String,
    denom: String,
) -> Result<Response, ContractError> {
    // Load existing tool
    let mut tool = TOOLS.may_load(deps.storage, &tool_id)?
        .ok_or(ContractError::ToolNotFound {})?;
    
    // Verify sender is the provider
    if info.sender != tool.provider {
        return Err(ContractError::Unauthorized {});
    }
    
    // Update denom and save
    tool.denom = denom.clone();
    TOOLS.save(deps.storage, &tool_id, &tool)?;
    
    Ok(Response::new()
        .add_attribute("method", "update_denom")
        .add_attribute("tool_id", tool_id)
        .add_attribute("new_denom", denom))
}

// UpdateEndpoint handler implementation
pub fn execute_update_endpoint(
    deps: DepsMut,
    info: MessageInfo,
    tool_id: String,
    endpoint: String,
) -> Result<Response, ContractError> {
    // Validate endpoint
    validate_endpoint(&endpoint)?;
    
    // Load existing tool
    let mut tool = TOOLS.may_load(deps.storage, &tool_id)?
        .ok_or(ContractError::ToolNotFound {})?;
    
    // Verify sender is the provider
    if info.sender != tool.provider {
        return Err(ContractError::Unauthorized {});
    }
    
    // Update endpoint and save
    tool.endpoint = endpoint.clone();
    TOOLS.save(deps.storage, &tool_id, &tool)?;
    
    Ok(Response::new()
        .add_attribute("method", "update_endpoint")
        .add_attribute("tool_id", tool_id)
        .add_attribute("new_endpoint", endpoint))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTool { tool_id } => query_tool(deps, tool_id),
        QueryMsg::GetTools {} => query_all_tools(deps),
    }
}

// GetTool query implementation
pub fn query_tool(deps: Deps, tool_id: String) -> StdResult<Binary> {
    let tool = TOOLS.may_load(deps.storage, &tool_id)?;
    
    match tool {
        Some(tool_meta) => {
            let response = ToolResponse {
                tool_id,
                provider: tool_meta.provider.to_string(),
                price: tool_meta.price,
                denom: tool_meta.denom,
                is_active: tool_meta.is_active,
                description: tool_meta.description,
                endpoint: tool_meta.endpoint,
            };
            to_json_binary(&response)
        },
        None => to_json_binary(&Option::<ToolResponse>::None),
    }
}

// GetTools query implementation
pub fn query_all_tools(deps: Deps) -> StdResult<Binary> {
    let tools: StdResult<Vec<_>> = TOOLS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| {
            let (tool_id, tool_meta) = item?;
            Ok(ToolResponse {
                tool_id: tool_id.to_string(),
                provider: tool_meta.provider.to_string(),
                price: tool_meta.price,
                denom: tool_meta.denom,
                is_active: tool_meta.is_active,
                description: tool_meta.description,
                endpoint: tool_meta.endpoint,
            })
        })
        .collect();

    let response = crate::msg::ToolsResponse { tools: tools? };
    to_json_binary(&response)
}