use fake::Dummy;

#[derive(Dummy)]
struct User {
    id: u32,
}

// Simulates a second-framework arg: pool is a pass-through, user is fixtura-owned.
// The generated signature keeps `pool: u32` so the caller (framework) provides it.
#[fixtura::inject]
async fn with_passthrough(pool: u32, #[fixtura] user: User) {
    let _ = (pool, user);
}

fn main() {}
