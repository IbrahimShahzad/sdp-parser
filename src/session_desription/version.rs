use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u8},
    combinator::{map, opt},
    error::{FromExternalError, ParseError},
    sequence::{preceded, terminated},
    IResult, Parser,
};

#[derive(Debug)]
pub struct Version {
    version: u8,
}

impl Version {
    pub fn new(version: u8) -> Self {
        Self { version }
    }
}

#[derive(Debug)]
pub struct ParseVersionError;

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_version::<()>(s) {
            Ok((_, version)) => Ok(version),
            Err(_) => Err(ParseVersionError),
        }
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

pub fn parse_version<
    'i,
    E: ParseError<&'i str> + FromExternalError<&'i str, std::num::ParseIntError>,
>(
    input: &'i str,
) -> IResult<&'i str, Version, E> {
    map(preceded(tag("v="), terminated(u8, opt(line_ending))), |v| {
        Version::new(v)
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        let input = "v=0\r\n";
        let expected = Version::new(0);
        let (tail, result) = parse_version::<()>(input).unwrap();
        println!("{:?}", result);
        assert_eq!(result.version, expected.version);
        assert_eq!(tail, "");
    }

    #[test]
    fn test_parse_version_without_line_ending() {
        let input = "v=0";
        let expected = Version::new(0);
        let result = parse_version::<()>(input).unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1.version, expected.version);
    }

    #[test]
    fn test_version_from_str_ok() {
        let input = "v=0";
        let expected = Version::new(0);
        let result = Version::from_str(input);
        assert_eq!(result.unwrap().version, expected.version);
    }
}
