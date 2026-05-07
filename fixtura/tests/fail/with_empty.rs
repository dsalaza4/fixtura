use fake::Dummy;

#[derive(Dummy)]
struct User {
    name: String,
}

#[fixtura::test]
fn test(#[with()] user: User) {
    let _ = user;
}
