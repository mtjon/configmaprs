use std::{env, fs::OpenOptions, io::Write, str::FromStr};

use anyhow::Context;

// std::fs::permissions is limited to a+w, a-w, etc. May want to consider unix-only
// std::os::unix::fs::PermissionsExt if needed.
#[derive(Debug, Clone)]
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

#[derive(Debug)]
struct ParsePermissionsError;

impl FromStr for Permissions {
    type Err = ParsePermissionsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ReadOnly" => Ok(Permissions::ReadOnly),
            "ReadWrite" => Ok(Permissions::ReadWrite),
            e => panic!("Unable to parse Permission from: {}", e)
        }
    }
}

fn create_config_map(
    path: &str,
    permissions: Permissions,
    config: &str,
) -> anyhow::Result<()> {
    let path = std::path::Path::new(path);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix)?;
    let mut f = OpenOptions::new().write(true).create_new(true).open(path).context("failed to create file")?;
    f.write(config.as_bytes()).context("failed writing to file")?;
    let p = {
        let mut perms = f.metadata().context("unable to acquire file metadata")?.permissions();
        match permissions {
            Permissions::ReadOnly => perms.set_readonly(true),
            Permissions::ReadWrite => perms.set_readonly(false),
        }
        perms
    };
    f.set_permissions(p).context("failed setting file permissions")?;
    Ok(())
}

pub fn main() -> anyhow::Result<()> {
    for (key, value) in env::vars() {
        match key.as_str().ends_with("_CONFIGMAP") {
            true => {
                // TODO:cleanup
                // Use std:str::split_once twice to return a (&str, &str, &str)
                let (pth, per, cfg_env) = value
                    .split_once(',')
                    .map(|(p, r)| {
                        let (per, map) = r
                            .split_once(',')
                            .expect("snd element should be split into per and cfg_env");
                        (p, per, map)
                    })
                    .expect("should return a triple of &str");
                let p: Permissions =
                    per.parse().unwrap();
                let cfg = env::var(cfg_env).context("cfg_env not in env")?;
                create_config_map(pth, p, &cfg).expect("unable to create configmap");
            }
            false => (),
        }
    }
    Ok(())
}
