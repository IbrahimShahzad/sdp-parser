mod origin;
mod version;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, not_line_ending, u8},
    combinator::peek,
    error::{FromExternalError, ParseError},
    sequence::{preceded, terminated},
    IResult, Parser,
};

#[derive(Debug)]
enum SessionDescriptionKeys {
    Version,
    Origin,
}

struct SessionDescription<'a> {
    version: u8,
    origin: &'a str,
}

impl<'a> SessionDescription<'a> {
    fn new(version: u8, origin: &'a str) -> Self {
        Self { version, origin }
    }

    fn from_str(s: &'a str) -> Result<Self, ()> {
        let mut version: u8 = 0;
        let mut origin: &str = "";
        let tail: &str = s;
        while !tail.is_empty() {
            let (mut tail, key) = peek_key::<()>(tail).unwrap();
            match key {
                SessionDescriptionKeys::Version => {
                    let (rem, v) = parse_version::<()>(tail).unwrap();
                    version = v;
                    tail = rem;
                    if tail.is_empty() {
                        break;
                    }
                }
                SessionDescriptionKeys::Origin => {
                    let (rem, o) = parse_origin::<()>(tail).unwrap();
                    origin = o;
                    tail = rem;
                    if tail.is_empty() {
                        break;
                    }
                }
            }
        }
        Ok(SessionDescription::new(version, origin))
    }
}

fn parse_origin<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, &'i str, E> {
    preceded(tag("o="), terminated(not_line_ending, line_ending)).parse(input)
}

fn parse_version<
    'i,
    E: ParseError<&'i str> + FromExternalError<&'i str, std::num::ParseIntError>,
>(
    input: &'i str,
) -> IResult<&'i str, u8, E> {
    preceded(tag("v="), terminated(u8, line_ending)).parse(input)
}

fn peek_key<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, SessionDescriptionKeys, E> {
    let (tail, p) = peek(alpha1)(input)?;
    let key = match p {
        "v" => SessionDescriptionKeys::Version,
        "o" => SessionDescriptionKeys::Origin,
        _ => unimplemented!("key not implemented {}", p),
    };
    Ok((tail, key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        let (tail, value) = parse_version::<()>("v=0\r\n").unwrap();
        assert_eq!(tail, "");
        assert_eq!(value, 0 as u8);
    }

    #[test]
    fn test_parse_origin() {
        let (tail, value) = parse_origin::<()>("o=jdoe 2890844526 2890842807 IN IP4\r\n").unwrap();
        assert_eq!(tail, "");
        assert_eq!(value, "jdoe 2890844526 2890842807 IN IP4");
    }

    #[test]
    fn test_peek_key() {
        let (tail, key) = peek_key::<()>("v=0\r\n").unwrap();
        assert_eq!(tail, "v=0\r\n");
        match key {
            SessionDescriptionKeys::Version => {}
            _ => panic!("unexpected key"),
        }
    }
}
