/**
 * TypeScript types for the Registry contract
 * 
 * These types define the messages, queries, and responses for interacting with
 * the Registry contract.
 */
import type { Uint128 } from "./common.js";

/**
 * InstantiateMsg for the Registry contract
 * 
 * This message is empty for the Registry contract MVP
 */
export interface RegistryInstantiateMsg {}

/**
 * ExecuteMsg for the Registry contract
 * 
 * A union type representing all possible execute messages that can be sent
 * to the Registry contract.
 */
export type RegistryExecuteMsg =
  | { register_tool: { tool_id: string; price: Uint128; denom?: string; description: string } }
  | { update_price: { tool_id: string; price: Uint128 } }
  | { update_denom: { tool_id: string; denom: string } }
  | { pause_tool: { tool_id: string } }
  | { resume_tool: { tool_id: string } };

/**
 * QueryMsg for the Registry contract
 * 
 * A union type representing all possible query messages that can be sent
 * to the Registry contract.
 */
export type RegistryQueryMsg = 
  | { get_tool: { tool_id: string } }
  | { get_tools: {} };

/**
 * ToolResponse from the Registry contract
 * 
 * This is the return type for a GetTool query
 */
export interface ToolResponse {
  tool_id: string;
  provider: string;
  price: Uint128;
  denom: string;
  is_active: boolean;
  description: string;
}

/**
 * ToolsResponse from the Registry contract
 * 
 * This is the return type for a GetTools query
 */
export interface ToolsResponse {
  tools: ToolResponse[];
}
