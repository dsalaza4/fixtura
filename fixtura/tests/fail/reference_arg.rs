use fake::Dummy;

#[derive(Dummy)]
struct User {
    name: String,
}

#[fixtura::test]
fn test(user: &User) {}
