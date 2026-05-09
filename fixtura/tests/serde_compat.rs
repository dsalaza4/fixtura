#![allow(dead_code)]
/// Tests that #[derive(Serialize, Deserialize, Dummy)] composes with fixtura without conflict,
/// and that field overrides work correctly on serde-annotated types.
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Dummy, Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    active: bool,
}

// Deriving Dummy alongside Serialize/Deserialize does not prevent plain injection.
#[fixtura::test]
fn serde_derives_compose(user: User) {
    let _ = user;
}

// Field overrides work on serde-annotated types exactly as on plain Dummy types.
#[fixtura::test]
fn override_on_serde_type(#[fixtura(active = false)] user: User) {
    assert!(!user.active);
}

// Multiple overrides on a serde type work together.
#[fixtura::test]
fn multiple_overrides_on_serde_type(#[fixtura(active = true, id = 7u32)] user: User) {
    assert!(user.active);
    assert_eq!(user.id, 7);
}

// serde round-trip preserves fixtura-set field values.
#[fixtura::test]
fn serde_roundtrip_preserves_overrides(#[fixtura(active = true, id = 99u32)] user: User) {
    let json = serde_json::to_string(&user).unwrap();
    let back: User = serde_json::from_str(&json).unwrap();
    assert!(back.active);
    assert_eq!(back.id, 99);
}

// Cross-arg reference works on serde-annotated types.
#[derive(Dummy, Debug, Serialize, Deserialize)]
struct Order {
    id: u32,
    user_id: u32,
}

#[fixtura::test]
fn cross_ref_on_serde_types(user: User, #[fixtura(user_id = user.id)] order: Order) {
    assert_eq!(order.user_id, user.id);
}
