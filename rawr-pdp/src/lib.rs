use std::sync::{Arc, RwLock};

use lru::LruCache;
use rawr_acm::{Acm, AcmConfig};
use rawr_pip::AcmLoader;
use svix_ksuid::Ksuid;

mod error;
pub use error::PdpError;

#[allow(async_fn_in_trait)]
pub trait Decider {
    async fn decide(
        &self,
        account_ksuid: &Ksuid,
        principal_ksuid: &Ksuid,
        action: &str,
        resource_path: &str,
    ) -> Result<bool, PdpError>;
}

pub struct RawrDecider<L: AcmLoader> {
    acm_loader: L,
    cache: Arc<RwLock<LruCache<Ksuid, Arc<Acm>>>>,
}

impl<L: AcmLoader> RawrDecider<L> {
    pub fn new(acm_loader: L) -> Self {
        Self::with_config(acm_loader, AcmConfig::default())
    }

    pub fn with_config(acm_loader: L, config: AcmConfig) -> Self {
        let max_size = std::num::NonZeroUsize::new(config.max_lru_size.max(1))
            .unwrap_or_else(|| std::num::NonZeroUsize::new(25).unwrap());
        RawrDecider {
            acm_loader,
            cache: Arc::new(RwLock::new(LruCache::new(max_size))),
        }
    }
}

impl<L: AcmLoader> Decider for RawrDecider<L> {
    async fn decide(
        &self,
        account_ksuid: &Ksuid,
        principal_ksuid: &Ksuid,
        action: &str,
        resource_path: &str,
    ) -> Result<bool, PdpError> {
        // check cache first
        let acm = {
            let mut cache = self.cache.write().unwrap();
            cache.get(principal_ksuid).cloned()
        };

        let acm = match acm {
            Some(cached) => cached,
            None => {
                // pc load letter
                let acm = Arc::new(
                    self.acm_loader
                        .load(account_ksuid, principal_ksuid)
                        .await
                        .map_err(PdpError::from)?,
                );

                // thank u, next
                let mut cache = self.cache.write().unwrap();
                cache.put(*principal_ksuid, acm.clone());

                acm
            }
        };

        // no wammies! no wammies! no wammies!
        let granted = acm.enforce(action, resource_path);

        // may the odds be ever in your favor
        Ok(granted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    use rawr_pip::JsonPolicyLoader;
    use svix_ksuid::Ksuid;

    const ACCOUNT_KSUID: &str = "365SmM3pnQ5Flegvk5nA7leI5KL";
    const ADMIN_KSUID: &str = "35zt2PT8xuykGfG83dEp5cZY4AM";
    const NETWORK_ADMIN_KSUID: &str = "35zt2PHzXp3K5VzHTNuTbTgYNSl";
    const HAHA_BUSINESS_KSUID: &str = "35zt2LD8MyWdngVgdr4Qaqcpesb";

    #[tokio::test]
    async fn test_admin() {
        let loader = JsonPolicyLoader::default();
        let decider = RawrDecider::new(loader);
        let account_ksuid = Ksuid::from_str(ACCOUNT_KSUID).unwrap();
        let principal_ksuid = Ksuid::from_str(ADMIN_KSUID).unwrap();

        // you know you want to. dont fight it, it's okay.
        assert!(
            decider
                .decide(
                    &account_ksuid,
                    &principal_ksuid,
                    "never:gonna",
                    "give/you/up"
                )
                .await
                .unwrap()
        );
        assert!(
            decider
                .decide(
                    &account_ksuid,
                    &principal_ksuid,
                    "never:gonna",
                    "let/you/down"
                )
                .await
                .unwrap()
        );
        assert!(
            decider
                .decide(
                    &account_ksuid,
                    &principal_ksuid,
                    "never:gonna",
                    "run/around/and/desert/you"
                )
                .await
                .unwrap()
        );
        assert!(
            decider
                .decide(
                    &account_ksuid,
                    &principal_ksuid,
                    "never:gonna",
                    "say/goodbye"
                )
                .await
                .unwrap()
        );
        assert!(
            decider
                .decide(
                    &account_ksuid,
                    &principal_ksuid,
                    "never:gonna",
                    "tell/a/lie/and/hurt/you"
                )
                .await
                .unwrap()
        );
        assert!(
            decider
                .decide(
                    &account_ksuid,
                    &principal_ksuid,
                    "when:i",
                    "was/a/young/boy"
                )
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    async fn test_network_admin() {
        let loader = JsonPolicyLoader::default();
        let decider = RawrDecider::new(loader);
        let account_ksuid = Ksuid::from_str(ACCOUNT_KSUID).unwrap();
        let principal_ksuid = Ksuid::from_str(NETWORK_ADMIN_KSUID).unwrap();

        assert!(
            decider
                .decide(
                    &account_ksuid,
                    &principal_ksuid,
                    "user:GetProfile",
                    "user/nick"
                )
                .await
                .unwrap()
        );
        assert!(
            decider
                .decide(
                    &account_ksuid,
                    &principal_ksuid,
                    "deploy:ListDeployments",
                    "*"
                )
                .await
                .unwrap()
        );
        assert!(
            decider
                .decide(
                    &account_ksuid,
                    &principal_ksuid,
                    "network:CreateVLAN",
                    "VLAN-20"
                )
                .await
                .unwrap()
        );
        assert!(
            !decider
                .decide(
                    &account_ksuid,
                    &principal_ksuid,
                    "network:DeleteVLAN",
                    "mhq/VLAN-1"
                )
                .await
                .unwrap()
        );
        assert!(
            !decider
                .decide(
                    &account_ksuid,
                    &principal_ksuid,
                    "coffee:FillCup",
                    "french-press"
                )
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    async fn test_haha_business() {
        use rawr_pap::Role;
        use rawr_pip::AcmLoader;
        use std::fs;

        let loader = JsonPolicyLoader::default();
        let account_ksuid = Ksuid::from_str(ACCOUNT_KSUID).unwrap();
        let principal_ksuid = Ksuid::from_str(HAHA_BUSINESS_KSUID).unwrap();

        let acm = loader
            .load(&account_ksuid, &principal_ksuid)
            .await
            .expect("failed to load ACM for haha business");

        let file_path = format!("testdata/{}.json", HAHA_BUSINESS_KSUID);
        let contents =
            fs::read_to_string(&file_path).expect("failed to read haha business test data file");

        #[derive(serde::Deserialize)]
        struct PrincipalData {
            roles: Vec<Role>,
        }

        let roles: Vec<Role> = serde_json::from_str::<PrincipalData>(&contents)
            .map(|pd| pd.roles)
            .or_else(|_| serde_json::from_str(&contents))
            .expect("failed to parse haha business test data JSON");

        let mut allow_combinations = Vec::new();
        let mut deny_combinations = Vec::new();
        for role in &roles {
            for policy in &role.policies {
                for action in &policy.actions {
                    for resource in &policy.resources {
                        match policy.effect {
                            rawr_pap::Effect::Allow => {
                                allow_combinations.push((action.clone(), resource.clone()));
                            }
                            rawr_pap::Effect::Deny => {
                                deny_combinations.push((action.clone(), resource.clone()));
                            }
                        }
                    }
                }
            }
        }

        for (action, resource) in &allow_combinations {
            let result = acm.enforce(action, resource);
            let is_explicitly_denied =
                deny_combinations.contains(&(action.clone(), resource.clone()));

            if is_explicitly_denied {
                assert!(
                    !result,
                    "expected deny (overrides allow) for action: {}, resource: {}",
                    action, resource
                );
            } else {
                assert!(
                    result,
                    "expected allow for action: {}, resource: {} (may be overridden by deny pattern)",
                    action, resource
                );
            }
        }

        for (action, resource) in &deny_combinations {
            let granted = acm.enforce(action, resource);
            assert!(
                !granted,
                "expected deny for action: {}, resource: {}",
                action, resource
            );
        }

        let known_absent_combos = vec![
            ("nonexistent:Action", "nonexistent/resource"),
            ("fake:Service", "fake/path"),
            ("test:Operation", "test/data"),
            ("unknown:Method", "unknown/path"),
            ("missing:Call", "missing/item"),
        ];

        for (action, resource) in known_absent_combos {
            assert!(
                !acm.enforce(action, resource),
                "expected deny for action: {}, resource: {}",
                action,
                resource
            );
        }
    }
}
