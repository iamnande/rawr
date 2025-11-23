use authz::{ACM, Role, Policy, Effect};

#[test]
fn test_role_policy_application() {
    let mut acm = ACM::new();
    let role = Role::new("MHQ.NetworkAdmin", "Lorem Ipsum.")
        .with_policy(Policy {
            effect: Effect::Deny,
            actions: vec!["networks:*".into()],
            resources: vec!["VLAN-1".into()],
        })
        .with_policy(Policy {
            effect: Effect::Allow,
            actions: vec![
                "networks:GetVLAN".into(),
            ],
            resources: vec![
                "*".into(),
            ],
        })
        .with_policy(Policy {
            effect: Effect::Allow,
            actions: vec![
                "networks:UpdateVLAN".into(),
            ],
            resources: vec![
                "VLAN-20".into(),
            ],
        })
        .with_policy(Policy {
            effect: Effect::Allow,
            actions: vec![
                "networks:*".into(),
            ],
            resources: vec![
                "VLAN-70".into(),
            ],
        })
        .with_policy(Policy {
            effect: Effect::Allow,
            actions: vec![
                "networks:AddVLANTag".into(),
            ],
            resources: vec![
                "nick/lab/*".into(),
            ],
        })
        .with_policy(Policy {
            effect: Effect::Allow,
            actions: vec![
                "calendar:Get*".into(),
            ],
            resources: vec![
                "laura/*".into(),
            ],
        });

    acm.apply_role(&role);

    let cases = vec![
        ("networks:GetVLAN", "VLAN-50", true),
        ("networks:UpdateVLAN", "VLAN-20", true),
        ("networks:UpdateVLAN", "VLAN-30", false),
        ("networks:GetVLAN", "VLAN-1", false),
        ("networks:DeleteVLAN", "VLAN-70", true),
        ("networks:AddVLANTag", "nick/lab/VLAN-70", true),
        ("calendar:GetCalendar", "laura/shared-family-calendar", true),
    ];

    for (action, resource, expected) in cases {
        let granted = acm.authorized(action, resource);
        let message = format!(
            "[AUTHZ] action: {}, resource: {}, expected: {}, granted: {}",
            action, resource, expected, granted
        );
        assert_eq!(granted, expected, "{}", message);
        println!("{}", message);
    }
}
