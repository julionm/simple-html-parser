use std::collections::HashMap;

pub fn or<'a, T, A>(parser1: T, parser2: T) -> impl FnOnce(&'a str) -> Result<A, String>
where 
    T: Fn(&'a str) -> Result<A, String>
{
    move |input: &str| {
        match parser1(input) {
            Ok(v) => Ok(v),
            Err(_) => parser2(input)
        }
    }  
}

// TODO rewrite this method
pub fn match_literal<'a>(expected: &'a str) -> impl Fn(&'a str) -> Result<&str, String> {
    move |input: &str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok(&input[expected.len()..]),
        _ => Err(format!("Couldn't match the result for {}", expected))
    }
}

// TODO rewrite this method
pub fn match_identifier(input: &str) -> Result<(&str, String), String> {

    let input = input.trim();

    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if !next.is_whitespace() => matched.push(next),
        _ => return Err("Invalid identifier".into())
    }

    while let Some(next) = chars.next() {
        if next.is_alphanumeric() || next == '-' {
            matched.push(next)
        } else {
            break;
        }
    }

    let next_index = matched.len();

    Ok((&input[next_index..], matched))
}

pub fn match_string(line: &str) -> Result<(&str, String), String> {
    let line = line.trim();
    let match_dbquotes = match_literal("\"");

    match_dbquotes(line).and_then(
        |rest| {
            let idx = rest.find("\"");

            let idx = match idx {
                Some(idx) => idx,
                None => return Err("Not closing attribute value".into())
            };
            let new_str = &rest[..idx];

            Ok((&rest[idx+1..], new_str.into()))
        }
    )
}

pub fn match_attributes(line: &str) -> Result<(&str, HashMap<String, String>), String> {

    let mut line = line.trim();
    let mut attributes: HashMap<String, String> = HashMap::new();

    let match_eq = match_literal("=");

    loop {
        if line == "/>" || line == ">" {
            break;
        }

        let mut key = String::new();
        let mut value = String::new();

        line = match 
            match_identifier(line)
            .and_then(
                |(rest, attr_name)| {
                    key = attr_name;

                    match_eq(rest)
                }
            ).and_then(
                |rest| match_string(rest)
            ).and_then(
                |(rest, attr_value)| {
                    value = attr_value;

                    Ok(rest)
                }
            )
        {
            Ok(rest) => rest.trim_start().into(),
            Err(err) => return Err(err)
        };

        attributes.insert(key, value);
    }

    Ok((line, attributes))
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use super::{
        match_attributes,
        match_identifier,
        match_literal,
        match_string,
        or,
    };

    #[test]
    fn test_match_literal() {
        let parse_opening_chevron = match_literal("<");
        let html_line = "<div/>";

        assert_eq!(Ok("div/>"), parse_opening_chevron(html_line));
    }

    #[test]
    fn test_match_string() {
        let test_str = "\"test test test \" anotherthing";

        assert_eq!(
            Ok((" anotherthing", String::from("test test test "))),
            match_string(test_str)
        );
    }

    #[test]
    fn test_match_identifier() {
        let html_line = "div anything=\"another-thing\"";

        assert_eq!(
            Ok((" anything=\"another-thing\"", "div".into())),
            match_identifier(html_line)
        );
    }

    #[test]
    fn test_match_attributes() {
        let attrs_line = " class=\"class-test\" width=\"180\" height=\"180\" />";
        let expected_map = HashMap::from([
            (String::from("class"), String::from("class-test")),
            (String::from("width"), String::from("180")),
            (String::from("height"), String::from("180"))
        ]);

        assert_eq!(Ok(("/>", expected_map)), match_attributes(attrs_line));
    }

    #[test]
    fn test_match_one_or_other() {
        let match_end_of_line = or(
            match_literal("/>"),
            match_literal(">")
        );

        let line = ">";

        assert_eq!(Ok(""), match_end_of_line(line))
    }

}