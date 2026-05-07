use fake::Dummy;

#[derive(Dummy)]
struct User {
    id: u32,
}

#[derive(Dummy)]
struct Order {
    user_id: u32,
}

#[fixtura::test]
fn forward_ref(
    #[fixtura(user_id = order.user_id)] user: User,
    order: Order,
) {
    let _ = (user, order);
}
