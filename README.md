# Bitsong Rust

<div align="center">

[![Bannger](/static/banner.png)]()

</div>


## TODO
- rust <-> python bindings (via: https://github.com/PyO3/pyo3)
- demo webapp interacting with each command


## Crates

| Name                 | Description                 | crates.io | docs.rs | CI Build |
|----------------------|-----------------------------|-----------|---------|----------|
| [`btisong-rs`]           | Bitsong SDK for Rust         | [![crates.io][bitsong-rs-crate-img]][bitsong-rs-crate-link] | [![docs.rs][bitsong-rs-docs-img]][bitsong-rs-docs-link] | [![CI][bitsong-rs-ci-img]][bitsong-rs-ci-link] |
| [`rs-bitsong‑proto`] | Proto and gRPC definitions  | [![crates.io][rs-bitsong‑proto-crate-img]][rs-bitsong‑proto-crate-link] | ![docs.rs][rs-bitsong‑proto-docs-img] | [![CI][rs-bitsong‑proto-ci-img]][rs-bitsong‑proto-ci-link] |
| [`rs-bitsong‑derive`] | Custom derive traits to generate QueryWrappers  | [![crates.io][rs-bitsong‑derive-crate-img]][rs-bitsong‑derive-crate-link] | ![docs.rs][rs-bitsong‑derive-docs-img] | [![CI][rs-bitsong‑derive-ci-img]][rs-bitsong‑derive-ci-link] |

## Building Proto files from source

```sh
cd proto-build && cargo run 
```

## Crafting Messages & Queries Like A Pro

### Preparing Structures with Protobuf Type URLs

Leverage the `CosmwasmExt` derive macro to automatically generate type URLs and conversion traits for your protobuf messages:

```rust
use rs_bitsong_derive::CosmwasmExt;
use bitsong_rs::types::bitsong::fantoken::v1beta1::*;

// Messages automatically get TYPE_URL constants and to_any() methods
let msg = MsgIssue {
    symbol: "BTSG".to_string(),
    name: "BitSong Token".to_string(),
    max_supply: "1000000".to_string(),
    // ... other fields
};

// Access the generated TYPE_URL constant
println!("Type URL: {}", MsgIssue::TYPE_URL);

// Convert to Any message for transactions
let any_msg = msg.to_any();
let cosmos_msg: CosmosMsg<_> = msg.into();
```

### Custom Query Clients

Import and use the pre-built query clients for each Bitsong module with CosmWasm's custom querier:

#### Importing Query Structures

```rust
use cosmwasm_std::{Deps, StdResult, to_binary, Binary};
use bitsong_rs::types::bitsong::fantoken::v1beta1::{
    QueryFanTokenRequest, QueryFanTokensRequest, QueryParamsRequest
};
use bitsong_rs::types::bitsong::cadence::v1::QueryParamsRequest as CadenceQueryParamsRequest;
use bitsong_rs::types::bitsong::smartaccount::v1beta1::{
    QueryParamsRequest as SmartAccountQueryParamsRequest,
    QuerySettingRequest
};
```

#### Using Query Clients Directly

The `CosmwasmExt` derive macro automatically generates `.query()` methods for all query structures:

```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // Fantoken queries
        QueryMsg::GetFantoken { denom } => {
            let request = QueryFanTokenRequest { denom };
            let response = request.query(&deps.querier)?;
            to_binary(&response)
        },
        
        QueryMsg::GetFantokens { authority, pagination } => {
            let request = QueryFanTokensRequest { authority, pagination };
            let response = request.query(&deps.querier)?;
            to_binary(&response)
        },

        QueryMsg::GetFantokenParams {} => {
            let request = QueryParamsRequest {};
            let response = request.query(&deps.querier)?;
            to_binary(&response)
        },

        // Cadence queries  
        QueryMsg::GetCadenceParams {} => {
            let request = CadenceQueryParamsRequest {};
            let response = request.query(&deps.querier)?;
            to_binary(&response)
        },

        // SmartAccount queries
        QueryMsg::GetSmartAccountSetting { address } => {
            let request = QuerySettingRequest { address };
            let response = request.query(&deps.querier)?;
            to_binary(&response)
        },
    }
}
```

#### Available Query Methods by Module

**Fantoken Module:**
- `QueryFanTokenRequest` - Query specific fantoken by denom
- `QueryFanTokensRequest` - Query all fantokens with optional filters  
- `QueryParamsRequest` - Query fantoken module parameters

**Cadence Module:**
- `QueryParamsRequest` - Query cadence module parameters

**SmartAccount Module:**
- `QueryParamsRequest` - Query smartaccount module parameters
- `QuerySettingRequest` - Query smartaccount settings by address

#### Error Handling

All query methods return `StdResult<ResponseType>` for consistent error handling:

```rust
match request.query(&deps.querier) {
    Ok(response) => to_binary(&response),
    Err(e) => Err(StdError::generic_err(format!("Query failed: {}", e))),
}
```

### Avoiding Rust Dependency Feature Hell

This workspace uses careful feature management to avoid dependency conflicts:

```toml
# Core proto crate with minimal dependencies but enable std
rs-bitsong-proto = { default-features = false, features = ["std"] }

# SDK with specific cosmos-sdk-proto version to avoid conflicts
cosmos-sdk-proto = { 
    version = "0.28.0", 
    default-features = false, 
    features = ["std"],
    git = "https://github.com/permissionlessweb/rs-bitsong" 
}

# Crypto dependencies with precise feature selection
k256 = { version = "0.13", default-features = false, features = ["ecdsa", "sha256"] }
```

**Key practices:**

- **Use `default-features = false`** to avoid unnecessary dependency bloat
- **Pin specific git commits** for cosmos-sdk-proto to ensure compatibility
- **Enable only required features** (`std`, `stargate`, etc.) to minimize conflicts
- **Workspace resolver = "2"** for better dependency resolution across crates

#### Avoiding Tokio/WASM32 Compilation Issues

When targeting `wasm32-unknown-unknown` (CosmWasm contracts, Trunk frontend apps), tokio-related dependencies cause compilation failures. Prevent these issues:

```toml
# In your CosmWasm contract Cargo.toml
[dependencies]
# Explicitly exclude tokio-dependent features
bitsong-rs = { version = "0.1", default-features = false }
rs-bitsong-proto = { version = "0.1", default-features = false, features = ["std"] }

# For crypto dependencies, avoid async features
k256 = { version = "0.13", default-features = false, features = ["ecdsa", "sha256"] }
rand_core = { version = "0.6", default-features = false }
```

**Critical exclusions for WASM targets:**

- ❌ **Never enable** `tonic`, `grpc`, `transport` features  
- ❌ **Avoid** `tokio` runtime features in any dependency
- ❌ **Don't use** `async-std` or other async runtimes
- ✅ **Always use** `default-features = false`
- ✅ **Enable only** `std`, `serde`, and crypto-specific features

#### Frontend/Trunk Integration

For frontend apps using Trunk, add these to your workspace `Cargo.toml`:

```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
# WASM-specific overrides to prevent tokio inclusion
getrandom = { version = "0.2", features = ["js"] }

[profile.release]
# Optimize for WASM size and performance
opt-level = "s"
lto = true
codegen-units = 1
panic = "abort"
```

#### Troubleshooting Common Errors

**Error: `tokio::runtime` not available on wasm32**
```bash
# Check for problematic dependencies
cargo tree --target wasm32-unknown-unknown | grep -i tokio
```

**Solution:** Ensure all cosmos/tonic dependencies use `default-features = false`