use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{alpha1, alphanumeric1, digit1, line_ending, multispace1, u64, u8},
        streaming::not_line_ending,
    },
    combinator::map,
    error::ParseError,
    sequence::{terminated, tuple},
    IResult, Parser,
};
use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

#[derive(Debug)]
pub struct Origin<'a> {
    username: &'a str,
    session_id: &'a str,
    session_version: u64,
    nettype: NetType,
    addrtype: AddrType,
    unicast_address: IpAddr,
}

impl<'a> Origin<'a> {
    pub fn new(
        username: &'a str,
        session_id: &'a str,
        session_version: u64,
        nettype: NetType,
        addrtype: AddrType,
        unicast_address: IpAddr,
    ) -> Self {
        Self {
            username,
            session_id,
            session_version,
            nettype,
            addrtype,
            unicast_address,
        }
    }
}

impl PartialEq for Origin<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username
            && self.session_id == other.session_id
            && self.session_version == other.session_version
            && self.nettype == other.nettype
            && self.addrtype == other.addrtype
            && self.unicast_address == other.unicast_address
    }
}

#[derive(Debug)]
pub enum NetType {
    IN,
}

impl PartialEq for NetType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NetType::IN, NetType::IN) => true,
        }
    }
}

#[derive(Debug)]
pub struct ParseNetTypeError;

impl FromStr for NetType {
    type Err = ParseNetTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IN" => Ok(NetType::IN),
            _ => Err(ParseNetTypeError),
        }
    }
}

#[derive(Debug)]
pub enum AddrType {
    IP4,
    IP6,
}

impl PartialEq for AddrType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AddrType::IP4, AddrType::IP4) => true,
            (AddrType::IP6, AddrType::IP6) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct ParseAddrTypeError;

impl FromStr for AddrType {
    type Err = ParseAddrTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IP4" => Ok(AddrType::IP4),
            "IP6" => Ok(AddrType::IP6),
            _ => Err(ParseAddrTypeError),
        }
    }
}

fn parse_username<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, &'i str, E> {
    terminated(alt((alphanumeric1, tag("-"))), multispace1).parse(input)
}

fn parse_session_id<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, &'i str, E> {
    terminated(digit1, multispace1).parse(input)
}

fn parse_session_version<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, u64, E> {
    terminated(u64, multispace1).parse(input)
}

fn parse_nettype<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, NetType, E> {
    terminated(
        map(alpha1, |s: &str| NetType::from_str(s).unwrap()),
        multispace1,
    )
    .parse(input)
}

fn parse_addrtype<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, AddrType, E> {
    terminated(
        map(alphanumeric1, |s: &str| AddrType::from_str(s).unwrap()),
        multispace1,
    )
    .parse(input)
}

fn parse_ip_address<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, IpAddr, E> {
    alt((
        map(
            tuple((
                terminated(u8, tag(".")),
                terminated(u8, tag(".")),
                terminated(u8, tag(".")),
                terminated(u8, line_ending),
            )),
            |(a, b, c, d)| IpAddr::V4(Ipv4Addr::new(a, b, c, d)),
        ),
        map(terminated(not_line_ending, line_ending), |s: &str| {
            s.parse::<IpAddr>().unwrap()
        }),
    ))
    .parse(input)
}

/// o=<username> <sess-id> <sess-version> <nettype> <addrtype> <unicast-address>
/// o=jdoe 2890844526 2890842807 IN IP4
/// see https://tools.ietf.org/html/rfc8866#section-5.2
pub fn parse_origin<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Origin, E> {
    let (tail, _) = tag("o=").parse(input)?;
    let (tail, username) = parse_username(tail)?;
    let (tail, session_id) = parse_session_id(tail)?;
    let (tail, session_version) = parse_session_version(tail)?;
    let (tail, nettype) = parse_nettype(tail)?;
    let (tail, addrtype) = parse_addrtype(tail)?;
    let (tail, unicast_address) = parse_ip_address(tail)?;

    Ok((
        tail,
        Origin {
            username: username,
            session_id: session_id,
            session_version,
            nettype,
            addrtype,
            unicast_address,
        },
    ))
}

#[cfg(test)]
mod tests {
    use std::net::Ipv6Addr;

    use super::*;

    #[test]
    fn test_parse_username() {
        let (tail, value) = parse_username::<()>("jdoe 123").unwrap();
        assert_eq!(tail, "123");
        assert_eq!(value, "jdoe");
    }

    #[test]
    fn test_parse_session_id() {
        let (tail, value) = parse_session_id::<()>("123 456").unwrap();
        assert_eq!(tail, "456");
        assert_eq!(value, "123");
    }

    #[test]
    fn test_parse_session_version() {
        let (tail, value) = parse_session_version::<()>("123 456").unwrap();
        assert_eq!(tail, "456");
        assert_eq!(value, 123);
    }

    #[test]
    fn test_parse_nettype() {
        let (tail, value) = parse_nettype::<()>("IN 123").unwrap();
        assert_eq!(tail, "123");
        assert_eq!(value, NetType::IN);
    }

    #[test]
    fn test_parse_addrtype() {
        let (tail, value) = parse_addrtype::<()>("IP4 123").unwrap();
        assert_eq!(tail, "123");
        assert_eq!(value, AddrType::IP4);
    }

    #[test]
    fn test_parse_origin() {
        let (tail, value) =
            parse_origin::<()>("o=jdoe 2890844526 2890842807 IN IP4 192.168.10.1\r\n").unwrap();
        assert_eq!(tail, "");
        assert_eq!(value.username, "jdoe");
        assert_eq!(value.session_id, "2890844526");
        assert_eq!(value.session_version, 2890842807);
        assert_eq!(value.nettype, NetType::IN);
        assert_eq!(value.addrtype, AddrType::IP4);
        assert_eq!(
            value.unicast_address,
            IpAddr::V4(Ipv4Addr::new(192, 168, 10, 1))
        );
    }

    #[test]
    fn test_parse_origin_with_ipv6() {
        let (tail, value) =
            parse_origin::<()>("o=jdoe 2890844526 2890842807 IN IP6 ::1\r\n").unwrap();
        assert_eq!(tail, "");
        assert_eq!(value.username, "jdoe");
        assert_eq!(value.session_id, "2890844526");
        assert_eq!(value.session_version, 2890842807);
        assert_eq!(value.nettype, NetType::IN);
        assert_eq!(value.addrtype, AddrType::IP6);
        assert_eq!(
            value.unicast_address,
            IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1))
        );
    }
}
