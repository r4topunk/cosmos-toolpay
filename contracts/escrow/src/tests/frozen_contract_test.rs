//! # Frozen Contract Test
//! 
//! This module tests that the Escrow contract properly rejects operations
//! when the contract is in a frozen state.
//! 
//! ## Test Coverage
//! 
//! This test verifies that:
//! 1. When the contract is frozen via sudo, operations are rejected
//! 2. LockFunds, Release, and RefundExpired cannot be executed when frozen
//! 3. The correct error is returned for operations on a frozen contract

use cosmwasm_std::{Addr, Coin, Uint128, to_json_binary};
use cw_multi_test::{Executor, SudoMsg as CwSudoMsg};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, SudoMsg};
use crate::tests::setup_contract::{
    setup_contracts, register_tool, lock_funds, NEUTRON, DEFAULT_TOOL_ID, 
    DEFAULT_MAX_FEE, USER, PROVIDER, DEFAULT_TTL,
};

/// # Test: Contract Operations When Frozen
/// 
/// This test ensures that the Escrow contract correctly rejects all operations
/// when the contract has been frozen by the admin.
/// 
/// ## Test Steps:
/// 
/// 1. Set up Registry and Escrow contracts
/// 2. Register a tool and lock funds in an escrow
/// 3. Freeze the contract using sudo
/// 4. Try various operations (lock funds, release, refund)
/// 5. Verify all operations fail with the correct error
#[test]
fn test_frozen_contract() {
    // Set up the contracts
    let mut contracts = setup_contracts();
    
    // Register a tool
    register_tool(
        &mut contracts,
        DEFAULT_TOOL_ID,
        DEFAULT_MAX_FEE,
        PROVIDER,
    ).unwrap();
    
    // Lock funds for a test escrow (before freezing)
    let auth_token = "frozen_contract_test".to_string();
    let escrow_id = lock_funds(
        &mut contracts,
        DEFAULT_TOOL_ID,
        DEFAULT_MAX_FEE,
        DEFAULT_TTL,
        auth_token.clone(),
        USER,
        &[Coin {
            denom: NEUTRON.to_string(),
            amount: Uint128::new(DEFAULT_MAX_FEE),
        }],
    ).unwrap();
    
    // Freeze the contract using sudo (as contract owner/admin)
    contracts.app.sudo(
        CwSudoMsg::Wasm(cw_multi_test::WasmSudo {
            contract_addr: Addr::unchecked(&contracts.escrow_addr),
            message: to_json_binary(&SudoMsg::Freeze {}).unwrap(),
        }),
    ).unwrap();
    
    // SECTION 1: Test locking funds on frozen contract
    let user_addr = contracts.app.api().addr_make(USER);
    let result = contracts.app.execute_contract(
        user_addr.clone(),
        Addr::unchecked(&contracts.escrow_addr),
        &ExecuteMsg::LockFunds {
            tool_id: DEFAULT_TOOL_ID.to_string(),
            max_fee: Uint128::new(DEFAULT_MAX_FEE),
            expires: contracts.app.block_info().height + DEFAULT_TTL,
            auth_token: "another_token".into(),
        },
        &[Coin {
            denom: NEUTRON.to_string(),
            amount: Uint128::new(DEFAULT_MAX_FEE),
        }],
    );
    
    // Verify operation failed with Frozen error
    assert!(result.is_err());
    match result.unwrap_err().downcast::<ContractError>() {
        Ok(ContractError::Frozen {}) => {}, // Expected error
        Ok(err) => panic!("Unexpected error: {:?}", err),
        Err(err) => panic!("Wrong error type: {:?}", err),
    }
    
    // SECTION 2: Test releasing escrow on frozen contract
    let provider_addr = contracts.app.api().addr_make(PROVIDER);
    let result = contracts.app.execute_contract(
        provider_addr,
        Addr::unchecked(&contracts.escrow_addr),
        &ExecuteMsg::Release {
            escrow_id,
            usage_fee: Uint128::new(DEFAULT_MAX_FEE / 2),
        },
        &[],
    );
    
    // Verify operation failed with Frozen error
    assert!(result.is_err());
    match result.unwrap_err().downcast::<ContractError>() {
        Ok(ContractError::Frozen {}) => {}, // Expected error
        Ok(err) => panic!("Unexpected error: {:?}", err),
        Err(err) => panic!("Wrong error type: {:?}", err),
    }
    
    // SECTION 3: Test refunding expired escrow on frozen contract
    // First advance the blockchain to expire the escrow
    contracts.app.update_block(|block| {
        block.height += DEFAULT_TTL + 1;
    });
    
    let result = contracts.app.execute_contract(
        user_addr.clone(),
        Addr::unchecked(&contracts.escrow_addr),
        &ExecuteMsg::RefundExpired {
            escrow_id,
        },
        &[],
    );
    
    // Verify operation failed with Frozen error
    assert!(result.is_err());
    match result.unwrap_err().downcast::<ContractError>() {
        Ok(ContractError::Frozen {}) => {}, // Expected error
        Ok(err) => panic!("Unexpected error: {:?}", err),
        Err(err) => panic!("Wrong error type: {:?}", err),
    }
}
