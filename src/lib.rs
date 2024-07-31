mod session_desription;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// An SDP description consists of a number of lines of text of the form:
//    <type>=<value>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
