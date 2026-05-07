use fake::Dummy;

#[derive(Dummy)]
struct User {
    id: u32,
}

#[fixtura::inject(seed = "hello")]
async fn bad_seed_inject(user: User) {
    let _ = user;
}
