# Fluence IPFS Effector 

In this project, we provide an effector for [IPFS CLI](https://docs.ipfs.tech/install/command-line/#system-requirements).

### Project Structure

This project consists of 4 crates:
- `effector` is the effector module itself. To obtain the correct WASM module, it must be built with marine build `--release`;
- `cid` is a rust crate for exporting the [CID](https://docs.ipfs.tech/concepts/content-addressing/#version-1-v1) of the effector module. This crate is optional and only provides means to embed CIDs of selected effectors into your rust project, for example, [Nox](https://github.com/fluencelabs/nox);
- `types` is a rust crate with the type definitions used in the API functions of the effector module (the one with the `#[marine]` tag). This crate helps to interact with the effector module's API in non-effector modules. However, this crate isn't supposed to be used on its own outside of the effector crate scope; it's re-exported by the imports crate below;
- `imports` is a rust crate providing the type definitions (via the types crate) as well as effector module's [import definition](https://fluence.dev/docs/marine-book/marine-rust-sdk/developing/import-functions). This crate is aimed to help import the effector modules without copy-pasting the definitions manually.

### How to build

To build the project, you need:
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to build the rust project
- [IPFS CLI](https://docs.ipfs.tech/install/command-line/#system-requirements) to build the cid crate
- [Fluence CLI](https://fluence.dev/docs/build/setting-up/installing_cli) to build the effector

The build.sh bash script located in the repository's root contains the commands to build the effector module and the cid crate.
The test.sh bash script located in the repository's root contains the commands to run tests in the effector module.

### Interface

The `ipfs-effector-imports` provides the following interface:
```rust
#[marine]
#[derive(Clone, Debug)]
pub struct IpfsResult {
    /// True when the operation is successful
    pub success: bool,
    /// Contains an error message when `success` is false
    pub error: String,
}

#[marine]
#[derive(Clone, Debug)]
pub struct IpfsAddResult {
    /// True when the operation is successful
    pub success: bool,
    /// Contains an error message when `success` is false
    pub error: String,
    /// CIDv1 of the uploaded file
    pub hash: String,
}


// Upload a file `input_vault_path` to IPFS node with the `api_multiaddr` multiaddress
pub fn add(api_multiaddr: String, input_vault_path: String) -> IpfsAddResult;

// Downloads a file by `cid` to the `output_vault_path` file from IPFS node with the `api_multiaddr` multiaddress
pub fn get(api_multiaddr: String, cid: String, output_vault_path: &str) -> IpfsResult;
```
