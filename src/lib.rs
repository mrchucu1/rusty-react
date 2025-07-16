use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Node {
    Element(Element),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub tag_name: String,
    pub props: HashMap<String, String>,
    pub children: Vec<Node>,
}

pub fn create_element(tag_name: String, props: HashMap<String, String>, children: Vec<Node>) -> Node {
    Node::Element(Element {
        tag_name,
        props,
        children,
    })
}

pub fn create_text_node(text: String) -> Node {
    Node::Text(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // We can keep our old test, but it will need updating. Let's write a new one.
    #[test]
    fn it_builds_a_virtual_dom_tree() {
        // Create the <p>This is our rusty-react app.</p> element
        let paragraph = create_element(
            "p".to_string(),
            HashMap::new(), // No props
            vec![create_text_node("This is our rusty-react app.".to_string())],
        );

        // Create the <h1>Welcome</h1> element
        let heading = create_element(
            "h1".to_string(),
            HashMap::new(), // No props
            vec![create_text_node("Welcome".to_string())],
        );

        // Create the root <div> with its props and children
        let app_div = create_element(
            "div".to_string(),
            {
                let mut props = HashMap::new();
                props.insert("id".to_string(), "app".to_string());
                props.insert("class".to_string(), "container".to_string());
                props
            },
            vec![heading, paragraph],
        );

        // Let's create our expected structure for comparison
        let expected_tree = Node::Element(Element {
            tag_name: "div".to_string(),
            props: {
                let mut props = HashMap::new();
                props.insert("id".to_string(), "app".to_string());
                props.insert("class".to_string(), "container".to_string());
                props
            },
            children: vec![
                Node::Element(Element {
                    tag_name: "h1".to_string(),
                    props: HashMap::new(),
                    children: vec![Node::Text("Welcome".to_string())]
                }),
                Node::Element(Element {
                    tag_name: "p".to_string(),
                    props: HashMap::new(),
                    children: vec![Node::Text("This is our rusty-react app.".to_string())]
                })
            ]
        });

        assert_eq!(app_div, expected_tree);
    }
}
