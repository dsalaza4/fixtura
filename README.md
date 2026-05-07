# fixtura

Declarative fake data injection for Rust tests.

Built on [`fake-rs`](https://github.com/cksac/fake-rs).

---

## Install

```toml
[dev-dependencies]
fixtura = "0.1"
fake = { version = "5", features = ["derive"] }
```

---

## The problem

Every test that needs fake data looks like this:

```rust
#[test]
fn user_label_format() {
    let user: User = Faker.fake();
    assert!(label(&user).contains(&user.name));
    assert!(label(&user).contains(&user.email));
}
```

Multiply this across a test suite and you get walls of setup that obscure what the test actually asserts.

---

## The fix

```rust
#[fixtura::test]
fn user_label_format(user: User) {
    assert!(label(&user).contains(&user.name));
    assert!(label(&user).contains(&user.email));
}
```

Fixtura injects a fake value for every argument. The only requirement is `#[derive(Dummy)]` on your types — no custom traits, no registration, no boilerplate.

```rust
use fake::Dummy;

#[derive(Dummy)]
struct User {
    name: String,
    email: String,
}
```

---

## What you get

**Declarative.** The test signature is the setup.

**Simple.** Works with any type that derives `Dummy`. No configuration required.

**Compatible.** `#[fixtura::test]` composes with `#[should_panic]` and any other test attribute.

```rust
#[fixtura::test]
#[should_panic(expected = "inactive")]
fn inactive_user_panics(user: User) {
    process(&user).unwrap();
}
```

---

## Status

Early development — v0.1 is available. Field overrides, seeded randomness, and async support are coming. Feedback welcome — open an issue or start a discussion.
