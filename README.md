```bash
NONE OF THESE CONTRACTS ARE AUDITED
```


# Solana Counter Program

A simple counter smart contract built with Anchor framework for learning Solana development.

## What it does

This program creates a counter that can:
- Initialize with count = 0
- Increment the counter
- Decrement the counter (prevents going below 0)
- Reset to 0 (only the creator can do this)

## Quick Start

### Prerequisites
- [Rust](https://rustup.rs/)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor Framework](https://www.anchor-lang.com/docs/installation)

### Setup
```bash
# Clone the repo
git clone <your-repo-url>
cd counter-program

# Install dependencies
yarn install

# Build the program
anchor build

# Run tests
anchor test
```

### Deploy
```bash
# Deploy to devnet
anchor deploy --provider.cluster devnet
```

## Program Instructions

| Instruction | Description | Who can call |
|-------------|-------------|--------------|
| `initialize` | Creates a new counter starting at 0 | Anyone |
| `increment` | Adds 1 to the counter | Anyone |
| `decrement` | Subtracts 1 from counter | Anyone |
| `reset` | Sets counter back to 0 | Only the creator |

## Example Usage

```typescript
// Initialize counter
const tx = await program.methods
      .initialize()
      .accounts({
        counter: counterKeypair.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([counterKeypair])
      .rpc();

    console.log("Initialize transaction signature:", tx);

// Increment counter
await program.methods
  .increment()
  .accounts({
    counter: counterKeypair.publicKey,
  })
  .rpc();
```

## Learning Goals

This project teaches:
- ✅ Basic Anchor program structure
- ✅ Account creation and management
- ✅ State persistence on Solana
- ✅ Access control (authority patterns)
- ✅ Error handling and validation
- ✅ Writing and running tests

## Next Steps

After mastering this counter:
1. Try adding a `set_value` function
2. Add multiple counters per user
3. Implement counter with expiration time
4. Build a more complex program with token transfers

---

*Part of my Solana learning journey - building towards becoming job-ready by end of year!*