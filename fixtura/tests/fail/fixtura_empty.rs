use fake::Dummy;

#[derive(Dummy)]
struct User {
    name: String,
}

#[fixtura::test]
fn test(#[fixtura()] user: User) {
    let _ = user;
}
