use rawr_acm::Acm;
use rawr_pap::Role;
use svix_ksuid::Ksuid;

mod error;
mod json;
// TODO(nick): mod static_loader;?

pub use error::PipError;
pub use json::JsonPolicyLoader;

// NOTE(*): it's physically painful _not_ to write ACMLoader, but like - w/e
pub trait AcmLoader {
    fn load(&self, principal_ksuid: &Ksuid) -> Result<Acm, AcmLoaderError>;
}

// TODO(nick): we should probably make this more generic, like eventually.
pub trait PolicyLoader {
    fn load_from_str(&self, data: &str) -> Result<Role, serde_json::Error>;
}
