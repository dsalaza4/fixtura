use fake::Dummy;

#[derive(Dummy)]
struct User {
    name: String,
    age: u32,
}

#[derive(Dummy)]
struct Order {
    id: u32,
    total: f64,
}

#[fixtura::test]
fn injects_single_arg(user: User) {
    let _ = user.name;
}

#[fixtura::test]
fn injects_multiple_args(user: User, order: Order) {
    let _ = (user.age, order.total);
}

#[fixtura::test]
#[should_panic]
fn preserves_other_attributes(user: User) {
    panic!("id was {}", user.age);
}
