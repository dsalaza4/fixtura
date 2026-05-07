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

async fn find_user(id: u32) -> Option<User> {
    if id == 0 {
        None
    } else {
        Some(User {
            id,
            name: "stub".into(),
            active: true,
        })
    }
}

async fn submit_order(order: &Order) -> Result<u32, &'static str> {
    if order.status.is_empty() {
        Err("status required")
    } else {
        Ok(order.id)
    }
}

async fn activate_user(user: &mut User) {
    user.active = true;
}

#[tokio::test]
#[fixtura::inject]
async fn find_user_returns_none_for_zero_id(_user: User) {
    assert!(find_user(0).await.is_none());
}

#[tokio::test]
#[fixtura::inject]
async fn find_user_returns_user_for_nonzero_id(user: User) {
    let found = find_user(user.id.max(1)).await;
    assert!(found.is_some());
}

#[tokio::test]
#[fixtura::inject]
async fn submit_order_succeeds_with_nonempty_status(
    #[fixtura(status = "pending".to_string())] order: Order,
) {
    let result = submit_order(&order).await;
    assert_eq!(result, Ok(order.id));
}

#[tokio::test]
#[fixtura::inject]
async fn submit_order_fails_with_empty_status(#[fixtura(status = String::new())] order: Order) {
    assert!(submit_order(&order).await.is_err());
}

#[tokio::test]
#[fixtura::inject]
async fn activate_user_sets_active_true(#[fixtura(active = false)] user: User) {
    let mut user = user;
    activate_user(&mut user).await;
    assert!(user.active);
}

#[tokio::test]
#[fixtura::inject]
async fn cross_reference_preserved_across_await(
    user: User,
    #[fixtura(user_id = user.id)] order: Order,
) {
    let found = find_user(user.id.max(1)).await;
    assert!(found.is_some());
    assert_eq!(order.user_id, user.id);
}

#[tokio::test]
#[fixtura::inject]
#[should_panic(expected = "not implemented")]
async fn should_panic_preserved_in_async(_user: User) {
    unimplemented!()
}
