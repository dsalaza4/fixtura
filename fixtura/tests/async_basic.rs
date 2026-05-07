#![allow(dead_code)]

use fake::Dummy;

#[derive(Dummy, Debug)]
struct User {
    id: u32,
    name: String,
    active: bool,
}

#[derive(Dummy, Debug)]
struct Order {
    id: u32,
    user_id: u32,
    status: String,
}

#[tokio::test]
#[fixtura::inject]
async fn async_plain_injection(user: User) {
    assert!(user.id < u32::MAX);
}

#[tokio::test]
#[fixtura::inject]
async fn async_with_override(#[with(active = false)] user: User) {
    assert!(!user.active);
}

#[tokio::test]
#[fixtura::inject]
async fn async_cross_reference(user: User, #[with(user_id = user.id)] order: Order) {
    assert_eq!(order.user_id, user.id);
}

#[tokio::test]
#[fixtura::inject]
async fn async_multiple_args(user: User, order: Order) {
    assert!(user.id < u32::MAX);
    assert!(order.id < u32::MAX);
}

#[tokio::test]
#[fixtura::inject]
#[should_panic(expected = "not implemented")]
async fn async_preserves_should_panic(user: User) {
    let _ = user;
    unimplemented!()
}
