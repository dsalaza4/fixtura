# fixtura

Declarative fake data injection for Rust tests.

Built on [`fake-rs`](https://github.com/cksac/fake-rs). Compatible with `#[tokio::test]`, `#[sqlx::test]`, and any other test macro.

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

Multiply this across a test suite and you get hundreds of lines of setup that obscure what the test actually asserts.

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

Fixtura generates fake values for every argument. Use `#[with(...)]` to pin the fields your test actually cares about — everything else is randomized.

---

## What you get

**Declarative.** The test signature is the setup. No boilerplate, no helper functions, no builder chains.

**Reproducible.** Pin a seed and every run produces identical values — useful when a CI failure needs to be reproduced locally.

```rust
#[fixtura::test(seed = 42)]
fn reproduces_on_every_machine(user: User, order: Order) {
    assert!(process(&user, &order).is_ok());
}
```

**Composable.** Works alongside `#[tokio::test]`, `#[sqlx::test]`, or any other macro. Arguments fixtura doesn't own stay in the signature untouched.

```rust
#[sqlx::test]
#[fixtura::inject]
async fn saves_to_db(
    pool: PgPool,                               // owned by sqlx
    #[fake] user: User,                         // owned by fixtura
    #[fake(user_id = user.id)] order: Order,    // owned by fixtura, field pinned
) {
    db::save_order(&pool, &user, &order).await.unwrap();
}
```

**Simple.** The only requirement is `#[derive(Dummy)]` from `fake-rs`. No custom traits, no new derives, no registration.

---

## Status

Early design phase. Feedback welcome — open an issue or start a discussion.
