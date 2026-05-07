use fake::Dummy;

#[derive(Dummy)]
struct User {
    id: u32,
}

#[fixtura::test]
fn self_ref(#[with(id = user.id)] user: User) {
    let _ = user;
}
