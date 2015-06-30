mod spdx;

use self::LicenseExpr::*;

#[derive(Debug, Clone, Copy)]
pub enum LicenseExpr<'a> {
    License(&'a str),
    Exception(&'a str),
    With, And, Or
}

#[derive(Debug, Clone, Copy)]
pub enum ParseError<'a> {
    UnknownLicenseId(&'a str),
    InvalidStructure(LicenseExpr<'a>)
}

pub fn validate_license_expr(license_expr: &str) -> Result<&str, ParseError> {
    license_expr.split_whitespace().map(|word| match word {
        "WITH" => Ok(With),
        "AND"  => Ok(And),
        "OR"   => Ok(Or),
        _ if spdx::LICENSES.contains(&word)   => Ok(License(word)),
        _ if spdx::EXCEPTIONS.contains(&word) => Ok(Exception(word)),
        _ => Err(ParseError::UnknownLicenseId(word))
    }).fold(Ok(Or), |prev, word| match (prev, word) {
        (_, Err(_)) => word,
        (Err(_), _) => prev,
        (Ok(License(_)), Ok(With))
            | (Ok(License(_)), Ok(And))
            | (Ok(License(_)), Ok(Or))
            | (Ok(Exception(_)), Ok(And))
            | (Ok(Exception(_)), Ok(Or))
            | (Ok(With), Ok(Exception(_)))
            | (Ok(And), Ok(License(_)))
            | (Ok(Or), Ok(License(_)))
            => word,
        _ => Err(ParseError::InvalidStructure(word.unwrap()))
    }).map(|expr| { println!("{:?}", expr); license_expr })
}

#[cfg(test)]
mod tests {

    #[test]
    fn single_license() {
        super::validate_license_expr("MIT").unwrap();
    }

    #[test]
    fn compound_license() {
        super::validate_license_expr("GPL-3.0+ WITH Classpath-exception-2.0 OR MIT AND AAL")
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fails_invalid_license() {
        super::validate_license_expr("asdfghjkl").unwrap();
        super::validate_license_expr("MIT AND qwerty").unwrap();
    }

    #[test]
    #[should_panic]
    fn fails_incorrect_structure() {
        super::validate_license_expr("WITH").unwrap();
        super::validate_license_expr("MIT OR WITH").unwrap();
        super::validate_license_expr("MIT AND Classpath-exception-2.0").unwrap();
        super::validate_license_expr("Classpath-exception-2.0").unwrap();
    }

}
