use std::{env, fs::OpenOptions, io::Write, path::PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};

// std::fs::permissions is limited to a+w, a-w, etc. May want to consider unix-only
// std::os::unix::fs::PermissionsExt if needed.
#[derive(Debug, Clone, Serialize, Deserialize)]
enum Permissions {
    ReadOnly,
    ReadWrite,
    //    UserReadOnly,
    //    UserReadWrite,
    //    GroupReadOnly,
    //    GroupReadWrite,
    //    AllReadOnly,
    //    AllReadWrite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConfigTriple {
    path: PathBuf,
    permissions: Permissions,
    config_map: String,
}

fn create_config_map(
    path: &str,
    permissions: Permissions,
    config: &str,
) -> Result<(), std::io::Error> {
    let mut f = OpenOptions::new().write(true).create_new(true).open(path)?;
    f.write(config.as_bytes())?;
    let p = {
        let mut perms = f.metadata()?.permissions();
        match permissions {
            Permissions::ReadOnly => perms.set_readonly(true),
            Permissions::ReadWrite => perms.set_readonly(false),
        }
        perms
    };
    f.set_permissions(p)?;
    Ok(())
}

// Load ConfigTriple from env (use serde)
// Check if file exists at path
// Check if path is writable
// ? Validate ConfigMap; but how? Do we allow specific file formates, look to k8s for inspiration
// Write ConfigMap
pub fn main() -> anyhow::Result<()> {
    for (key, value) in env::vars() {
        match key.as_str().ends_with("_CONFIGMAP") {
            true => {
                println!("ConfigMap key: {key}\n{value}");
                // TODO:cleanup
                // Use std:str::split_once twice to return a (&str, &str, &str)
                let (pth, per, cfg_env) = value
                    .split_once(',')
                    .map(|(p, r)| {
                        let (per, map) = r.split_once(',').expect("snd element should be split into per and cfg_env");
                        (p, per, map)
                    }).expect("should end up with a triple");
                let p: Permissions = serde_json::from_str(per).context("could not deserialize permissions")?;
                let cfg = env::var(cfg_env).context("cfg_env not in env")?;
                create_config_map(pth, p, &cfg).unwrap();
            }
            false => (),
        }
    }
    Ok(())
}
