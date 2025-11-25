use rawr_acm::Acm;
use rawr_pip::AcmLoader;
use svix_ksuid::Ksuid;

pub trait Decider {
    fn decide(&self, action: &str, resource_path: &str) -> bool;
}

pub struct RawrDecider {
    acm_loader: Box<dyn AcmLoader>,
}

impl RawrDecider {
    pub fn new(acm_loader: Box<dyn AcmLoader>) -> Self {
        RawrDecider { acm_loader }
    }
}

impl Decider for RawrDecider {
    fn decide(
        &self,
        principal_ksuid: &Ksuid,
        action: &str,
        resource_path: &str,
    ) -> Result<bool, PdpError> {
        // pc load letter
        let acm = self.acm_loader.load(principal_ksuid)?;
        Ok(acm.authorized(action, resource_path))
    }
}
