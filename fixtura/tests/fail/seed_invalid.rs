use fake::Dummy;

#[derive(Dummy)]
struct User {
    id: u32,
}

#[fixtura::test(seed = "hello")]
fn bad_seed(user: User) {
    let _ = user;
}
