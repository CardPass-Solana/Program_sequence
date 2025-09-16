Config File: /Users/hyeokx/.config/solana/cli/config.yml
RPC URL: https://api.mainnet-beta.solana.com 
WebSocket URL: wss://api.mainnet-beta.solana.com/ (computed)
Keypair Path: /Users/hyeokx/.config/solana/id.json 
Commitment: confirmed 

Typical Folder Structure:

  my-solana-program/
  ├── programs/
  │   └── my-prosgram/
  │       ├── src/
  │       │   ├── lib.rs           # Main program logic
  │       │   ├── instructions/    # Individual instruction handlers
  │       │   ├── state/          # Account structures
  │       │   └── errors.rs       # Custom error definitions
  │       └── Cargo.toml
  ├── tests/                      # Integration tests
  ├── migrations/                 # Deployment scripts
  ├── app/                       # Frontend (optional)
  └── Anchor.toml                # Project config

  Key Components:
  - Instructions - Functions that modify blockchain state
  - Accounts - Data structures stored on-chain
  - Program Derived Addresses (PDAs) - Deterministic addresses
  - Cross Program Invocations (CPIs) - Calling other programs

  Development Tools:
  - Anchor framework (recommended) - simplifies development
  - Solana CLI - deployment and testing
  - Phantom/Solflare - wallet integration

  The exact structure depends on whether you use vanilla Solana or
  the Anchor framework, with Anchor being more opinionated but
  easier to work with.