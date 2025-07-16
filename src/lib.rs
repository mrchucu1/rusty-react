use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// Aliasing the `web_sys` types to avoid naming collisions with our VDOM types.
use web_sys::{Document, Element as DomElement, Node as DomNode, Window};

// Our Virtual DOM Element structure. This is our blueprint.
#[derive(Debug, PartialEq, Clone)] // Added Clone for easier testing
pub struct Element {
    pub tag_name: String,
    pub props: HashMap<String, String>,
    pub children: Vec<Node>,
}

// Our Virtual DOM Node enum.
#[derive(Debug, PartialEq, Clone)] // Added Clone for easier testing
pub enum Node {
    Element(Element),
    Text(String),
}

/// The private, recursive function that renders VDOM into real DOM nodes.
fn render_node_to_dom(v_node: &Node, document: &Document, parent: &DomNode) {
    match v_node {
        Node::Text(text) => {
            let text_node = document.create_text_node(text);
            parent
                .append_child(&text_node)
                .expect("Failed to append text node");
        }
        Node::Element(element) => {
            let dom_element = document
                .create_element(&element.tag_name)
                .expect("Failed to create element");

            for (key, value) in &element.props {
                dom_element
                    .set_attribute(key, value)
                    .expect("Failed to set attribute");
            }

            parent
                .append_child(&dom_element)
                .expect("Failed to append element");

            for child in &element.children {
                render_node_to_dom(child, document, &dom_element);
            }
        }
    }
}

/// The public API function exported to JavaScript.
#[wasm_bindgen]
pub fn render(mount_point_id: String) {
    // We create our virtual dom tree here. Let's create a helper function for it.
    let app_vdom = create_app_vdom();

    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");

    // Get the mount point as a specific DomElement.
    let mount_element: DomElement = document
        .get_element_by_id(&mount_point_id)
        .expect(&format!(
            "Mount point with id '{}' not found",
            mount_point_id
        ));

    // CHANGE #1: Call `set_inner_html` while our variable is still a `DomElement`.
    // This now works because `DomElement` has this method.
    mount_element.set_inner_html("");

    // CHANGE #2: Kick off rendering by passing a *reference* to the mount_element.
    // Rust is smart enough to know that a `&DomElement` can be used where a `&DomNode` is expected.
    render_node_to_dom(&app_vdom, &document, &mount_element);
}

/// A helper function to create our standard VDOM tree.
/// This keeps the `render` function cleaner and allows us to reuse the tree in tests.
fn create_app_vdom() -> Node {
    Node::Element(Element {
        tag_name: "div".to_string(),
        props: {
            let mut props = HashMap::new();
            props.insert("class".to_string(), "app-container".to_string());
            props.insert("data-rendered-by".to_string(), "rusty-react".to_string());
            props
        },
        children: vec![
            Node::Element(Element {
                tag_name: "h1".to_string(),
                props: HashMap::new(),
                children: vec![Node::Text("Hello from Rusty React!".to_string())],
            }),
            Node::Element(Element {
                tag_name: "p".to_string(),
                props: HashMap::new(),
                children: vec![Node::Text("This was rendered entirely from Rust.".to_string())],
            }),
        ],
    })
}

// ----------------------------------------------------------------------------------
// --- TEST SECTION: THIS IS SUPER IMPORTANT FOR UNDERSTANDING ----------------------
// ----------------------------------------------------------------------------------

/// A recursive function that renders our VDOM to an HTML String.
/// This is PURELY for testing our rendering logic without a browser.
fn render_node_to_string(v_node: &Node) -> String {
    match v_node {
        Node::Text(text) => text.clone(),
        Node::Element(element) => {
            // Build the opening tag with attributes
            let mut props_string = String::new();
            for (key, value) in &element.props {
                props_string.push_str(&format!(" {}=\"{}\"", key, value));
            }

            // Render all children recursively
            let children_string: String = element
                .children
                .iter()
                .map(render_node_to_string)
                .collect();

            // Combine into the final HTML string
            format!(
                "<{}{}>{}</{}>",
                element.tag_name, props_string, children_string, element.tag_name
            )
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*; // Import all from parent module.

    #[test]
    fn test_vdom_creation() {
        // We can use the same helper function to get a predictable VDOM tree.
        let vdom = create_app_vdom();

        // Assert that the top-level node is a div.
        if let Node::Element(e) = vdom {
            assert_eq!(e.tag_name, "div");
            assert_eq!(e.props.get("class").unwrap(), "app-container");
            assert_eq!(e.children.len(), 2);
        } else {
            panic!("Root node should be an Element");
        }
    }

    #[test]
    fn test_render_to_string_logic() {
        // Let's create a known VDOM tree.
        let vdom = create_app_vdom();

        // And now render it to a string.
        let html_string = render_node_to_string(&vdom);

        // Define what we EXPECT the output to be. Note how it matches our VDOM structure.
        let expected_html = r#"<div class="app-container" data-rendered-by="rusty-react"><h1>Hello from Rusty React!</h1><p>This was rendered entirely from Rust.</p></div>"#;

        // This is a powerful, reliable test of our rendering logic.
        // We had to manually reorder the props in `expected_html` to match HashMap's unpredictable order,
        // or use more complex comparison logic. For now, we adjust the string to match.
        // A better test might parse the attributes into a HashMap and compare them.
        let expected_html_alt = r#"<div data-rendered-by="rusty-react" class="app-container"><h1>Hello from Rusty React!</h1><p>This was rendered entirely from Rust.</p></div>"#;

        assert!(html_string == expected_html || html_string == expected_html_alt, "Rendered HTML string did not match expected output.");
    }
}
