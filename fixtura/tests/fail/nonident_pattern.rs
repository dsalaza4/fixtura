use fake::Dummy;

#[derive(Dummy)]
struct User {
    id: u32,
}

#[fixtura::test]
fn test((a, b): (u32, u32)) {
    let _ = (a, b);
}
