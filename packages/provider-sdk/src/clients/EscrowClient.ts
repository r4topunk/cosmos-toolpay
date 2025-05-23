/**
 * Escrow contract client
 *
 * Provides a high-level API for interacting with the Escrow contract.
 */

import { CosmWasmClient, SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import type { Coin } from '@cosmjs/stargate';
import type { Uint128 } from '../types/common.js';
import type { EscrowExecuteMsg, EscrowResponse, CollectedFeesResponse } from '../types/escrow.js';

/**
 * Result type for lockFunds transaction
 */
export interface LockFundsResult {
  transactionHash: string;
  escrowId: number;
  denom: string | undefined;
}

/**
 * Client for interacting with the PayPerTool Escrow contract
 */
export class EscrowClient {
  private readonly client: CosmWasmClient | SigningCosmWasmClient;
  private readonly contractAddress: string;

  /**
   * Create a new EscrowClient
   *
   * @param client - CosmWasm client (signing or non-signing)
   * @param contractAddress - Address of the Escrow contract
   */
  constructor(client: CosmWasmClient | SigningCosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
  }

  /**
   * Get the Escrow contract address
   *
   * @returns The contract address
   */
  getContractAddress(): string {
    return this.contractAddress;
  }

  /**
   * Get the underlying CosmWasm client
   *
   * @returns The CosmWasm client
   */
  getClient(): CosmWasmClient | SigningCosmWasmClient {
    return this.client;
  }

  /**
   * Query information about a specific escrow
   *
   * @param escrowId - ID of the escrow to query
   * @returns Escrow information if found
   */
  async getEscrow(escrowId: number): Promise<EscrowResponse> {
    return await this.client.queryContractSmart(this.contractAddress, {
      get_escrow: { escrow_id: escrowId },
    });
  }

  /**
   * Check if the client is a signing client
   *
   * @private
   * @throws Error if the client is not a signing client
   */
  private getSigningClient(): SigningCosmWasmClient {
    if (!('execute' in this.client)) {
      throw new Error('This method requires a signing client');
    }
    return this.client as SigningCosmWasmClient;
  }

  /**
   * Lock funds for a tool provider with an authentication token
   *
   * @param senderAddress - The address executing the transaction
   * @param toolId - The tool ID in the registry
   * @param maxFee - The maximum fee the caller is willing to pay
   * @param authToken - Authentication token for the tool to verify the escrow
   * @param expires - Block height when this escrow expires
   * @param funds - Optional array of coins to send with the transaction
   * @param memo - Optional transaction memo
   * @returns Object containing transactionHash and escrowId
   */
  async lockFunds(
    senderAddress: string,
    toolId: string,
    maxFee: Uint128,
    authToken: string,
    expires: number,
    funds: readonly Coin[] = [],
    memo?: string,
  ): Promise<LockFundsResult> {
    const signingClient = this.getSigningClient();

    const msg: EscrowExecuteMsg = {
      lock_funds: {
        tool_id: toolId,
        max_fee: maxFee,
        auth_token: authToken,
        expires,
      },
    };

    const result = await signingClient.execute(
      senderAddress,
      this.contractAddress,
      msg,
      'auto',
      memo,
      funds,
    );

    // Extract escrow_id and denom from the event attributes
    let escrowId: number = 0;
    let denom: string | undefined;

    for (const event of result.events) {
      if (event.type === 'wasm') {
        for (const attr of event.attributes) {
          if (attr.key === 'escrow_id') {
            escrowId = parseInt(attr.value);
          }
          if (attr.key === 'denom') {
            denom = attr.value;
          }
        }
      }
    }

    return {
      transactionHash: result.transactionHash,
      escrowId,
      denom,
    };
  }

  /**
   * Release locked funds to the provider after tool usage
   *
   * @param senderAddress - The address executing the transaction
   * @param escrowId - The escrow ID to release funds from
   * @param usageFee - The actual usage fee to charge (must be ≤ max_fee)
   * @param funds - Optional array of coins to send with the transaction
   * @param memo - Optional memo for the transaction
   * @returns Transaction result
   */
  async releaseFunds(
    senderAddress: string,
    escrowId: number,
    usageFee: Uint128,
    funds: readonly Coin[] = [],
    memo?: string,
  ): Promise<string> {
    const signingClient = this.getSigningClient();

    const msg: EscrowExecuteMsg = {
      release: {
        escrow_id: escrowId,
        usage_fee: usageFee,
      },
    };

    const result = await signingClient.execute(
      senderAddress,
      this.contractAddress,
      msg,
      'auto',
      memo,
      funds,
    );

    return result.transactionHash;
  }

  /**
   * Refund locked funds to the caller if the escrow has expired
   *
   * @param senderAddress - The address executing the transaction
   * @param escrowId - The escrow ID to refund
   * @param funds - Optional array of coins to send with the transaction
   * @param memo - Optional memo for the transaction
   * @returns Transaction result
   */
  async refundExpired(
    senderAddress: string,
    escrowId: number,
    funds: readonly Coin[] = [],
    memo?: string,
  ): Promise<string> {
    const signingClient = this.getSigningClient();

    const msg: EscrowExecuteMsg = {
      refund_expired: {
        escrow_id: escrowId,
      },
    };

    const result = await signingClient.execute(
      senderAddress,
      this.contractAddress,
      msg,
      'auto',
      memo,
      funds,
    );

    return result.transactionHash;
  }

  /**
   * Query current collected fees
   *
   * @returns Information about collected fees including owner, fee percentage, and amounts by denomination
   */
  async getCollectedFees(): Promise<CollectedFeesResponse> {
    return await this.client.queryContractSmart(this.contractAddress, {
      get_collected_fees: {},
    });
  }

  /**
   * Claim accumulated fee(s) - owner only
   *
   * @param senderAddress - The address claiming the fees (must be contract owner)
   * @param denom - Optional specific denomination to claim, if not provided claims all denoms
   * @param funds - Optional array of coins to send with the transaction
   * @param memo - Optional memo for the transaction
   * @returns Transaction result
   */
  async claimFees(
    senderAddress: string,
    denom?: string,
    funds: readonly Coin[] = [],
    memo?: string,
  ): Promise<string> {
    const signingClient = this.getSigningClient();

    const msg: EscrowExecuteMsg = {
      claim_fees: denom ? { denom } : {},
    };

    const result = await signingClient.execute(
      senderAddress,
      this.contractAddress,
      msg,
      'auto',
      memo,
      funds,
    );

    return result.transactionHash;
  }
}
