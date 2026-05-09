use fake::Dummy;

#[derive(Dummy)]
struct User {
    id: u32,
}

// Self-reference in inject mode: an arg cannot reference itself in its own override.
#[tokio::test]
#[fixtura::inject]
async fn test(#[fixtura(id = user.id)] user: User) {
    let _ = user;
}
