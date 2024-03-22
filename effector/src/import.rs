use marine_rs_sdk::{marine, MountedBinaryResult};

#[marine]
#[host_import]
extern "C" {
    /// Execute provided cmd as a parameters of ipfs cli
    pub fn ipfs(cmd: Vec<String>) -> MountedBinaryResult;
}
