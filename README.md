# fixtura

[![crates.io](https://img.shields.io/crates/v/fixtura.svg)](https://crates.io/crates/fixtura)
[![docs.rs](https://docs.rs/fixtura/badge.svg)](https://docs.rs/fixtura)
[![license](https://img.shields.io/crates/l/fixtura.svg)](LICENSE)

Declarative fake data injection for Rust tests, built on [`fake-rs`](https://github.com/cksac/fake-rs).

---

Instead of this:

```rust
#[test]
fn order_belongs_to_user() {
    let user: User = Faker.fake();
    let mut order: Order = Faker.fake();
    order.user_id = user.id;
    order.status = "pending".to_string();

    assert!(is_billable(&order));
    assert_eq!(order.user_id, user.id);
}
```

Write this:

```rust
#[fixtura::test]
fn order_belongs_to_user(
    user: User,
    #[fixtura(user_id = user.id, status = "pending".to_string())]
    order: Order,
) {
    assert!(is_billable(&order));
    assert_eq!(order.user_id, user.id);
}
```

The test signature is the setup.

---

## Install

```toml
[dev-dependencies]
fixtura = "0.7.1"
fake = { version = "5", features = ["derive"] }
```

Add `#[derive(Dummy)]` to any type you want injected:

```rust
use fake::Dummy;

#[derive(Dummy)]
struct User {
    id: u32,
    name: String,
    active: bool,
}

#[derive(Dummy)]
struct Order {
    id: u32,
    user_id: u32,
    status: String,
}
```

---

## Usage

**Inject any `Dummy` type as a test argument:**

```rust
#[fixtura::test]
fn user_name_is_never_empty(user: User) {
    assert!(!user.name.is_empty());
}
```

**Pin the fields your test cares about — fixtura fakes the rest:**

```rust
#[fixtura::test]
fn inactive_users_cannot_checkout(
    #[fixtura(active = false)] user: User,
    order: Order,
) {
    assert!(checkout(&user, &order).is_err());
}
```

**Reference earlier arguments to keep data coherent:**

```rust
#[fixtura::test]
fn order_belongs_to_user(
    user: User,
    #[fixtura(user_id = user.id)] order: Order,
) {
    assert_eq!(order.user_id, user.id);
}
```

---

## Async tests

Use `#[fixtura::inject]` alongside your async runner. It injects args without emitting `#[test]` — the outer attribute does that.

Mark each arg fixtura should own with `#[fixtura]` or `#[fixtura(...)]`. Everything works the same: field overrides, cross-references, `#[should_panic]`.

```rust
#[tokio::test]
#[fixtura::inject]
async fn payment_fails_for_inactive_user(
    #[fixtura(active = false)] user: User,
    #[fixtura(user_id = user.id)] order: Order,
) {
    assert!(process_payment(&user, &order).await.is_err());
}
```

---

## Framework passthrough

When combining fixtura with another injecting framework (e.g. `sqlx::test`), mark only the args fixtura should own. Unmarked args stay in the signature for the other framework to inject.

```rust
#[sqlx::test]
#[fixtura::inject]
async fn saves_to_db(
    pool: PgPool,                                // sqlx owns
    #[fixtura] user: User,                       // fixtura owns
    #[fixtura(user_id = user.id)] order: Order,  // fixtura owns, with override
) {
    db::save_order(&pool, &user, &order).await.unwrap();
}
```

---

## Reproducible failures

Every test uses a seeded RNG. The seed is printed only on failure, so passing tests are silent:

```
---- order_belongs_to_user stdout ----
[fixtura] seed = 8317492031748291
```

Paste it back to replay the exact same values:

```rust
#[fixtura::test(seed = 8317492031748291)]
fn order_belongs_to_user(user: User, order: Order) { ... }
```

Pin a seed permanently for fully deterministic tests:

```rust
#[fixtura::test(seed = 42)]
fn my_test(user: User) { ... }
```

Works the same with `#[fixtura::inject(seed = 42)]`.

---

## IDE support

rust-analyzer provides type-checking and syntax highlighting inside `#[fixtura(...)]` overrides. Field name completions are not available — there is no stable mechanism for proc-macro crates to provide LSP completions inside attribute arguments. Mistyped field names surface at compile time as ordinary type errors.
