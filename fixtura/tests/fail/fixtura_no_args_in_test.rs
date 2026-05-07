use fake::Dummy;

#[derive(Dummy)]
struct User {
    id: u32,
}

#[fixtura::test]
fn test(#[fixtura] user: User) {
    let _ = user;
}
