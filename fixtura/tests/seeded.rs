use fake::Dummy;

#[derive(Dummy, Debug)]
struct User {
    id: u32,
    score: u64,
}

#[fixtura::test(seed = 42)]
fn seeded_values_match_manual_rng(user: User) {
    use fake::rand::SeedableRng;
    let mut rng = fake::rand::rngs::StdRng::seed_from_u64(42);
    let expected: User = fake::Faker.fake_with_rng(&mut rng);
    assert_eq!(user.id, expected.id);
    assert_eq!(user.score, expected.score);
}

#[tokio::test]
#[fixtura::inject(seed = 99)]
async fn seeded_inject_values_match_manual_rng(user: User) {
    use fake::rand::SeedableRng;
    let mut rng = fake::rand::rngs::StdRng::seed_from_u64(99);
    let expected: User = fake::Faker.fake_with_rng(&mut rng);
    assert_eq!(user.id, expected.id);
    assert_eq!(user.score, expected.score);
}
