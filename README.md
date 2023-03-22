# OpenBrush Chain Extension library

The library provides tools and primitives to simplify the development of chain 
extensions for ink! and Substrate.

OBCE automatically generates everything needed to correctly call chain extension
from ink! smart contracts, and to correctly implement the chain extension
itself on the Substrate side.

OBCE' macros automatically generate all the logic related to argument encoding/decoding,
function and extension identifier calculation and error handling.

The ink! side of OBCE is fully automated, while with Substrate all that's left is to
implement the chain extension using generated traits.

## Tutorial

In this tutorial we'll spin up our own Substrate node with `rand-extension` enabled.

1. Clone [`substrate-contracts-node`](https://github.com/Cardinal-Cryptography/substrate-contracts-node) repository.
2. Copy `examples/rand-extension/chain-extension` folder to a cloned `substrate-contracts-node` repo and add it as a workspace member to `Cargo.toml`:

```toml
# ...

members = [
    'chain-extension',
    'node',
    'runtime'
]

# ...
```

3. Add `rand-extension` to `runtime/Cargo.toml`:

```toml
rand-extension = { path = "../chain-extension", default-features = false, features = ["substrate"] }
```

Also, add `rand-extension/substrate-std` to feature list that is activated
when `std` feature is active:

```toml
[features]
# ...

std = [
    # ...
    "rand-extension/substrate-std",
]
```

4. Launch node with `./target/debug/substrate-contracts-node --dev --tmp`.
5. Install `cargo-contract` using the [installation guide](https://github.com/paritytech/cargo-contract#installation).
6. Create new contract via `cargo contract new` command.
7. Replace `lib.rs` file with the one, that is provided in `examples/rand-extension`.
8. Modify contract's `Cargo.toml` to include chain extension as a dependency,
and to activate `ink-std` feature if `std` feature of a contract is enabled. Replace `PATH_TO_CHAIN_EXTENSION_CRATE` with the path to `rand-extension` crate:

```toml
[dependencies]
# ...

rand-extension = { path = "PATH_TO_CHAIN_EXTENSION_CRATE", default-features = false, features = ["ink"] }

[dev-dependencies]
# Include OBCE as a dev dependency to test the contract
obce = { git = "https://github.com/727-Ventures/obce", default-features = false, features = ["ink-std"] }

[features]
# ...

std = [
    # ...
    "rand-extension/ink-std"
]
```
9. Build contract using `cargo contract build`.
10. Deploy contract using `cargo contract` or [Contracts UI](https://contracts-ui.substrate.io/).

## Usage examples

* `examples` directory
* [`pallet-assets`](https://github.com/727-Ventures/pallet-assets-chain-extension)