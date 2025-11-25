# rawr 🦖

> righteous authorization with rust, like the hipster gods intended

**rawr** is a blazing-fast, zero-nonsense authorization library for Rust
that will leave your access control game **dripping** with security rizz.

It's built on a trie-based pattern matching system with glob support, this bad
boy handles your Allow/Deny policies like a champ.

## peep the stats

rawr provides:
- ⚡ **lightning-fast authorization checks** using a trie data structure
- 🎯 **glob pattern matching** for actions and resources
- 🛡️ **role-based access control** with `Allow`/`Deny` policies

coming soon:
- **attribute-level conditions**

## rtfm

1. Add this to your `Cargo.toml`:

```toml
[dependencies]
authz = { path = "." }  # or from crates.io when we're famous
```

2. Create a `Role` with some `Policies`

```rust
use authz::{Role, Policy, Effect};

let role = Role::new("MHQ.NetworkAdmin", "The network whisperer")
    .with_policy(Policy::deny(
        vec!["networks:*".to_string()],
        vec!["VLAN-1".to_string()],
    ))
    .with_policy(Policy::allow(
        vec!["networks:GetVLAN".to_string()],
        vec!["*".to_string()],
    ))
    .with_policy(Policy::allow(
        vec!["networks:UpdateVLAN".to_string()],
        vec!["VLAN-20".to_string()],
    ))
    .with_policy(Policy::allow(
        vec!["networks:*".to_string()],
        vec!["VLAN-70".to_string()],
    ))
    .with_policy(Policy::allow(
        vec!["networks:AddVLANTag".to_string()],
        vec!["nick/lab/*".to_string()],
    ));
```

3. Apply the `Role` to an `ACM`. PIP problems, amirite?

```rust
use authz::ACM;

let mut acm = ACM::new();
acm.apply_role(&role);
```

4. Authorize against all the actions and resources you could ever desire.

```rust
// true
assert!(acm.authorized("networks:GetVLAN", "VLAN-50"));

// false
assert!(!acm.authorized("networks:DeleteVLAN", "VLAN-1"));

// true
assert!(acm.authorized("networks:UpdateVLAN", "VLAN-70"));

// true
assert!(acm.authorized("networks:AddVLANTag", "nick/lab/VLAN-70"));
```

## lez incompétentz

Here's the full workflow in one epic example:

```rust
use authz::{ACM, Role, Policy, Effect};

fn main() {
    // Step 1: Create a role with policies
    let role = Role::new("MHQ.Developer", "Can code, can deploy, can party")
        .with_policy(Policy::allow(
            vec!["code:Read".to_string(), "code:Write".to_string()],
            vec!["repos/*".to_string()],
        ))
        .with_policy(Policy::allow(
            vec!["deploy:*".to_string()],
            vec!["staging/*".to_string()],
        ))
        .with_policy(Policy::deny(
            vec!["deploy:*".to_string()],
            vec!["production/*".to_string()],
        ));

    // Step 2: Apply the role to ACM
    let mut acm = ACM::new();
    acm.apply_role(&role);

    // Step 3: Check authorization
    println!("Can read code? {}", acm.authorized("code:Read", "repos/my-app"));
    println!("Can deploy to staging? {}", acm.authorized("deploy:Rollout", "staging/web"));
    println!("Can deploy to prod? {}", acm.authorized("deploy:Rollout", "production/web"));
}
```

## fit check

rawr uses a **trie data structure** to store and match action/resource
patterns. When you call `authorized()`, it:

1. Splits the action by `:` and the resource by `/` to create segments
2. Checks the deny trie first (because explicit denies win)
3. If not denied, checks the allow trie
4. Uses glob pattern matching for each segment (so `*` matches anything,
`foo/*` matches `foo/bar`, etc.)

This means your authorization checks are **O(n)** where n is the depth of your
action/resource path, not the number of policies. That's what we call
**efficient**, my dude.

## API Reference

### `ACM`

An access control model with `allow`/`deny` role policies.

- `ACM::new()` - Create a new ACM
- `ACM::from_json(json_data)` - Load an ACM from JSON data containing roles
- `acm.apply_role(role)` - Apply a role's policies to the ACM
- `acm.authorized(action, resource)` - Check if an action on a resource is authorized
- `acm.allow(action, resource)` - Directly allow an action/resource (low-level)
- `acm.deny(action, resource)` - Directly deny an action/resource (low-level)

### `Role`

A role with a name, description, and policies.

- `Role::new(name, description)` - Create a new role
- `role.with_policy(policy)` - Add a policy to the role (returns self for chaining)

### `Policy`

A policy with an effect (`allow`/`deny`), actions, and resources.

- `Policy::allow(actions, resources)` - Create an `allow` policy
- `Policy::deny(actions, resources)` - Create a `deny` policy

### JSON Derulo

When loading from JSON, the format should be:

```json
{
  "roles": [
    {
      "name": "RoleName",
      "description": "Role description",
      "policies": [
        {
          "effect": "allow",
          "actions": ["action:pattern", "*"],
          "resources": ["resource/path", "*"]
        },
        {
          "effect": "deny",
          "actions": ["action:*"],
          "resources": ["resource/*"]
        }
      ]
    }
  ]
}
```

## code community

Want to make rawr even more righteous? PRs welcome, but keep the vibes high 
and the code clean. No cap.

## for the suits

Unlicensed. There's like, no rules here dude.
