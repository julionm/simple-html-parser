use std::collections::HashMap;
use super::utils::{
    match_literal,
    match_identifier,
    match_attributes,
    or
};

#[derive(Debug, PartialEq)]
pub struct Node {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Node>,
    content: String
}

impl Node {
    fn new() -> Node {
        Node {
            name: String::new(),
            attributes: HashMap::new(),
            children: Vec::new(),
            content: String::new()
        }
    }
}

// TODO add some ignore tags
pub fn process(html: String) -> Result<(String, Node), String> {
    let html = String::from(html.trim());
    let mut lines: Vec<String> = html.lines().map(|line| String::from(line)).collect();

    if lines[0].trim().ends_with("/>") {
        let initial_node = html_to_node(lines.remove(0).as_str())?;

        return Ok((lines.join("\n"), initial_node));
    }

    let mut initial_node = html_to_node(lines.remove(0).as_str())?;
    let mut children: Vec<Node> = Vec::new();
    let mut content = String::new();

    loop {
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
            content.push_str(next_line);
            content.push_str("\n");
            continue;
        }
    }

    initial_node.children = children;
    initial_node.content = content;

    Ok((lines.join("\n"), initial_node))
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
                    children: Vec::new(),
                    content: String::new()
                }
            ],
            content: String::new()
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
                    children: Vec::new(),
                    content: String::new(),
                }
            ],
            content: String::new()
        };

        assert_eq!(
            Ok(("".into(), node)),
            process(html.into())
        );
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
            children: Vec::new(),
            content: String::new()
        };

        assert_eq!(
            Ok(node),
            html_to_node(html_line)
        )

    }
}
