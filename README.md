# fixtura

Declarative fake data injection for Rust tests.

Built on [`fake-rs`](https://github.com/cksac/fake-rs).

---

## Install

```toml
[dev-dependencies]
fixtura = "0.7.0"
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
    #[fixtura(user_id = user.id, status = "pending".to_string())]
    order: Order,
) {
    assert!(is_billable(&order));
    assert_eq!(order.user_id, user.id);
}
```

`#[fixtura::test]` injects a fake value for every argument. Use `#[fixtura(...)]` to pin the fields your test cares about — everything else is randomized.

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
    #[fixtura(active = false)] user: User,
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
    #[fixtura(user_id = user.id)] order: Order,
) {
    assert_eq!(order.user_id, user.id);
}
```

**Async.** Use `#[fixtura::inject]` with any async test runner.

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

**Compatible.** Composes with `#[should_panic]` and any other test attribute.

**Reproducible.** Every test prints its seed on failure — paste it back to replay the exact same values.

```rust
#[fixtura::test]
fn my_test(user: User) { ... }
// On failure: [fixtura] seed = 8317492031748291

#[fixtura::test(seed = 8317492031748291)]
fn my_test(user: User) { ... }
// Replays the exact same user
```

---

## Seeded randomness

Every `#[fixtura::test]` and `#[fixtura::inject]` test automatically uses a seeded RNG. The seed is printed via `eprintln!` — Rust's test runner captures it and shows it only when the test fails, so passing tests are noise-free.

**Replay a failure** by pinning the seed shown in the output:

```rust
#[fixtura::test(seed = 8317492031748291)]
fn my_test(user: User) { ... }
```

**Pin a seed permanently** for deterministic tests that must always use the same data:

```rust
#[fixtura::test(seed = 42)]
fn my_test(user: User) { ... }
```

Works identically with `#[fixtura::inject]`:

```rust
#[tokio::test]
#[fixtura::inject(seed = 42)]
async fn my_test(#[fixtura] user: User) { ... }
```

---

## Async tests

`#[fixtura::inject]` is the async counterpart to `#[fixtura::test]`. It handles arg injection but does not emit `#[test]` — the outer test attribute does that.

Place the runner above `#[fixtura::inject]`:

```rust
#[tokio::test]      // runs the async runtime
#[fixtura::inject]  // injects fake args
async fn my_test(#[fixtura] user: User) { ... }
```

Mark each arg fixtura should own with `#[fixtura]` or `#[fixtura(...)]`. Everything works the same: field overrides, cross-references, `#[should_panic]`.

Add tokio to your dev-dependencies to use it:

```toml
tokio = { version = "1", features = ["rt", "macros"] }
```

---

## Framework passthrough

When combining fixtura with another injecting framework (e.g. `sqlx::test`), mark only the args fixtura should own. Unmarked args stay in the signature for the other framework to inject.

```rust
#[sqlx::test]
#[fixtura::inject]
async fn saves_to_db(
    pool: PgPool,                                   // sqlx owns — untouched
    #[fixtura] user: User,                          // fixtura owns
    #[fixtura(user_id = user.id)] order: Order,     // fixtura owns, with override
) {
    db::save_order(&pool, &user, &order).await.unwrap();
}

```

---

## IDE support

rust-analyzer provides syntax highlighting and type-checking inside `#[fixtura(...)]` overrides. Field-name completions are not available — rust-analyzer does not offer completions inside attribute arguments for third-party attributes. Errors such as mistyped field names will surface at compile time as normal type errors.

---

## Status

Early development — v0.7 available. Feedback welcome — open an issue or start a discussion.
