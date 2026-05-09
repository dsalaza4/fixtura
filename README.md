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

**The test signature is the setup.**

---

## Features

- Inject any `Dummy` type as a test argument — no setup boilerplate
- Pin specific fields with `#[fixtura(field = value)]`, fake the rest
- Reference earlier args in overrides: `user_id = user.id`
- Nested field paths: `address.city = "Portland".to_string()`
- Seeded RNG — seed printed on failure, replayable on demand
- Async support via `#[fixtura::inject]` alongside any async runner
- Framework passthrough — compose with `sqlx::test`, `axum_test`, and others

---

## Installation

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

### Quick start

```rust
use fake::Dummy;

#[derive(Dummy)]
struct User { id: u32, name: String, active: bool }

#[fixtura::test]
fn active_user_can_login(#[fixtura(active = true)] user: User) {
    assert!(user.active);
}
```

That's it.

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

**Nested field paths work too:**

```rust
#[fixtura::test]
fn delivery_uses_city(
    #[fixtura(address.city = "Portland".to_string())] profile: Profile,
) {
    assert_eq!(profile.address.city, "Portland");
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

**Chain references across multiple args:**

```rust
#[fixtura::test]
fn line_item_links_to_user_and_order(
    user: User,
    #[fixtura(user_id = user.id)] order: Order,
    #[fixtura(user_id = user.id, order_id = order.id)] line: LineItem,
) {
    assert_eq!(line.user_id, user.id);
    assert_eq!(line.order_id, order.id);
}
```

---

## Async tests

Use `#[fixtura::inject]` alongside your async runner. It injects args without emitting `#[test]` — the outer attribute handles that.

Mark each arg fixtura should own with `#[fixtura]` or `#[fixtura(...)]`. Field overrides, cross-references, and `#[should_panic]` all work the same.

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

Every test uses a seeded RNG. The seed is printed only on failure, so passing tests stay silent:

```
---- order_belongs_to_user stdout ----
[fixtura] seed = 8317492031748291
```

Paste it back to replay the exact same values:

```rust
#[fixtura::test(seed = 8317492031748291)]
fn order_belongs_to_user(user: User, order: Order) { ... }
```

> [!TIP]
> Pin a seed permanently for fully deterministic tests:
> ```rust
> #[fixtura::test(seed = 42)]
> fn my_test(user: User) { ... }
> ```
> Works the same with `#[fixtura::inject(seed = 42)]`.

---

## IDE support

> [!NOTE]
> rust-analyzer provides type-checking and syntax highlighting inside `#[fixtura(...)]` overrides. Field name completions are not available — there is no stable mechanism for proc-macro crates to provide LSP completions inside attribute arguments. Mistyped field names surface at compile time as ordinary type errors.
