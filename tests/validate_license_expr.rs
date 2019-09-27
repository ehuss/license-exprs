extern crate license_exprs;

use license_exprs::validate_license_expr;

macro_rules! assert_licenses_ok {
    ($($license:expr),+ $(,)+) => {{
        $(assert!(validate_license_expr($license).is_ok());)+
    }}
}

macro_rules! assert_licenses_err {
    ($($license:expr),+ $(,)+) => {{
        $(assert!(validate_license_expr($license).is_err());)+
    }}
}

#[test]
fn single_license() {
    assert_licenses_ok! {
        "MIT",
        "MIT ",
        " MIT",
        " MIT ",
        "    MIT    ",
        "AGPL-1.0+",
        "BlueOak-1.0.0",
        "Parity-6.0.0",
    }
}

#[test]
fn compound_license() {
    assert_licenses_ok! {
        "GPL-3.0+ WITH Classpath-exception-2.0 OR MIT AND AAL",
        "MIT AND Apache-2.0",
        "MIT OR Apache-2.0",
        "GPL-3.0+ WITH Classpath-exception-2.0",
        "MIT   AND    Apache-2.0",
    }
}

#[test]
fn fails_invalid_license() {
    assert_licenses_err! {
        "asdfghjkl",
        "MIT AND qwerty",
    }
}

#[test]
fn fails_incorrect_structure() {
    assert_licenses_err! {
        "()",
        "(MIT",
        "MIT)",
        "MIT Apache-2.0",
        "MIT and Apache-2.0",
        "MIT or Apache-2.0",
        "GPL-3.0+ with Classpath-exception-2.0",
        "MIT (MIT AND MIT)",
        "(MIT AND MIT) MIT",
        "and Apache-2.0",
        "(MIT and Apache-2.0 and  )",
        "MIT xor Apache-2.0",
        "WITH",
        "MIT OR WITH",
        // "MIT WITH", // TODO: Incorrectly marked as valid
        // "MIT AND", // TODO: Incorrectly marked as valid
        "MIT AND Classpath-exception-2.0",
        "Classpath-exception-2.0 WITH MIT",
        "Classpath-exception-2.0",
        "AGPL-1.0 +",
    }
}
