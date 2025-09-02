# rust-proto-gen

A Rust proto code generation tool for Bitsong blockchain, forked from [Osmosis-Rust](https://github.com/osmosis-labs/osmosis-rust/blob/main/Cargo.toml).

## Overview

This crate generates Rust bindings from Protocol Buffer definitions for the Bitsong blockchain and Cosmos SDK. It automatically clones the specified versions of cosmos-sdk and go-bitsong repositories, processes their proto files, and generates type-safe Rust code with enhanced features for CosmWasm integration.

## Features

### Automatic Code Generation

- **Proto Message Bindings**: Generates Rust structs from .proto files
- **Type URL Constants**: Creates `TYPE_URL` constants for message identification  
- **Serialization Methods**: Provides `to_proto_bytes()` and `to_any()` conversion methods
- **Query Client Integration**: Generates `query()` methods for query structs
- **CosmWasm Integration**: Automatic `From`/`TryFrom` conversions for seamless CosmWasm usage

### Enhanced Query Support
Query structs with `#[proto_query]` attributes automatically generate query client methods:

```rust
#[derive(CosmwasmExt)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.QueryFanTokenRequest")]
#[proto_query(
    path = "/bitsong.fantoken.v1beta1.Query/FanToken", 
    response_type = QueryFanTokenResponse
)]
pub struct QueryFanTokenRequest {
    pub denom: String,
}

// Generated query method available:
let response = QueryFanTokenRequest { 
    denom: "ufantoken".to_string() 
}.query(&querier)?;
```

## Usage

### Basic Generation
Run the code generator to build proto files from the current submodule versions:

```bash
cargo run
```

### Update Dependencies
To update the cosmos-sdk and go-bitsong submodules to the latest specified versions:

```bash
cargo run -- --update-deps
```

## Configuration

The tool uses these default versions (defined in `src/main.rs`):
- **Cosmos SDK**: `v0.53.0`
- **Bitsong**: `v0.23.0`

Generated files are output to: `../rs-bitsong-proto/src/types/`

## Directory Structure

```
rust-proto-gen/
├── src/
│   ├── main.rs           # Entry point and configuration
│   ├── lib.rs            # Module exports
│   ├── code_generator.rs # Core generation logic
│   ├── git.rs            # Git submodule management
│   ├── mod_gen.rs        # Module generation
│   ├── transform.rs      # Code transformations
│   └── transformers.rs   # Specific transformers
├── Cargo.toml
└── Readme.md
```

## Dependencies

Key dependencies include:
- `prost` & `prost-build` for Protocol Buffer support
- `tonic` & `tonic-build` for gRPC code generation  
- `syn` & `quote` for Rust code manipulation
- `regex` for pattern matching and transformations

## Development

This is an internal build tool (`publish = false`) designed specifically for the rs-bitsong project. It processes proto definitions from both Cosmos SDK and Bitsong to generate the complete set of Rust bindings needed for blockchain interaction.