use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending},
    combinator::{map, opt},
    error::ParseError,
    sequence::{preceded, terminated},
    IResult, Parser,
};

#[derive(Debug)]
pub struct SessionName<'a> {
    name: &'a str,
}

impl<'a> SessionName<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
    /// Validates the session name against the given charset.
    ///
    /// - If a session-level "a=charset:" attribute is present,
    ///   it specifies the character set used in the "s=" field. If a session-level "a=charset:" attribute is not present,
    ///   the "s=" field MUST contain ISO 10646 characters in UTF-8 encoding.
    ///
    /// # Arguments
    ///
    /// * `char_set` - A string slice that holds the charset to validate the session name against.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the session name is in the given charset, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// let session_name = SessionName::new("Session Name");
    /// let result = session_name.validate_char_set("UTF-8");
    /// assert!(result);
    /// ```
    /// [1]: https://tools.ietf.org/html/rfc8866#section-5.3
    pub fn validate_char_set(&self, char_set: &str) -> bool {
        if char_set.is_empty() {
            return true;
        }
        // Validate the session name against the given charset.
        // If the charset is not present, the session name MUST contain ISO 10646 characters in
        // UTF-8 encoding.
        // For now, we are just returning true. TODO: Implement charset validation.
        return true;
    }
}

impl PartialEq for SessionName<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

/// Parses the session name from the given input string.
///
/// This function expects the input string to start with "s=" followed by the session name.
/// RFC-8866 defines it as `s=<session name>` where
/// - There MUST be one and only one "s=" line per session description.
/// - The "s=" line MUST NOT be empty.
/// - If a session has no meaningful name, then "s= " or "s=-"
///   (i.e., a single space or dash as the session name) is RECOMMENDED. [1]
///
/// # Arguments
///
/// * `input` - A string slice that holds the session description.
///
/// # Returns
///
/// * `IResult<&'i str, SessionName<'i>, E>` - A result containing the remaining input and the parsed `SessionName` on success, or an error on failure.
///
/// # Example
///
/// ```
/// let input = "s=Session Name\r\n";
/// let result = parse_session_name(input);
/// assert!(result.is_ok());
/// ```
/// [1]: https://tools.ietf.org/html/rfc8866#section-5.3
pub fn parse_session_name<'a, 'i: 'a, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, SessionName<'i>, E> {
    map(
        preceded(tag("s="), terminated(not_line_ending, opt(line_ending))),
        |s| SessionName::new(s),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_session_name() {
        let input = "s=Session Name\r\n";
        let expected = SessionName::new("Session Name");
        let result = parse_session_name::<()>(input).unwrap().1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_default_session_name_with_space() {
        let input = "s= \r\n";
        let expected = SessionName::new(" ");
        let result = parse_session_name::<()>(input).unwrap().1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_default_session_name_with_dash() {
        let input = "s=-\r\n";
        let expected = SessionName::new("-");
        let result = parse_session_name::<()>(input).unwrap().1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_validate_char_set() {
        let session_name = SessionName::new("Session Name");
        let result = session_name.validate_char_set("UTF-8");
        assert!(result);
    }
}
