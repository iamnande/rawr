use rawr_pap::Role;

/*
* JSON derulo (static)
*/
pub struct JsonPolicyLoader;

impl JsonPolicyLoader {
    pub fn new() -> Self {
        JsonPolicyLoader
    }
}

impl PolicyLoader for JsonPolicyLoader {
    fn load_from_str(&self, data: &str) -> Result<Role, serde_json::Error> {
        let role: Role = serde_json::from_str(data)?;
        Ok(role)
    }
}

impl AcmLoader for JsonPolicyLoader {
    fn load(&self, principal_ksuid: &Ksuid) -> Result<Acm, PipError> {
        let role: Role = serde_json::from_str(&self.json_data)
            .map_err(|e| PipError::AcmNotFound(e.to_string()))?;

        // Build ACM from the role
        let mut acm = Acm::new();
        acm.apply_role(&role);

        Ok(acm)
    }
}

pub fn load_roles_from_json(json_data: &str) -> Result<Role, serde_json::Error> {
    let loader = JsonPolicyLoader::new();
    loader.load_from_str(json_data)
}
