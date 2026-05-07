use fake::Dummy;

#[derive(Dummy)]
struct User {
    id: u32,
}

#[fixtura::test]
async fn async_test(user: User) {
    let _ = user;
}
