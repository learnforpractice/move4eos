// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use bytecode_verifier::verifier::VerifiedModule;
use ir_to_bytecode::{compiler::compile_module, parser::parse_module};
use std::{
    fs,
    path::{Path, PathBuf},
};
use types::account_address::AccountAddress;
use vm::{access::ModuleAccess, file_format::CompiledModule};

pub fn do_compile_module<T: ModuleAccess>(
    source_path: &Path,
    address: &AccountAddress,
    dependencies: &[T],
) -> CompiledModule {
    let source = fs::read_to_string(source_path)
        .unwrap_or_else(|_| panic!("Unable to read file: {:?}", source_path));
    let parsed_module = parse_module(&source).unwrap();
    compile_module(address, &parsed_module, dependencies).unwrap()
}

pub fn build_stdlib(address: &AccountAddress) -> Vec<VerifiedModule> {
    // TODO: Change source paths for stdlib when we have proper SDK packaging.
    let mut stdlib_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    stdlib_root.pop();
    stdlib_root.push("stdlib");

    let mut stdlib_modules = vec![];
    for e in [
        "modules/address_util.mvir",
        "modules/u64_util.mvir",
        "modules/bytearray_util.mvir",
        "modules/hash.mvir",
        "modules/signature.mvir",
        "modules/libra_coin.mvir",
        "modules/libra_account.mvir",
        "modules/validator_set.mvir",
        "modules/debug.mvir",
        "modules/db.mvir",
    ]
    .iter()
    {
        let res = do_compile_module(&Path::join(&stdlib_root, e), address, &stdlib_modules);
        let verified_module = VerifiedModule::new(res)
            .unwrap_or_else(|errors| panic!("Failed to verify module: {:?}: {:?}", e, errors));
        stdlib_modules.push(verified_module);
    }
    stdlib_modules
}
