#![allow(dead_code)]
/// Tests that #[fixtura::test] expands correctly for various argument configurations
/// and composes with other test attributes. No field overrides or cross-references —
/// those live in overrides.rs and cross_ref.rs.
use fake::Dummy;

#[derive(Dummy, Debug)]
struct User {
    id: u32,
    name: String,
    active: bool,
}

// Single arg: injected and accessible in the body.
#[fixtura::test]
fn single_arg(user: User) {
    let _ = user;
}

// Multiple args: each is injected independently.
#[fixtura::test]
fn multiple_args(a: User, b: User) {
    let _ = (a, b);
}

// Primitive Dummy types work, not just structs.
#[fixtura::test]
fn primitive_arg(n: u32, s: String, flag: bool) {
    let _ = (n, s, flag);
}

// No-arg test: macro skips the rng preamble entirely and the body runs.
#[fixtura::test]
fn no_args() {}

// #[should_panic] composes correctly.
#[fixtura::test]
#[should_panic(expected = "boom")]
fn composes_with_should_panic(_user: User) {
    panic!("boom");
}
