#![allow(dead_code)]

use fake::Dummy;

// --- Domain types ---

#[derive(Dummy, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
    active: bool,
}

#[derive(Dummy, Debug)]
struct Product {
    id: u32,
    name: String,
    price_cents: u32,
    in_stock: bool,
}

#[derive(Dummy, Debug)]
struct Order {
    id: u32,
    user_id: u32,
    product_id: u32,
    quantity: u32,
}

// --- Business logic under test ---

fn display_name(user: &User) -> String {
    format!("@{}", user.name.to_lowercase().replace(' ', "_"))
}

fn label(user: &User) -> String {
    format!("{} <{}>", user.name, user.email)
}

fn subtotal_cents(order: &Order, product: &Product) -> u32 {
    order.quantity.saturating_mul(product.price_cents)
}

fn receipt(user: &User, order: &Order, product: &Product) -> String {
    format!(
        "Order #{} for {} — {} x {} = ${:.2}",
        order.id,
        user.name,
        order.quantity,
        product.name,
        subtotal_cents(order, product) as f64 / 100.0,
    )
}

// --- Tests ---

#[fixtura::test]
fn display_name_is_always_lowercase(user: User) {
    let name = display_name(&user);
    assert_eq!(name, name.to_lowercase());
}

#[fixtura::test]
fn display_name_starts_with_at(user: User) {
    assert!(display_name(&user).starts_with('@'));
}

#[fixtura::test]
fn label_contains_name_and_email(user: User) {
    let l = label(&user);
    assert!(l.contains(&user.name));
    assert!(l.contains(&user.email));
}

#[fixtura::test]
fn subtotal_is_zero_when_quantity_is_zero(product: Product) {
    let order = Order {
        quantity: 0,
        id: 1,
        user_id: 1,
        product_id: product.id,
    };
    assert_eq!(subtotal_cents(&order, &product), 0);
}

#[fixtura::test]
fn receipt_contains_order_id(user: User, order: Order, product: Product) {
    assert!(receipt(&user, &order, &product).contains(&order.id.to_string()));
}

#[fixtura::test]
fn receipt_contains_user_name(user: User, order: Order, product: Product) {
    assert!(receipt(&user, &order, &product).contains(&user.name));
}

#[fixtura::test]
fn receipt_contains_product_name(user: User, order: Order, product: Product) {
    assert!(receipt(&user, &order, &product).contains(&product.name));
}

#[fixtura::test]
#[should_panic(expected = "not implemented")]
fn preserves_should_panic(user: User) {
    let _ = user;
    unimplemented!()
}
