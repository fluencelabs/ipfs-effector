#![feature(try_blocks)]
#![feature(assert_matches)]
#![allow(improper_ctypes)]
#![allow(non_snake_case)]

mod import;
mod utils;

use eyre::{eyre, Result};
use ipfs_effector_types::*;
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;

use crate::import::ipfs;
use crate::utils::inject_vault;

module_manifest!();

/// Default chunk size for `ipfs add` command to produce stable CIDs.
const CHUCK_SIZE: usize = 262144;
const CONNECT_TIMEOUT: usize = 5;

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Debug)
        .build()
        .unwrap();
}

/// Run `ipfs` mounted binary with the specified arguments
fn run_ipfs(cmd: Vec<String>) -> Result<String> {
    let result = ipfs(cmd.clone());

    result
        .into_std()
        .ok_or(eyre!("stdout or stderr contains non valid UTF8 string"))?
        .map_err(|e| eyre::eyre!("ipfs cli call failed \n{:?}: {}", cmd.join("  "), e))
}

fn make_cmd_args(args: Vec<String>, api_multiaddr: String) -> Vec<String> {
    args.into_iter()
        .chain(vec![
            String::from("--timeout"),
            format!("{}s", CONNECT_TIMEOUT),
            String::from("--api"),
            api_multiaddr,
        ])
        .collect()
}

/// Put file from specified path to IPFS and return its hash.
// ipfs add --cid-version 1 --hash sha2-256 --chunker=size-262144 # to produce CIDv1
//   --api <api>
//   -Q   # to get hash as the output
//   <data_vault_path>
#[marine]
pub fn add(api_multiaddr: String, input_vault_path: String) -> IpfsAddResult {
    add_impl(api_multiaddr, input_vault_path).into()
}

fn add_impl(api_multiaddr: String, input_vault_path: String) -> Result<String> {
    if !std::path::Path::new(&input_vault_path).exists() {
        return Err(eyre!("path {} doesn't exist", input_vault_path));
    }

    let input_vault_path = inject_vault(&input_vault_path)?;
    let args = vec![
        String::from("add"),
        String::from("-Q"),
        input_vault_path,
        String::from("--cid-version=1"),
        format!("--chunker=size-{}", CHUCK_SIZE),
    ];
    let cmd = make_cmd_args(args, api_multiaddr);
    run_ipfs(cmd).map(|res| res.trim().to_string())
}

/// Get file by provided hash from IPFS, save it to a `file_path`, and return that path
#[marine]
pub fn get(api_multiaddr: String, cid: String, output_vault_path: &str) -> IpfsResult {
    get_impl(api_multiaddr, cid, output_vault_path).into()
}

fn get_impl(api_multiaddr: String, cid: String, output_vault_path: &str) -> Result<()> {
    let output_vault_path = inject_vault(output_vault_path)?;
    let args = vec![
        String::from("get"),
        String::from("-o"),
        output_vault_path,
        cid,
    ];
    let cmd = make_cmd_args(args, api_multiaddr);
    run_ipfs(cmd).map(drop)
}
