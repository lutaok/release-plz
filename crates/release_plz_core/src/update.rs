use crate::{PackagePath, UpdateRequest};
use cargo_edit::LocalManifest;
use cargo_metadata::{Package, Version};
use git_cmd::{self, Repo};
use std::path::Path;

use tracing::{debug, instrument};

/// Update a local rust project
#[instrument]
pub fn update(input: &UpdateRequest) -> anyhow::Result<(Vec<(Package, Version)>, Repo)> {
    let (packages_to_update, repository) = crate::next_versions(input)?;
    update_versions(&packages_to_update);
    Ok((packages_to_update, repository))
}

#[instrument]
fn update_versions(local_packages: &[(Package, Version)]) {
    for (package, next_version) in local_packages {
        let package_path = package.package_path();
        set_version(package_path, next_version);
    }
}

#[instrument]
fn set_version(package_path: &Path, version: &Version) {
    debug!("updating version");
    let mut local_manifest =
        LocalManifest::try_new(&package_path.join("Cargo.toml")).expect("cannot read manifest");
    local_manifest.set_package_version(version);
    local_manifest.write().expect("cannot update manifest");
}
