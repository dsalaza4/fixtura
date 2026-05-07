use fake::Dummy;

#[derive(Dummy)]
struct User {
    id: u32,
}

// Pass-through arg stays in the generated signature; fixtura-owned arg becomes a let binding.
#[fixtura::inject]
async fn with_passthrough(pool: u32, #[fixtura] user: User) {
    let _ = (pool, user);
}

// Reference types are valid for pass-through args (framework may inject &PgPool etc.).
#[fixtura::inject]
async fn with_ref_passthrough(pool: &u32, #[fixtura] user: User) {
    let _ = (pool, user);
}

// A pass-through ident can be referenced inside #[fixtura(...)] overrides — it is a
// function parameter, so it is in scope when fixtura bindings are generated.
#[fixtura::inject]
async fn with_passthrough_in_override(caller_id: u32, #[fixtura(id = caller_id)] user: User) {
    let _ = (caller_id, user);
}

fn main() {}
