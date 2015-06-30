use std::error::Error;
use std::fmt;

mod spdx;

use self::LicenseExpr::*;

#[derive(Debug, Clone, Copy)]
pub enum LicenseExpr<'a> {
    License(&'a str),
    Exception(&'a str),
    And, Or, With, 
}

impl<'a> fmt::Display for LicenseExpr<'a> {
    fn fmt(&self, format: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            With => format.write_str("WITH"),
            And  => format.write_str("AND"),
            Or   => format.write_str("OR"),
            License(info) | Exception(info) => format.write_str(info),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParseError<'a> {
    UnknownLicenseId(&'a str),
    InvalidStructure(LicenseExpr<'a>)
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, format: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ParseError::UnknownLicenseId(info)
                => format.write_fmt(format_args!("{}: {}", self.description(), info)),
            ParseError::InvalidStructure(info)
                => format.write_fmt(format_args!("{}: {}", self.description(), info)),
        }
    }
}

impl<'a> Error for ParseError<'a> {
    fn description(&self) -> &str {
        match *self {
            ParseError::UnknownLicenseId(_) => "unknown license or other term",
            ParseError::InvalidStructure(_) => "invalid license expression",
        }
    }
}

pub fn validate_license_expr(license_expr: &str) -> Result<&str, ParseError> {
    license_expr.split_whitespace().map(|word| match word {
        "AND"  => Ok(And),
        "OR"   => Ok(Or),
        "WITH" => Ok(With),
        _ if spdx::LICENSES.contains(&word)   => Ok(License(word)),
        _ if spdx::EXCEPTIONS.contains(&word) => Ok(Exception(word)),
        _ => Err(ParseError::UnknownLicenseId(word))
    }).fold(Ok(Or), |prev, word| match (prev, word) {
        (err @ Err(_), _) | (_, err @ Err(_)) => err,
        (Ok(License(_)), Ok(With))
            | (Ok(License(_)), Ok(And))
            | (Ok(License(_)), Ok(Or))
            | (Ok(Exception(_)), Ok(And))
            | (Ok(Exception(_)), Ok(Or))
            | (Ok(And), Ok(License(_)))
            | (Ok(Or), Ok(License(_)))
            | (Ok(With), Ok(Exception(_)))
            => word,
        _ => Err(ParseError::InvalidStructure(word.unwrap()))
    }).map(|_| license_expr)
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
