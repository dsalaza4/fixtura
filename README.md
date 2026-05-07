# fixtura

Declarative fake data injection for Rust tests.

Built on [`fake-rs`](https://github.com/cksac/fake-rs).

---

## Install

```toml
[dev-dependencies]
fixtura = "0.4.0"
fake = { version = "5", features = ["derive"] }
```

---

## The problem

Every test that needs fake data looks like this:

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

Multiply this across a test suite and you get walls of setup that obscure what the test actually asserts.

---

## The fix

```rust
#[fixtura::test]
fn order_belongs_to_user(
    user: User,
    #[with(user_id = user.id, status = "pending".to_string())]
    order: Order,
) {
    assert!(is_billable(&order));
    assert_eq!(order.user_id, user.id);
}
```

`#[fixtura::test]` injects a fake value for every argument. Use `#[with(...)]` to pin the fields your test cares about — everything else is randomized.

The only requirement is `#[derive(Dummy)]` on your types.

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
    total: f64,
}
```

---

## What you get

**Declarative.** The test signature is the setup. No boilerplate, no helper functions, no builder chains.

**Precise.** Pin exactly the fields your test depends on. Fixtura fakes the rest.

```rust
#[fixtura::test]
fn inactive_users_cannot_checkout(
    #[with(active = false)] user: User,
    order: Order,
) {
    assert!(checkout(&user, &order).is_err());
}
```

**Relational.** Reference earlier arguments in overrides to keep related objects coherent.

```rust
#[fixtura::test]
fn order_belongs_to_user(
    user: User,
    #[with(user_id = user.id)] order: Order,
) {
    assert_eq!(order.user_id, user.id);
}
```

**Async.** Use `#[fixtura::inject]` with any async test runner.

```rust
#[tokio::test]
#[fixtura::inject]
async fn payment_fails_for_inactive_user(
    #[with(active = false)] user: User,
    #[with(user_id = user.id)] order: Order,
) {
    assert!(process_payment(&user, &order).await.is_err());
}
```

**Compatible.** Composes with `#[should_panic]` and any other test attribute.

---

## Async tests

`#[fixtura::inject]` is the async counterpart to `#[fixtura::test]`. It handles arg injection but does not emit `#[test]` — the outer test attribute does that.

Place the runner above `#[fixtura::inject]`:

```rust
#[tokio::test]      // runs the async runtime
#[fixtura::inject]  // injects fake args
async fn my_test(user: User) { ... }
```

Everything works the same: `#[with(...)]`, cross-references, `#[should_panic]`.

Add tokio to your dev-dependencies to use it:

```toml
tokio = { version = "1", features = ["rt", "macros"] }
```

---

## Status

Early development — v0.4 available. Seeded randomness is coming. Feedback welcome — open an issue or start a discussion.
