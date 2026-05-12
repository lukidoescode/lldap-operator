use std::fs;
use std::path::{Path, PathBuf};

use kube::CustomResourceExt;
use lldap_operator_crds::{LldapAttributeSchema, LldapGroup, LldapMembership, LldapUser};

#[derive(Debug, thiserror::Error)]
enum CrdgenError {
    #[error("io error writing CRD YAML")]
    Io(#[from] std::io::Error),
    #[error("yaml serialization failed")]
    Yaml(#[from] serde_yaml_ng::Error),
    #[error("CARGO_MANIFEST_DIR not set")]
    NoManifestDir,
}

fn output_dir() -> Result<PathBuf, CrdgenError> {
    let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR").ok_or(CrdgenError::NoManifestDir)?;
    Ok(Path::new(&manifest_dir).join("../../charts/lldap-operator/crds"))
}

fn clean_yaml(dir: &Path) -> Result<(), CrdgenError> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().is_some_and(|ext| ext == "yaml") {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

fn write_crd<T: CustomResourceExt>(dir: &Path, filename: &str) -> Result<(), CrdgenError> {
    let crd = T::crd();
    let yaml = serde_yaml_ng::to_string(&crd)?;
    fs::write(dir.join(filename), yaml)?;
    Ok(())
}

fn main() -> Result<(), CrdgenError> {
    let dir = output_dir()?;
    fs::create_dir_all(&dir)?;
    clean_yaml(&dir)?;
    write_crd::<LldapUser>(&dir, "lldap-user.yaml")?;
    write_crd::<LldapGroup>(&dir, "lldap-group.yaml")?;
    write_crd::<LldapMembership>(&dir, "lldap-membership.yaml")?;
    write_crd::<LldapAttributeSchema>(&dir, "lldap-attribute-schema.yaml")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_yaml_removes_only_yaml_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        fs::write(dir.path().join("stale.yaml"), "x").expect("write yaml");
        fs::write(dir.path().join("keep.txt"), "y").expect("write txt");
        clean_yaml(dir.path()).expect("clean");
        assert!(!dir.path().join("stale.yaml").exists());
        assert!(dir.path().join("keep.txt").exists());
    }
}
