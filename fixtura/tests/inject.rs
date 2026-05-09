#![allow(dead_code)]
/// Tests specific to #[fixtura::inject]: sync functions, pass-through args,
/// inject with only pass-throughs (no rng preamble), and async composition.
use fake::Dummy;

#[derive(Dummy, Debug)]
struct User {
    id: u32,
    active: bool,
}

#[derive(Dummy, Debug)]
struct Order {
    id: u32,
    user_id: u32,
}

// --- Sync inject ---

// inject works on non-async functions.
#[fixtura::inject]
fn sync_owned(pass: u32, #[fixtura] user: User) -> (u32, u32) {
    (pass, user.id)
}

// Pass-through is provided by the caller; owned arg is injected.
#[test]
fn sync_inject_pass_through_provided_by_caller() {
    let (received, _) = sync_owned(42);
    assert_eq!(received, 42);
}

// Owned arg is available as a local binding in a sync inject function.
#[test]
fn sync_inject_owned_arg_is_accessible() {
    let (_, user_id) = sync_owned(0);
    let _ = user_id;
}

// inject with only pass-through args: no owned args, no rng preamble generated.
#[fixtura::inject]
fn only_passthroughs(a: u32, b: u32) -> u32 {
    a + b
}

#[test]
fn inject_only_passthroughs_no_rng() {
    assert_eq!(only_passthroughs(3, 4), 7);
}

// --- Async inject ---

// Basic async inject: owned arg is available.
#[tokio::test]
#[fixtura::inject]
async fn async_owned_arg_available(#[fixtura] user: User) {
    let _ = user;
}

// Override in inject mode applies correctly.
#[tokio::test]
#[fixtura::inject]
async fn override_in_inject(#[fixtura(active = false)] user: User) {
    assert!(!user.active);
}

// Cross-reference in inject mode resolves correctly.
#[tokio::test]
#[fixtura::inject]
async fn cross_ref_in_inject(#[fixtura] user: User, #[fixtura(user_id = user.id)] order: Order) {
    assert_eq!(order.user_id, user.id);
}

// #[should_panic] composes with inject.
#[tokio::test]
#[fixtura::inject]
#[should_panic(expected = "boom")]
async fn should_panic_in_inject(#[fixtura] _user: User) {
    panic!("boom");
}

// --- Pass-through args: runtime behavior ---

// A pass-through ident referenced in #[fixtura(...)] override resolves from the caller.
#[fixtura::inject]
async fn pass_through_in_override(
    caller_id: u32,
    #[fixtura(user_id = caller_id)] order: Order,
) -> u32 {
    order.user_id
}

#[tokio::test]
async fn pass_through_drives_override() {
    assert_eq!(pass_through_in_override(99).await, 99);
    assert_eq!(pass_through_in_override(0).await, 0);
}

// Reference-typed pass-through arg is left untouched in the signature.
#[fixtura::inject]
async fn ref_pass_through(pool: &u32, #[fixtura] _user: User) -> u32 {
    *pool
}

#[tokio::test]
async fn ref_pass_through_received_from_caller() {
    assert_eq!(ref_pass_through(&55).await, 55);
}
