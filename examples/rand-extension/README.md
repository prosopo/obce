# OBCE usage example

This example is a port of a [rand-extension example](https://github.com/paritytech/ink/tree/master/examples/rand-extension) from ink! repo.

As in the original example, this one provides you with:

* ink! smart contract, that calls the chain extension
* Substrate extension

## Example integration

### Substrate

1. Copy `chain-extension` crate into your project and add it as a workspace member.
2. Add `rand-extension` to `runtime/Cargo.toml`:

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

3. Change `pallet_contracts::Config` to use the chain extension like so:

```rust
impl pallet_contracts::Config for Runtime {
    // ...

    type ChainExtension = (
        pallet_assets_chain_extension::substrate::AssetsExtension,

        // Your custom extension
        rand_extension::substrate::Extension,
    );

    // ...
}
```

### Ink

Use `lib.rs` file as a contract example.
