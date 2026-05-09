use fake::Dummy;

#[derive(Dummy)]
struct User {
    id: u32,
    active: bool,
    name: String,
}

// Three #[fixtura] attrs on one arg: exercises the [2..] error loop.
#[fixtura::test]
fn test(
    #[fixtura(id = 1u32)]
    #[fixtura(active = false)]
    #[fixtura(name = "x".to_string())]
    user: User,
) {
    let _ = user;
}
