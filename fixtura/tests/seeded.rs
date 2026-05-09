/// Tests that fixtura's seeded RNG is deterministic, matches manual StdRng usage,
/// advances sequentially across multiple args, and is unaffected by pass-through args
/// or field overrides.
use fake::Dummy;

#[derive(Dummy, Debug, PartialEq)]
struct User {
    id: u32,
    score: u64,
}

fn fake_with_seed(seed: u64) -> User {
    use fake::rand::SeedableRng;
    use fake::Fake;
    fake::Faker.fake_with_rng(&mut fake::rand::rngs::StdRng::seed_from_u64(seed))
}

// Same seed always produces identical values.
#[test]
fn same_seed_same_values() {
    assert_eq!(fake_with_seed(42), fake_with_seed(42));
    assert_eq!(fake_with_seed(0), fake_with_seed(0));
    assert_eq!(fake_with_seed(u64::MAX), fake_with_seed(u64::MAX));
}

// Different seeds produce different values (with overwhelming probability).
#[test]
fn different_seeds_different_values() {
    assert_ne!(fake_with_seed(1), fake_with_seed(2));
}

// fixtura::test with a pinned seed produces the exact same value as manual StdRng.
#[fixtura::test(seed = 42)]
fn seeded_test_matches_manual(user: User) {
    assert_eq!(user, fake_with_seed(42));
}

// fixtura::inject with a pinned seed matches manual StdRng.
#[tokio::test]
#[fixtura::inject(seed = 99)]
async fn seeded_inject_matches_manual(#[fixtura] user: User) {
    assert_eq!(user, fake_with_seed(99));
}

// Multiple args advance the RNG sequentially; each matches the corresponding manual advance.
#[fixtura::test(seed = 7)]
fn multiple_args_advance_rng_sequentially(first: User, second: User) {
    use fake::rand::SeedableRng;
    let mut rng = fake::rand::rngs::StdRng::seed_from_u64(7);
    let e1: User = fake::Faker.fake_with_rng(&mut rng);
    let e2: User = fake::Faker.fake_with_rng(&mut rng);
    assert_eq!(first, e1);
    assert_eq!(second, e2);
}

// Two sequential args differ: the RNG advances between them.
#[fixtura::test(seed = 1)]
fn sequential_args_differ(a: User, b: User) {
    assert_ne!(a, b);
}

// Seed + field override: the base struct is seeded, then the override is applied on top.
// The non-overridden field (score) still matches the seeded fake.
#[fixtura::test(seed = 55)]
fn seeded_with_override(#[fixtura(id = 9999u32)] user: User) {
    assert_eq!(user.id, 9999);
    assert_eq!(user.score, fake_with_seed(55).score);
}

// Pass-through args do not consume the RNG; the owned arg still matches the manual seed.
#[fixtura::inject(seed = 13)]
async fn seeded_with_passthrough(caller: u32, #[fixtura] user: User) -> (u32, User) {
    (caller, user)
}

#[tokio::test]
async fn pass_through_does_not_consume_rng() {
    let (received_caller, user) = seeded_with_passthrough(0).await;
    assert_eq!(received_caller, 0);
    assert_eq!(user, fake_with_seed(13));
}
