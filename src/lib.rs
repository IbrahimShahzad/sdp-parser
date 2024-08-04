mod session_desription;
// mod utils;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// An SDP description consists of a number of lines of text of the form:
//    <type>=<value>

enum SDPLevel {
    SessionLevel,
    TimeDescriptionLevel,
    MediaDescriptionLevel,
}

impl SDPLevel {
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "s" => Ok(SDPLevel::SessionLevel),
            "t" => Ok(SDPLevel::TimeDescriptionLevel),
            "m" => Ok(SDPLevel::MediaDescriptionLevel),
            _ => Err(()),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            SDPLevel::SessionLevel => "s",
            SDPLevel::TimeDescriptionLevel => "t",
            SDPLevel::MediaDescriptionLevel => "m",
        }
    }
}

fn get_session_description_chunk() {
    // get till the next time description
    unimplemented!("get_session_description_chunk");
}

fn get_time_description_chunk() {
    // get till the next media description
    unimplemented!("get_time_description_chunk");
}

fn get_media_description_chunk() {
    // get till the next media description
    unimplemented!("get_media_description_chunk");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
