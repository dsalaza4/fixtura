use fake::Dummy;

#[derive(Dummy)]
struct User {
    id: u32,
    active: bool,
}

// Duplicate #[fixtura] in inject mode: only one is allowed per argument.
#[tokio::test]
#[fixtura::inject]
async fn test(
    #[fixtura]
    #[fixtura(id = 1u32)]
    user: User,
) {
    let _ = user;
}
