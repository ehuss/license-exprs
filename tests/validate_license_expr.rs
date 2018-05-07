extern crate license_exprs;

use license_exprs::validate_license_expr;

#[test]
fn single_license() {
    assert!(validate_license_expr("MIT").is_ok());
}

#[test]
fn compound_license() {
    let license = "GPL-3.0+ WITH Classpath-exception-2.0 OR MIT AND AAL";
    assert!(validate_license_expr(license).is_ok());
}

#[test]
fn fails_invalid_license() {
    assert!(validate_license_expr("asdfghjkl").is_err());
    assert!(validate_license_expr("MIT AND qwerty").is_err())
}

#[test]
fn fails_incorrect_structure() {
    assert!(validate_license_expr("WITH").is_err());
    assert!(validate_license_expr("MIT OR WITH").is_err());
    assert!(validate_license_expr("MIT AND Classpath-exception-2.0").is_err());
    assert!(validate_license_expr("Classpath-exception-2.0").is_err());
}
