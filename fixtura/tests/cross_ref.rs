#![allow(dead_code)]
/// Tests that #[fixtura(field = prior_arg.field)] resolves correctly at runtime.
/// Fixtura evaluates override expressions in top-to-bottom declaration order,
/// so earlier args are always in scope for later overrides.
use fake::Dummy;

#[derive(Dummy, Debug)]
struct User {
    id: u32,
}

#[derive(Dummy, Debug)]
struct Order {
    id: u32,
    user_id: u32,
    qty: u32,
}

#[derive(Dummy, Debug)]
struct LineItem {
    id: u32,
    order_id: u32,
    user_id: u32,
}

// A later arg can pin a field to a value from an earlier arg.
#[fixtura::test]
fn references_prior_arg(user: User, #[fixtura(user_id = user.id)] order: Order) {
    assert_eq!(order.user_id, user.id);
}

// The same prior arg can be referenced multiple times in one override.
#[fixtura::test]
fn same_prior_arg_twice(user: User, #[fixtura(user_id = user.id, qty = user.id)] order: Order) {
    assert_eq!(order.user_id, user.id);
    assert_eq!(order.qty, user.id);
}

// Multiple earlier args can each contribute to one override.
#[fixtura::test]
fn references_two_earlier_args(
    user: User,
    order: Order,
    #[fixtura(user_id = user.id, order_id = order.id)] line: LineItem,
) {
    assert_eq!(line.user_id, user.id);
    assert_eq!(line.order_id, order.id);
}

// Chained: c references b, b references a.
#[fixtura::test]
fn chained(
    user: User,
    #[fixtura(user_id = user.id)] order: Order,
    #[fixtura(user_id = user.id, order_id = order.id)] line: LineItem,
) {
    assert_eq!(order.user_id, user.id);
    assert_eq!(line.user_id, user.id);
    assert_eq!(line.order_id, order.id);
}

// Override expression can be a computed value, not just a field copy.
#[fixtura::test]
fn expression_over_prior_arg(
    order: Order,
    #[fixtura(qty = order.qty.saturating_add(1))] order2: Order,
) {
    assert_eq!(order2.qty, order.qty.saturating_add(1));
}

// A plain arg can appear between two override args; the last can reference both.
#[fixtura::test]
fn plain_between_override_args(
    user: User,
    order: Order,
    #[fixtura(user_id = user.id, order_id = order.id)] line: LineItem,
) {
    assert_eq!(line.user_id, user.id);
    assert_eq!(line.order_id, order.id);
}
