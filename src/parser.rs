use std::collections::HashMap;

#[derive(Debug)]
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

    println!("line0: {}", lines[0]);
    // ! there's still problems happening with self-closing tags

    if lines[0].ends_with("/>") {
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
                children.push(html_to_node(next_line));
                break;
            }

            let (rest, new_node) = process(lines.join("\n").clone())?;

            lines = rest.lines().map(|line| String::from(line)).collect();
            children.push(new_node);
        } else {
            // TODO rethink this later
            break;
        }
    }

    initial_node.children = children;

    Ok((lines.join("\n"), initial_node))
}

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

    #[test]
    fn test_line_processing() {
        // create test for process_html_line()
    }

    #[test]
    fn test_processing_simple_html() {
        // TODO create test for simple html
    }

}
