# Token Factory Scaffold

## 1. Setup

### Initialize Project
```bash
anchor new token-factory
```

### Install Dependencies
```bash
yarn add -D @solana/spl-token @solana/web3.js @types/bn.js @types/bs58 @types/chai @types/chai-as-promised @types/deepmerge @types/lodash @types/luxon @types/mocha anchor-litesvm bs58 chai chai-as-promised deepmerge env-cmd keccak256 litesvm lodash luxon merkletreejs mocha prettier ts-mocha typescript viem ts-node
```

## 2. Architecture

```text
├── programs
│   ├── token-factory
│   │   ├── src
│   │   │   ├── lib.rs            // Entry point
│   │   │   ├── error.rs          // Error definitions
│   │   │   ├── access_controls   // Authorization logic
│   │   │   ├── instructions      // Instruction handlers
│   │   │   │   └── admin         // Admin-specific instructions
│   │   │   ├── views             // View functions
│   │   │   ├── states            // Account state definitions
│   │   │   └── utils             // Utility functions
├── shared                        // Shared TypeScript code (tests & scripts)
├── scripts                       // Interaction scripts
├── tests
│   ├── fixtures
│   │   ├── chai-extended.ts      // Chai assertions extension
│   │   ├── factory-fixture.ts    // Program test fixture
│   │   ├── spl.ts                // Token utilities
│   │   └── utils.ts              // General test helpers
│   └── token-factory.ts          // Feature tests
```

## 3. Features

The **Token Factory** reference implementation includes:
- **Token Creation**: Support for creating tokens with protocol fees.
- **Whitelisted Creation**: Fee-free token creation for whitelisted users.

## 4. Development Practices

- **Clean Architecture**: Separation of instructions, views, states, utilities, and errors.
- **Access Control**: Granular permissions via `onlyOwner`, `onlyPermission`, and `onlyPermissionAdmin` modifiers.
- **Clean Code**: CPI transactions are isolated into helper functions for improved readability.
- **High-Performance Testing**: Integrated with **LiteSVM** for fast, lightweight simulation.

## 5. Environment Configuration

### Keypair Generation
Create a dedicated folder for keypairs and generate a new wallet:
```bash
mkdir -p .pks && solana-keygen new --no-bip39-passphrase -o .pks/dev.json
```

### Environment Variables
Configure your environment using `.env-cmdrc`. This file manages environment-specific variables (e.g., wallet path, RPC URL).

1. Create `.env-cmdrc` based on the sample:
   ```bash
   cp .env-cmdrc.sample .env-cmdrc
   ```

2. Update `.env-cmdrc` with your configuration:
   ```json
   {
     "dev": {
       "ANCHOR_WALLET": "./.pks/dev.json",
       "SOL_URL": "https://api.devnet.solana.com"
     }
   }
   ```
