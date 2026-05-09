#![allow(dead_code)]
/// Tests that #[fixtura(field = value)] pins fields to exact values
/// while the rest of the struct is still faked. Includes nested field paths.
use fake::Dummy;

#[derive(Dummy, Debug)]
struct User {
    id: u32,
    name: String,
    active: bool,
}

#[derive(Dummy, Debug)]
struct Address {
    city: String,
    zip: String,
}

#[derive(Dummy, Debug)]
struct Profile {
    id: u32,
    address: Address,
}

// A single field is pinned to the exact literal value.
#[fixtura::test]
fn single_field_pinned_false(#[fixtura(active = false)] user: User) {
    assert!(!user.active);
}

#[fixtura::test]
fn single_field_pinned_true(#[fixtura(active = true)] user: User) {
    assert!(user.active);
}

// Multiple fields can be pinned in one attribute.
#[fixtura::test]
fn multiple_fields_pinned(#[fixtura(id = 7u32, active = false)] user: User) {
    assert_eq!(user.id, 7);
    assert!(!user.active);
}

// Non-overridden fields are still faked and accessible.
#[fixtura::test]
fn non_overridden_fields_accessible(#[fixtura(active = false)] user: User) {
    let _ = (user.id, &user.name);
}

// Override value can be any expression, not just a literal.
#[fixtura::test]
fn override_with_expression(#[fixtura(id = 2u32 + 3u32)] user: User) {
    assert_eq!(user.id, 5);
}

// Override value can be a method call producing an owned value.
#[fixtura::test]
fn override_with_method_call(#[fixtura(name = "alice".to_string())] user: User) {
    assert_eq!(user.name, "alice");
}

// Overriding a field in the middle arg; flanking args are plain.
#[fixtura::test]
fn override_in_middle_arg(first: User, #[fixtura(active = false)] mid: User, last: User) {
    assert!(!mid.active);
    let _ = (first, last);
}

// Nested field path: v.address.city = value.
#[fixtura::test]
fn nested_field_path(#[fixtura(address.city = "Portland".to_string())] profile: Profile) {
    assert_eq!(profile.address.city, "Portland");
}

// Nested path pins only the targeted sub-field; sibling is still faked.
#[fixtura::test]
fn nested_path_sibling_still_faked(
    #[fixtura(address.city = "Portland".to_string())] profile: Profile,
) {
    assert_eq!(profile.address.city, "Portland");
    let _ = &profile.address.zip; // faked, accessible
}
