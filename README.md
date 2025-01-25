# Rust State Machine

A minimal blockchain-style state machine implementation showcasing account management, transaction processing, and state transitions.

## Technical Stack
- Rust 1.84+
- No external dependencies
- Built-in test framework

## Quick Start

```bash
git clone https://github.com/qxprakash/state.rs
cd rust-state-machine
cargo run
```

## Example Usage

```rust
let mut runtime = Runtime::new();
runtime.balances.set_balance(&"alice".to_string(), 100);
runtime.balances.transfer("alice".to_string(), "bob".to_string(), 30)?;
```

## Runtime State

```rust
Runtime {
    system: Pallet {
        block_number: 1,
        nonce: {
            "alice": 2,
        },
    },
    balances: Pallet {
        balances: {
            "alice": 50,
            "bob": 30,
            "charlie": 20,
        },
    },
}
```

## Project Structure

```
src/
├── main.rs      # Runtime implementation
├── system.rs    # Block/nonce tracking
└── balances.rs  # Token management
```