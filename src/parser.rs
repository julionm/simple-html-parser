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

// TODO change this to a Result
// ? (the_rest_of_the_string, the nodes_processed)
pub fn process(html: String) -> (String, Node) {
    let html = String::from(html.trim());
    // ? maybe convert to a Vec<String>
    let mut lines: Vec<String> = html.lines().map(|line| String::from(line)).collect();
    println!("line0: {}", lines[0]);

    if lines[0].ends_with("/>") {
        // self-closing tag

        let initial_node = html_to_node(lines.remove(0).as_str());

        return (lines.join("\n"), initial_node);
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

            let (rest, new_node) = process(lines.join("\n").clone());

            lines = rest.lines().map(|line| String::from(line)).collect();
            children.push(new_node);
        } else {
            // TODO rethink this later
            break;
        }
    }

    initial_node.children = children;

    (lines.join("\n"), initial_node)
}

// ! consider a valid html line
fn html_to_node2(line: &str) -> Node {
    println!("html_to_node: {}", &line);
    // TODO accept - and numbers for identifier names

    // ? I'll use this example to illustrate how this function works
    // ? line = " <a link="https://google.com"> "

    // this is a line of an opening html tag
    
    // ? "<a link="https://google.com">"
    let mut line = line.trim();
    
    // ? "a link="https://google.com">"
    let chars = &line[0..].chars();
    let mut identifier = String::new();

    // processing identifier
    // ? link="https://google.com">
    while let Some(ch) = chars.next() {
        if ch.is_whitespace() {
            break;
        }

        identifier.push(ch);
    }

    // ? identifier = "a"

    let rest: String = chars.collect();
    let rest = rest.trim();

    let exists_whitespace = line.find(' ');

    // if exists whitespace, then it is a tag with attributes
    return match exists_whitespace {
        Some(index) => {
            let mut node = Node {
                name: String::from(&line[0..index]),
                attributes: HashMap::new(),
                children: Vec::new(),
            };

            let mut attributes: HashMap<String, String> = HashMap::new();

            // removing the last >
            // and only attributes are left
            line = &line[index..line.len() - 1];

            for attr in line.split(" ") {
                // here it'll be only attributes
                let attr_splited: Vec<&str> = attr.split("=").collect();

                attributes.insert(String::from(attr_splited[0]), String::from(attr_splited[1]));
            }

            node.attributes = attributes;

            node
        },
        None => 
            Node {
                name: String::from(&line[1..line.len()-1]), // <--- ERROR
                attributes: HashMap::new(),
                children: Vec::new(),
            }
            
    }
}


// ! consider a valid html line
fn html_to_node(line: &str) -> Node {

    // ? MAYBE... it'll be better to use some parse combinators here

    // ? removing opening <
    let line = &line[1..];
    let chars = line.chars();

    let identifier = String::new();

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

    let rest: String = chars.collect();
    let rest = rest.trim();
    
    // ? try to think about attributes later?

    let node = Node::new();
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
