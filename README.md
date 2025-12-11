# Solana Program Boilerplate

## 1. Overview
Production-ready Anchor boilerplate for building scalable Solana programs. This template provides a solid foundation with clean architecture, comprehensive testing setup, and best practices for rapid development.

## 2. Key Features
- **Clean Architecture**: Separation of concerns with distinct layers for instructions, views, states, and utilities.
- **Built-in Access Control**: Granular permission management using custom modifiers.
- **Testing with LiteSVM**: High-performance, lightweight simulation for faster test execution.
- **Ready Testing Fixture**: Pre-configured fixtures to streamline test setup.
- **Extended Chai**: Custom assertions for easier testing, including balance change checks and BN comparisons (see `tests/fixtures/chai-extended.ts`).

## 3. Tech Stack
- **Anchor Framework**: Framework for Solana's Sealevel runtime.
- **LiteSVM**: Fast Solana program simulation.
- **Chai**: BDD / TDD assertion library.

## 4. Prerequisites
- **Rust**: Latest stable version.
- **Solana Tool Suite**: Latest version.
- **Anchor CLI**: Latest version.
- **Node.js**: LTS version.
- **Yarn**: Package manager.

## 5. Getting Started

### Clone and Install Dependencies

1. **Clone the repository:**
   ```bash
   git clone <repository-url> <your-project-name>
   cd <your-project-name>
   ```

2. **Install dependencies:**
   ```bash
   yarn install
   ```

### Run Testing

Execute the test suite using Anchor:

```bash
anchor test
```

### Build

```bash
# Build the program
anchor build
```

## 6. Reference Implementation

This boilerplate includes a **Token Factory** as a reference implementation, demonstrating:
- Token creation with protocol fees
- Whitelisted creation for fee-free token minting
- Access control patterns
- View functions for fee calculation

You can use this as a starting point or replace it entirely with your own program logic.

## 7. Directory Structure

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
