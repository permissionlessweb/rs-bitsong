//! Build Bitsong proto files. This build script clones the CosmosSDK and Bitsong version
//! specified in the COSMOS_SDK_REV and GO_BITSONG_REV constant respectively and then
//! uses that to build the required proto files for further compilation.
//! This is based on the proto-compiler code in github.com/informalsystems/ibc-rs

use std::{env, path::PathBuf};

use proto_build::{
    code_generator::{CodeGenerator, CosmosProject},
    git,
};

/// The Cosmos SDK commit or tag to be cloned and used to build the proto files
const COSMOS_SDK_REV: &str = "v0.53.0";

/// The bitsong commit or tag to be cloned and used to build the proto files
const GO_BITSONG_REV: &str = "v0.23.0";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated cosmos-sdk proto files go into in this repo
const OUT_DIR: &str = "../rs-bitsong-proto/src/types/";
/// Directory where the cosmos-sdk submodule is located
const COSMOS_SDK_DIR: &str = "../submodules/cosmos-sdk/";
/// Directory where the bitsong submodule is located
const BITSONG_DIR: &str = "../submodules/go-bitsong/";
/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

pub fn generate() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--update-deps") {
        git::update_submodule(COSMOS_SDK_DIR, COSMOS_SDK_REV);
        git::update_submodule(BITSONG_DIR, GO_BITSONG_REV);
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let out_dir: PathBuf = OUT_DIR.parse().unwrap();

    let bitsong_project = CosmosProject {
        name: "bitsong".to_string(),
        version: GO_BITSONG_REV.to_string(),
        project_dir: BITSONG_DIR.to_string(),
        exclude_mods: vec![],
    };

    let cosmos_project = CosmosProject {
        name: "cosmos".to_string(),
        version: COSMOS_SDK_REV.to_string(),
        project_dir: COSMOS_SDK_DIR.to_string(),
        exclude_mods: vec!["reflection".to_string(), "autocli".to_string()],
    };

    let bitsong_code_generator = CodeGenerator::new(
        out_dir,
        tmp_build_dir,
        bitsong_project,
        vec![cosmos_project],
    );

    bitsong_code_generator.generate();
}

fn main() {
    pretty_env_logger::init();
    generate();
}