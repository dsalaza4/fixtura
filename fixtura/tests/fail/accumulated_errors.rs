use fake::Dummy;

#[derive(Dummy)]
struct User {
    id: u32,
}

#[derive(Dummy)]
struct Order {
    id: u32,
}

// Both args have errors simultaneously; fixtura must report both, not stop at the first.
#[fixtura::test]
fn test(user: &User, order: &Order) {
    let _ = (user, order);
}
