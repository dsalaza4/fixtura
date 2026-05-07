use fake::Dummy;

#[derive(Dummy, Debug, PartialEq)]
struct User {
    id: u32,
    score: u64,
}

fn user_with_seed(seed: u64) -> User {
    use fake::rand::SeedableRng;
    use fake::Fake;
    fake::Faker.fake_with_rng(&mut fake::rand::rngs::StdRng::seed_from_u64(seed))
}

// Same seed always yields identical values — run twice and compare directly.
#[test]
fn same_seed_always_same_values() {
    assert_eq!(user_with_seed(42), user_with_seed(42));
    assert_eq!(user_with_seed(0), user_with_seed(0));
    assert_eq!(user_with_seed(u64::MAX), user_with_seed(u64::MAX));
}

// Different seeds yield different values (with overwhelming probability).
#[test]
fn different_seeds_yield_different_values() {
    assert_ne!(user_with_seed(1), user_with_seed(2));
}

// Fixtura's pinned seed matches the equivalent manual RNG call.
#[fixtura::test(seed = 42)]
fn seeded_test_matches_manual_rng(user: User) {
    assert_eq!(user, user_with_seed(42));
}

#[tokio::test]
#[fixtura::inject(seed = 99)]
async fn seeded_inject_matches_manual_rng(#[fixtura] user: User) {
    assert_eq!(user, user_with_seed(99));
}

// Multiple args with the same seed advance the RNG sequentially — both must match.
#[fixtura::test(seed = 7)]
fn seeded_multiple_args_are_deterministic(user1: User, user2: User) {
    use fake::rand::SeedableRng;
    let mut rng = fake::rand::rngs::StdRng::seed_from_u64(7);
    let expected1: User = fake::Faker.fake_with_rng(&mut rng);
    let expected2: User = fake::Faker.fake_with_rng(&mut rng);
    assert_eq!(user1, expected1);
    assert_eq!(user2, expected2);
}
