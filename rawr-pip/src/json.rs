use std::path::{Path, PathBuf};
use tokio::fs;

use svix_ksuid::Ksuid;

use crate::{AcmLoader, PipError, PolicyLoader, apply_roles};
use rawr_acm::Acm;
use rawr_pap::Role;

/*
* JSON derulo (static, mainly for integration testdata)
*/
pub struct JsonPolicyLoader {
    base_dir: PathBuf,
}

impl JsonPolicyLoader {
    pub fn new(base_dir: impl AsRef<Path>) -> Self {
        JsonPolicyLoader {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }
}

impl Default for JsonPolicyLoader {
    fn default() -> Self {
        Self::new("testdata")
    }
}

impl PolicyLoader for JsonPolicyLoader {
    fn load_from_str(&self, data: &str) -> Result<Role, serde_json::Error> {
        let role: Role = serde_json::from_str(data)?;
        Ok(role)
    }
}

#[derive(serde::Deserialize)]
struct RoleAssignmentsData {
    roles: Vec<Role>,
}

impl AcmLoader for JsonPolicyLoader {
    async fn load(&self, _account_ksuid: &Ksuid, principal_ksuid: &Ksuid) -> Result<Acm, PipError> {
        let file_path = self.base_dir.join(format!("{principal_ksuid}.json"));

        let contents = fs::read_to_string(&file_path).await.map_err(|e| {
            PipError::AcmNotFound(format!("failed to read file {:?}: {}", file_path, e))
        })?;

        let roles: Vec<Role> = serde_json::from_str::<RoleAssignmentsData>(&contents)
            .map(|pd| pd.roles)
            .map_err(|e| {
                PipError::InvalidAcmFormat(format!("failed to parse {:?} JSON: {}", file_path, e))
            })?;

        let mut acm = Acm::new();
        apply_roles(&mut acm, &roles);

        Ok(acm)
    }
}
