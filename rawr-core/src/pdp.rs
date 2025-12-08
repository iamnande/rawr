use async_trait::async_trait;

use crate::{PrincipalId, TenantId};
use rawr_resource_name::ResourceName;

/// I know why you're here, Neo. I know what you've been doing... why you
/// hardly sleep, why you live alone, and why night after night, you sit by
/// your computer. You're looking for him. I know because I was once looking
/// for the same thing. And when he found me, he told me I wasn't really
/// looking for him. I was looking for an answer. It's the question, Neo. It's
/// the question that drives us. It's the question that brought you here. You
/// know the question, just as I did.
pub struct AuthorizationRequest<'a> {
    pub tenant_id: TenantId,
    pub principal_id: PrincipalId,
    pub action: &'a str,
    pub resource: ResourceName<'a>,
}

// What is the Matrix?

/// The answer is out there, Neo, and it's looking for you, and it will find you if you want it to.
pub enum AuthorizationResponse {
    Granted, // access granted
    Denied,  // access denied
}

#[async_trait]
pub trait PolicyDecisionPoint: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    /// evaluate an authorization request, returning either Granted or Denied.
    async fn authorize(
        &self,
        request: AuthorizationRequest<'_>,
    ) -> Result<AuthorizationResponse, Self::Error>;
}
