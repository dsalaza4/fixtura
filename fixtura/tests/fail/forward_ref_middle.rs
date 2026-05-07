use fake::Dummy;

#[derive(Dummy)]
struct User {
    id: u32,
}

#[derive(Dummy)]
struct Order {
    user_id: u32,
}

#[derive(Dummy)]
struct Product {
    order_id: u32,
}

#[fixtura::test]
fn forward_ref_middle(
    user: User,
    #[fixtura(order_id = product.order_id)] order: Order,
    product: Product,
) {
    let _ = (user, order, product);
}
