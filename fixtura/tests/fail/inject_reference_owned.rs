use fake::Dummy;

#[derive(Dummy)]
struct User {
    id: u32,
}

#[tokio::test]
#[fixtura::inject]
async fn test(#[fixtura] user: &User) {
    let _ = user;
}
