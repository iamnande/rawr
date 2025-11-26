use rawr_acm::Acm;
use rawr_pap::{Effect, Role};
use svix_ksuid::Ksuid;

mod error;
mod json;

pub use error::PipError;
pub use json::JsonPolicyLoader;

// NOTE(*): it's physically painful _not_ to write ACMLoader, but like - w/e
pub trait AcmLoader {
    fn load(&self, principal_ksuid: &Ksuid) -> Result<Acm, PipError>;
}

// TODO(nick): we should probably make this more generic, like eventually.
pub trait PolicyLoader {
    fn load_from_str(&self, data: &str) -> Result<Role, serde_json::Error>;
}

/// Apply a single role's policies to an ACM
pub fn apply_role(acm: &mut Acm, role: &Role) {
    for policy in &role.policies {
        for action in &policy.actions {
            for resource in &policy.resources {
                match policy.effect {
                    Effect::Allow => acm.allow(action, resource),
                    Effect::Deny => acm.deny(action, resource),
                }
            }
        }
    }
}

/// Apply multiple roles' policies to an ACM
pub fn apply_roles(acm: &mut Acm, roles: &[Role]) {
    for role in roles {
        apply_role(acm, role);
    }
}
