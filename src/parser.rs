use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Node>
}

impl Node {
    fn new() -> Node {
        Node {
            name: String::new(),
            attributes: HashMap::new(),
            children: Vec::new() 
        }
    }
}

pub fn process(html: String) -> Result<(String, Node), String> {
    let html = String::from(html.trim());
    let mut lines: Vec<String> = html.lines().map(|line| String::from(line)).collect();

    if lines[0].trim().ends_with("/>") {
        let initial_node = html_to_node(lines.remove(0).as_str())?;

        return Ok((lines.join("\n"), initial_node));
    }

    let mut initial_node = html_to_node(lines.remove(0).as_str())?;
    let mut children: Vec<Node> = Vec::new();

    loop {
        // starts always with a opening tag
        let next_line = lines[0].clone();
        let next_line = next_line.trim();

        if next_line.starts_with("</") {
            lines.remove(0);
            break;
        } else if next_line.starts_with("<") {
            // ? opening tag or self-closing tag?

            if next_line.ends_with("/>") {
                lines.remove(0);

                children.push(html_to_node(next_line)?);
                continue;
            }

            let (rest, new_node) = process(lines.join("\n").clone())?;

            lines = rest.lines().map(|line| String::from(line)).collect();
            children.push(new_node);
        } else {
            // ? this place is where the non-tags will fall
            // ? I don't know what should I expect here except for strings
            break;
        }
    }

    initial_node.children = children;

    Ok((lines.join("\n"), initial_node))
}

// TODO write this method for fun
fn or<'a, T, A>(parser1: T, parser2: T) -> impl FnOnce(&'a str) -> Result<A, String>
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
fn match_literal<'a>(expected: &'a str) -> impl Fn(&'a str) -> Result<&str, String> {
    move |input: &str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok(&input[expected.len()..]),
        _ => Err(format!("Couldn't match the result for {}", expected))
    }
}

// TODO rewrite this method
fn match_identifier(input: &str) -> Result<(&str, String), String> {

    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
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

fn match_string(line: &str) -> Result<(&str, String), String> {
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

fn match_attributes(line: &str) -> Result<(&str, HashMap<String, String>), String> {

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

fn html_to_node(line: &str) -> Result<Node, String> {

    let match_opening = match_literal("<");
    let match_end_of_line = 
        or(
            match_literal("/>"),
            match_literal(">")
        );
    let mut node = Node::new();

    match_opening(line)
    .and_then(
        |rest| match_identifier(rest)
    )
    .and_then(
        |(rest, name)| {
            node.name = name;
            match_attributes(rest) // * <-- to be finished
        } 
    )
    .and_then(
        |(rest, attributes)| {
            node.attributes = attributes;
            match_end_of_line(rest)
        }
    )?;

    Ok(node)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_processing_simple_html() {
        let html = "<html>
                            <body>
                            </body>
                        </html>";

        let node = Node {
            name: "html".into(),
            attributes: HashMap::new(),
            children: vec![
                Node {
                    name: "body".into(),
                    attributes: HashMap::new(),
                    children: Vec::new()
                }
            ]
        };

        assert_eq!(Ok(("".into(), node)), process(html.into()));
    }

    #[test]
    fn process_html_with_selfclosing() {
        let html = "<html>
        <body />
        </html>";

        let node = Node {
            attributes: HashMap::new(),
            name: "html".into(),
            children: vec![
                Node {
                    name: "body".into(),
                    attributes: HashMap::new(),
                    children: Vec::new()
                }
            ]
        };

        assert_eq!(
            Ok(("".into(), node)),
            process(html.into())
        );
    }

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
    fn test_process_html_line() {
        let html_line = "<div class=\"container flex\" width=\"180\" height=\"100\" />";
        let attrs = HashMap::from([
            (String::from("class"), String::from("container flex")),
            (String::from("width"), String::from("180")),
            (String::from("height"), String::from("100"))
        ]);
        let node = Node {
            name: "div".into(),
            attributes: attrs,
            children: Vec::new()
        };

        assert_eq!(
            Ok(node),
            html_to_node(html_line)
        )

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
