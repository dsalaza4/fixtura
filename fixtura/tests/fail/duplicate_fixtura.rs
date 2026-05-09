use fake::Dummy;

#[derive(Dummy)]
struct User {
    id: u32,
    active: bool,
}

#[fixtura::test]
fn test(
    #[fixtura(id = 1)]
    #[fixtura(active = false)]
    user: User,
) {
    let _ = user;
}
