# Pay-Per-Tool

**Pay-per-call escrow for AI tools on Neutron (CosmWasm 1.5 + TypeScript SDK)**

---

## Overview

Pay-Per-Tool is a minimal, secure, and extensible pay-per-call escrow system for AI tool providers and consumers, built on the Neutron blockchain using CosmWasm smart contracts. It enables users to lock funds for a tool call, providers to verify and claim fees, and ensures refunds if a provider does not deliver within a set time window.

---

## Architecture

```
┌─────────────────┐   register_tool   ┌─────────────────────┐
│  Provider CLI   │ ────────────────► │   Registry Contract │
└─────────────────┘                   └─────────────────────┘
        ▲                                         │
        │    lock_funds(tool_id, max_fee, token)  │
        │                                         ▼
┌─────────────────┐                        ┌──────────────────┐
│  AI Wallet SDK  │ ────────────────►      │ Escrow Contract  │
└─────────────────┘          escrow_id ◄───┤  (Custody + log) │
        │  auth_token                      └──────────────────┘
        │  + payload                                ▲
        ▼                                           │ verify_escrow(…)
┌─────────────────┐  HTTP POST (auth_token) ┌──────────────────┐
│  Provider API   │ ───────────────────────►│  Verifier lib    │
└─────────────────┘        usage_receipt    └──────────────────┘
```

---

## Features

- **Registry Contract**: Providers register tools, set prices, pause/resume tools, and update pricing.
- **Escrow Contract**: Users lock funds for tool usage, providers claim fees, and refunds are automatic on timeout.
- **Provider SDK (TypeScript)**: Production-ready SDK for off-chain verification, usage reporting, wallet integration, and error handling. Includes demo scripts and comprehensive documentation.
- **Comprehensive Testing**: All contract flows are covered by unit and integration tests.
- **Security**: Strict authorization, expiration limits, and contract freezing for emergencies.

---

## Project Structure

```
toolpay/
├── contracts/
│   ├── registry/   # Registry contract (CosmWasm)
│   └── escrow/     # Escrow contract (CosmWasm)
├── packages/
│   └── provider-sdk/  # TypeScript SDK (production-ready)
├── frontend/       # User interface (planned)
├── scripts/        # Helper scripts
├── Cargo.toml      # Rust workspace config
├── blueprint.md    # Step-by-step implementation plan
├── project.md      # Full specification
├── tasks.md        # Actionable task list
├── notes/index.md        # Implementation notes and history
└── ...
```

---

## Quickstart

### Prerequisites

- Rust 1.78+ with wasm32 target
- Node.js 20+ (for SDK/frontend, planned)
- [cargo-generate](https://github.com/ashleygwilliams/cargo-generate)
- [Neutron localnet](https://neutron.org/) (for integration testing)

### Build & Test Contracts

```sh
# Build all contracts
cargo build --release

# Run all unit and integration tests (wasm)
cargo wasm-test
```

### Directory Setup

- `contracts/registry`: Registry contract for tool metadata and pricing.
- `contracts/escrow`: Escrow contract for fund locking, release, and refunds.

---

## Smart Contract Specs

### Registry Contract

- **RegisterTool**: Register a tool with a unique ID (≤16 chars) and price.
- **UpdatePrice**: Update the price for a registered tool.
- **PauseTool/ResumeTool**: Temporarily disable or enable a tool.
- **GetTool**: Query tool metadata.

**Storage**: `TOOLS: Map<String, ToolMeta>`

### Escrow Contract

- **LockFunds**: User locks funds for a tool call, specifying max fee, auth token, and expiration (≤50 blocks).
- **Release**: Provider claims usage fee (≤max_fee), remainder refunded to user.
- **RefundExpired**: User refunds all funds if provider does not claim within TTL.
- **Freeze**: Admin can freeze contract for emergencies.

**Storage**: `ESCROWS: Map<u64, Escrow>`, `NEXT_ID: Item<u64>`, `CONFIG: Item<Config>`

---

## Development Phases

1. **Smart Contracts & Testing**: Registry and Escrow contracts, full test coverage. **(Complete)**
2. **Provider SDK**: TypeScript SDK for off-chain verification, usage reporting, wallet integration, and error handling. **(Complete)**
3. **Frontend**: User-facing app with shadcn UI (planned).

See [tasks.md](./tasks.md) for detailed progress and next steps.

---


## Example SDK Usage

```typescript
import { PayPerToolSDK } from '@toolpay/provider-sdk';

const sdk = new PayPerToolSDK({
  rpcEndpoint: 'https://rpc-pion-1.neutron.org',
  chainId: 'pion-1',
  registryAddress: 'neutron1...',
  escrowAddress: 'neutron1...',
});

const verification = await sdk.escrowVerifier.verifyEscrow({
  escrowId: '123',
  authToken: 'base64token',
  providerAddr: 'neutron1...',
});

if (verification.isValid) {
  const result = await sdk.usageReporter.postUsage({
    escrowId: '123',
    usageFee: '1000000',
    wallet: yourWallet,
  });
  console.log('Usage reported, tx hash:', result.txHash);
}
```

See [packages/provider-sdk/README.md](./packages/provider-sdk/README.md) for full details and more examples.

---


## Testing

- All contract and SDK logic is covered by unit and integration tests using `cw-multi-test` and Jest.
- Edge cases: TTL violations, over-limit fees, unauthorized access, contract freezing, refunds, and SDK error handling.
- See `contracts/escrow/src/tests/`, `contracts/registry/src/tests/`, and `packages/provider-sdk/__tests__/` for test modules.

---

## Tech Stack

| Layer      | Choice                                 |
|------------|----------------------------------------|
| Chain      | Neutron (Cosmos SDK, CosmWasm 1.5)     |
| Contracts  | Rust 1.78, cw-storage-plus 1.2         |
| Provider SDK | Node 20, TypeScript 5, cosmjs, telescope (production-ready) |
| Frontend   | React, shadcn UI, CosmJS (planned)     |
| CI         | GitHub Actions, localnet, npm test     |

---

## Documentation & References

- [blueprint.md](./blueprint.md): Step-by-step implementation plan
- [project.md](./project.md): Full contract and system specification
- [tasks.md](./tasks.md): Actionable task list and progress tracker
- [notes/index.md](./notes/index.md): Implementation notes and design decisions
- [cosmwasm-docs/](./cosmwasm-docs/): CosmWasm and cw-multi-test documentation

---

## Future Directions

- Multi-asset support (IBC)
- DAO-based registry governance
- On-chain metering and dynamic pricing
- Full-featured provider dashboard and analytics

---

## Troubleshooting

- Ensure all dependencies are installed and Rust toolchain is up to date.
- For localnet testing, see scripts and Docker Compose files (to be added in Phase 5).
- For contract errors, consult `notes/index.md` for common issues and solutions.

---

## License

MIT

---

## Deployed Contracts

Pay-Per-Tool contracts are deployed on Neutron testnet (pion-1) with the following addresses:

| Contract  | Address                                                             |
|-----------|---------------------------------------------------------------------|
| Registry  | neutron1zyfl347avgyncyfuqy5px2fapsy4slug83lnrg8vjxxp5jr42hgscv3xv2 |
| Escrow    | neutron1nhg2sqnfs9q5hzh7g0z6vwxqfghtqe65qdjmwdkajkfy2kqws7xsmfn9hx |

### Interacting with Deployed Contracts

```fish
# Query a registered tool
neutrond query wasm contract-state smart neutron1zyfl347avgyncyfuqy5px2fapsy4slug83lnrg8vjxxp5jr42hgscv3xv2 '{"get_tool":{"tool_id":"example-tool"}}'

# Query an escrow by ID
neutrond query wasm contract-state smart neutron1nhg2sqnfs9q5hzh7g0z6vwxqfghtqe65qdjmwdkajkfy2kqws7xsmfn9hx '{"get_escrow":{"escrow_id":1}}'
```

---

For more details, see the reference files in this repository. Contributions and feedback are welcome!
