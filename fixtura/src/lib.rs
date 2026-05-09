//! Declarative fake data injection for Rust tests, built on [`fake-rs`](https://crates.io/crates/fake).
//!
//! ## Quick start
//!
//! ```toml
//! [dev-dependencies]
//! fixtura = "0.8.3"
//! fake = { version = "5", features = ["derive"] }
//! ```
//!
//! Derive [`Dummy`](https://docs.rs/fake/latest/fake/trait.Dummy.html) on your types, then
//! declare them as test arguments:
//!
//! ```rust,ignore
//! use fake::Dummy;
//!
//! #[derive(Dummy)]
//! struct User { id: u32, name: String, active: bool }
//!
//! #[fixtura::test]
//! fn active_user_can_login(#[fixtura(active = true)] user: User) {
//!     assert!(user.active);
//! }
//! ```
//!
//! ## Attributes
//!
//! - [`#[fixtura::test]`](macro@test) — sync tests; injects all arguments and emits `#[test]`
//! - [`#[fixtura::inject]`](macro@inject) — async or framework-paired tests; injects marked
//!   arguments without emitting `#[test]`
//!
//! ## Field overrides
//!
//! Pin specific fields with `#[fixtura(field = expr)]`; fixtura fakes the rest:
//!
//! ```rust,ignore
//! #[fixtura::test]
//! fn order_belongs_to_user(
//!     user: User,
//!     #[fixtura(user_id = user.id, status = "pending".to_string())] order: Order,
//! ) {
//!     assert_eq!(order.user_id, user.id);
//! }
//! ```
//!
//! ## Reproducible failures
//!
//! Every run uses a seeded RNG. On failure the seed is printed; paste it back to replay:
//!
//! ```rust,ignore
//! #[fixtura::test(seed = 8317492031748291)]
//! fn my_test(user: User) { ... }
//! ```
//!
//! See the [repository README](https://github.com/dsalaza4/fixtura) for the full guide.

#[doc(inline)]
pub use fixtura_macros::inject;
#[doc(inline)]
pub use fixtura_macros::test;
