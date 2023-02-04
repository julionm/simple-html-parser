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

// TODO add better error treatment
// ? (the_rest_of_the_string, the nodes_processed)
pub fn process(html: String) -> Result<(String, Node), String> {
    let html = String::from(html.trim());
    let mut lines: Vec<String> = html.lines().map(|line| String::from(line)).collect();

    // ! REMOVE THIS AFTER BETTER TESTS
    println!("line0: {}", lines[0]);

    if lines[0].trim().ends_with("/>") {
        let initial_node = html_to_node(lines.remove(0).as_str());

        return Ok((lines.join("\n"), initial_node));
    }

    let mut initial_node = html_to_node(lines.remove(0).as_str());
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
                // ! REMOVE THIS AFTER BETTER TESTS
                println!("self-closing: {}", next_line);
                children.push(html_to_node(next_line));
                continue;
            }

            let (rest, new_node) = process(lines.join("\n").clone())?;

            lines = rest.lines().map(|line| String::from(line)).collect();
            children.push(new_node);
        } else {
            // TODO rethink this later
            println!("felt on else");
            break;
        }
    }

    initial_node.children = children;

    Ok((lines.join("\n"), initial_node))
}

fn match_literal<'a>(expected: &'a str) -> impl Fn(&'a str) -> Result<&str, String> {
    move |input: &str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok(&input[expected.len()..]),
        _ => Err(format!("Couldn't match the result for {}", expected))
    }
}

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

fn match_attributes(line: &str) -> Result<(&str, HashMap<String, String>), String> {

    // TODO write the method to match attributes

    /*
    
        to match an attribute

    
    */

    Ok((line, HashMap::new()))
}

fn html_to_node_with_combinators(line: &str) -> Result<Node, String> {

    let match_opening_chevron = match_literal("<");
    let match_closing_chevron = match_literal("/>");

    let mut node = Node::new();

    match_opening_chevron(line)
    .and_then(
        |rest| match_identifier(rest)
    )
    .and_then(
        |(rest, name)| {
            node.name = name;
            match_attributes(rest)
        } 
    )
    .and_then(
        |(rest, attributes)| {
            node.attributes = attributes;
            match_closing_chevron(rest)
        }
    )?;

    Ok(Node::new())
}

// ! This method will be replaces by the new one using combinators
// ! consider a valid html line
fn html_to_node(line: &str) -> Node {

    // ? MAYBE... it'll be better to use some parse combinators here

    // ? removing opening <
    let line = &line[1..];
    let mut chars = line.chars();

    let mut identifier = String::new();

    let mut next_char = 'a';

    while let Some(ch) = chars.next() {
        if ch.is_alphanumeric() {
            identifier.push(ch);
        } else {
            next_char = ch;
            break;
        }
    }

    if next_char == '/' || next_char == '>' {
        let mut node = Node::new();
        node.name = identifier;

        return node;
    }

    // ? try to think about attributes later?

    let mut node = Node::new();
    node.name = identifier;

    node
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
    fn test_match_identifier() {
        let html_line = "div qlqrcoisa=\"outra-coisa\"";

        assert_eq!(
            Ok((" qlqrcoisa=\"outra-coisa\"", "div".into())),
            match_identifier(html_line)
        );
    }

}
