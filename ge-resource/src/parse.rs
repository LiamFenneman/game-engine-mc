use nom::{
    bytes::complete::{take, take_till1},
    character::is_digit,
    IResult,
};

/// Parse a file name into a texture name and index.
/// 
/// # Errors
/// Errors if the file name cannot be parsed.
pub fn parse_file_name(file_name: &str) -> IResult<&str, (&str, u32)> {
    // split the input into the name+index and file extrension
    let (file_name, name) = take_till1(|c| return c == '.')(file_name)?;

    // split the name+index into name and index
    let (index, name) = take_till1(|c| return is_digit(c as u8))(name)?;

    // parse the index as u32
    let index = index.parse().expect("should be a number");

    // remove the last character (which should be an underscore)
    let (_, name) = take(name.len() - 1)(name)?;

    return Ok((file_name, (name, index)));
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let input = "grass_0.png";
        let (remaining, (texture_name, index)) = super::parse_file_name(input).unwrap();
        assert_eq!(remaining, ".png");
        assert_eq!(texture_name, "grass");
        assert_eq!(index, 0);
    }

    #[test]
    fn advanced() {
        let input = "something_with_multiple_underscores_10.png";
        let (remaining, (texture_name, index)) = super::parse_file_name(input).unwrap();
        assert_eq!(remaining, ".png");
        assert_eq!(texture_name, "something_with_multiple_underscores");
        assert_eq!(index, 10);
    }
}
