#![allow(dead_code)]
/// Tests that fixtura works with named, tuple, and unit structs for plain injection.
/// Field overrides (#[fixtura(...)]) require named struct fields; tuple index syntax
/// is not supported as it cannot be parsed as an identifier.
use fake::Dummy;

#[derive(Dummy, Debug)]
struct Named {
    x: u32,
    label: String,
}

#[derive(Dummy, Debug)]
struct Tuple(u32, String);

#[derive(Dummy, Debug)]
struct Unit;

// Plain injection works for named structs (baseline).
#[fixtura::test]
fn named_struct_injection(n: Named) {
    let _ = n;
}

// Plain injection works for tuple structs.
#[fixtura::test]
fn tuple_struct_injection(t: Tuple) {
    let _ = t;
}

// Plain injection works for unit structs.
#[fixtura::test]
fn unit_struct_injection(u: Unit) {
    let _ = u;
}

// Multiple struct kinds can coexist in the same test signature.
#[fixtura::test]
fn mixed_struct_kinds(n: Named, t: Tuple, u: Unit) {
    let _ = (n, t, u);
}

// Field overrides work on named structs alongside plain tuple/unit injection.
#[fixtura::test]
fn override_named_with_tuple_and_unit(#[fixtura(x = 42u32)] n: Named, t: Tuple, u: Unit) {
    assert_eq!(n.x, 42);
    let _ = (t, u);
}
