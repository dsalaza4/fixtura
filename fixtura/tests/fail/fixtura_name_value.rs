use fake::Dummy;

#[derive(Dummy)]
struct User {
    id: u32,
}

// #[fixtura = value] (NameValue form) is not a valid syntax.
#[fixtura::test]
fn test(#[fixtura = 42] user: User) {
    let _ = user;
}
