mod origin;
mod session_name;
mod version;
use std::net::{IpAddr, Ipv4Addr};

use nom::{character::complete::alpha1, combinator::peek, error::ParseError, IResult};
use origin::{parse_origin, AddrType, NetType, Origin};
use session_name::SessionName;
use version::{parse_version, Version};

#[derive(Debug)]
enum SessionDescriptionKeys {
    Version,
    Origin,
    SessionName,
    SessionInformation,
    URI,
    EmailAddress,
    PhoneNumber,
    ConnectionInformation,
    BandwidthInformation,
    EncryptionKey, // To be discarded
    Attribute,
}

#[derive(Debug)]
struct SessionDescription<'a> {
    version: Version,
    origin: Origin<'a>,
    session_name: SessionName<'a>,
}

impl<'a> SessionDescription<'a> {
    pub fn new(version: Version, origin: Origin<'a>, session_name: SessionName<'a>) -> Self {
        Self {
            version,
            origin,
            session_name,
        }
    }

    fn from_str(s: &'a str) -> Result<Self, ()> {
        let mut version: Version = { Version::new(0) };
        let mut origin: Origin = {
            Origin::new(
                "",
                "",
                0,
                NetType::IN,
                AddrType::IP4,
                IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            )
        };
        let mut session_name: SessionName = { SessionName::new("") };
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
                SessionDescriptionKeys::SessionName => {
                    let (rem, s) = session_name::parse_session_name::<()>(tail).unwrap();
                    session_name = s;
                    tail = rem;
                    if tail.is_empty() {
                        break;
                    }
                }
                _ => unimplemented!("key not implemented"),
            }
        }
        Ok(SessionDescription::new(version, origin, session_name))
    }
}

fn peek_key<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, SessionDescriptionKeys, E> {
    let (tail, p) = peek(alpha1)(input)?;
    let key = match p {
        "v" => SessionDescriptionKeys::Version,
        "o" => SessionDescriptionKeys::Origin,
        "s" => SessionDescriptionKeys::SessionName,
        _ => unimplemented!("key not implemented {}", p),
    };
    Ok((tail, key))
}

#[cfg(test)]
mod tests {

    use super::*;

    // FIXME: Tjere are issues with this test
    // #[test]
    // fn test_session_description() {
    //     let input = "v=0\r\no=jdoe 2890844526 2890842807 IN IP4 192.168.10.1\r\ns=SDP Seminar\r\n";
    //     let expected = SessionDescription::new(
    //         Version::new(0),
    //         Origin::new(
    //             "jdoe",
    //             "2890844526",
    //             2890842807,
    //             NetType::IN,
    //             AddrType::IP4,
    //             IpAddr::V4(Ipv4Addr::new(192, 168, 10, 1)),
    //         ),
    //         SessionName::new("SDP Seminar"),
    //     );
    //     let result = SessionDescription::from_str(input).unwrap();
    //     assert_eq!(result.version, expected.version);
    //     assert_eq!(result.origin, expected.origin);
    //     assert_eq!(result.session_name, expected.session_name);
    // }

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
