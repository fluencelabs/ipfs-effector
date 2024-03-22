use eyre::eyre;
use marine_rs_sdk::ParticleParameters;
use std::path::Path;

/// Map the virtual particle vault path to the real path
/// In effectors, we now accept two kinds of paths:
/// 1. A full virtual path to a file in the particle vault that follows the pattern `/tmp/vault/{particle}/{filename}`
/// 2. A file name that is relative to the particle vault and is interpreted as `/tmp/vault/{particle}/{filename}`
/// 3. A vault path directory itself `/tmp/vault/{particle}`
///
/// All other paths are rejected as invalid.
/// This is done because we don't have a reliable way to check that the paths leads to
/// the particle vault and not to some other (potentially dangerous) location.
///
/// Will be moved out to a separate crate eventually.
///
/// You are welcomed to modify this function to fit your needs for your effectors.
pub fn inject_vault(virtual_path: &str) -> eyre::Result<String> {
    let cp = marine_rs_sdk::get_call_parameters();
    let real_vault_prefix = get_host_vault_path("/tmp/vault")?;
    inject_vault_host_path(&cp.particle, &real_vault_prefix, virtual_path)
}

pub(crate) fn inject_vault_host_path(
    particle: &ParticleParameters,
    real_vault_prefix: &str,
    virtual_path: &str,
) -> eyre::Result<String> {
    let particle_virtual_vault_prefix = Path::new("/tmp/vault").join(format_particle_dir(particle));

    let path = Path::new(&virtual_path);
    // Get the filename from the path by cutting off the `/tmp/vault/{particle}` prefix if the path starts with / or return it
    // as it supposedly already a filename.
    let file_inside_vault = if path.has_root() {
        path.strip_prefix(particle_virtual_vault_prefix).map_err(|_| eyre!("invalid path provided, expected the full path to the particle vault for the current particle"))?
    } else {
        path
    };
    if let Some(filename) = file_inside_vault.file_name() {
        if filename != file_inside_vault.as_os_str() {
            return Err(eyre!("invalid path provided, expected the full path to the particle vault for the current particle or a filename"))?;
        }
        // At this point we are sure that the filename is a filename without any path components
        Ok(format!(
            "{real_vault_prefix}/{}/{}",
            format_particle_dir(particle),
            filename.to_string_lossy()
        ))
    } else {
        // Otherwise, it's the `/tmp/vault/{particle}` directory
        Ok(format!(
            "{real_vault_prefix}/{}",
            format_particle_dir(particle),
        ))
    }
}

// Look for the real directory of the particle vault mapping in the module config
// For local testing, the mapping happens in Config.toml
fn get_host_vault_path(vault_prefix: &str) -> eyre::Result<String> {
    std::env::var(vault_prefix)
        .map_err(|e| eyre!("vault must be mapped to {}: {:?}", vault_prefix, e))
}

// Format the particle directory name.
// The format is pre-defined and should be used in all effectors.
// Particle vaults of other particles are unavailable for all modules.
fn format_particle_dir(particle: &ParticleParameters) -> String {
    format!("{}-{}", particle.id, particle.token)
}

#[cfg(test)]
mod unit_tests {
    use crate::utils::inject_vault_host_path;
    use marine_rs_sdk::ParticleParameters;
    use std::assert_matches::assert_matches;

    #[test]
    fn test_inject() {
        let mut particle = ParticleParameters::default();
        particle.id = "test_id".to_string();
        particle.token = "token".to_string();

        let real_vault_prefix = "/real/storage";

        let result = inject_vault_host_path(
            &particle,
            real_vault_prefix,
            "/tmp/vault/test_id-token/input.json",
        );
        assert_matches!(result, Ok(_));
        assert_eq!(result.unwrap(), "/real/storage/test_id-token/input.json");

        let result = inject_vault_host_path(&particle, real_vault_prefix, "input.json");
        assert_matches!(result, Ok(_));
        assert_eq!(result.unwrap(), "/real/storage/test_id-token/input.json");

        let result = inject_vault_host_path(&particle, real_vault_prefix, "/etc/passwd");
        assert_matches!(result, Err(_), "non-vault paths are forbidden");

        let result = inject_vault_host_path(
            &particle,
            real_vault_prefix,
            "/tmp/vault/test_id2-token2/input.json",
        );
        assert_matches!(
            result,
            Err(_),
            "paths in vaults of other particles are also forbidden"
        );

        let result = inject_vault_host_path(&particle, real_vault_prefix, "vault_dir/input.json");
        assert_matches!(result, Err(_), "only filenames in the vault are allowed");
    }
}
