use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

// Aliasing the `web_sys` types to avoid naming collisions with our VDOM types.
use web_sys::{Document, Element as DomElement, Node as DomNode, Window};

// Our Virtual DOM Element structure. This is our blueprint.
#[derive(Debug, Clone)]
pub struct Element {
    pub tag_name: String,
    pub props: HashMap<String, String>,
    pub children: Vec<Node>,
}

/// The "newtype" pattern: a struct that wraps `Rc<dyn Component>`.
/// Because `VComponent` is a type local to our crate, we can implement foreign
/// traits like `Clone` for it, satisfying Rust's orphan rule.
#[derive(Debug)]
pub struct VComponent(Rc<dyn Component>);

impl Clone for VComponent {
    fn clone(&self) -> Self {
        // To clone our newtype, we call the cloning method defined on our component trait.
        // `self.0` accesses the inner `Rc<dyn Component>`.
        VComponent(self.0.clone_rc())
    }
}

/// The contract for any reusable, renderable component.
/// It must be clonable itself, and provide a way to be cloned into a smart pointer.
pub trait Component: Debug {
    fn render(&self) -> Node;
    fn clone_rc(&self) -> Rc<dyn Component>;
}

// Our Virtual DOM Node enum. It can now be cloned efficiently thanks to our VComponent newtype.
#[derive(Debug, Clone)]
pub enum Node {
    Element(Element),
    Text(String),
    Component(VComponent),
}

/// Our first component. We `derive(Clone)` so we can call `self.clone()` inside `clone_rc`.
#[derive(Debug, Clone)]
pub struct App;

/// We implement the `Component` trait for `App` to tell our library how to render it.
impl Component for App {
    fn render(&self) -> Node {
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
                    children: vec![Node::Text("Hello from a Rusty Component!".to_string())],
                }),
                Node::Element(Element {
                    tag_name: "p".to_string(),
                    props: HashMap::new(),
                    children: vec![Node::Text("This was rendered via a component trait.".to_string())],
                }),
            ],
        })
    }

    /// Implements the required cloning method for the Component trait.
    fn clone_rc(&self) -> Rc<dyn Component> {
        Rc::new(self.clone())
    }
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
        Node::Component(v_component) => {
            // Access the inner component via `.0` and render it.
            let rendered_node = v_component.0.render();
            render_node_to_dom(&rendered_node, document, parent);
        }
    }
}

/// The public API function exported to JavaScript.
#[wasm_bindgen]
pub fn render(mount_point_id: String) {
    let root_component = App;
    // We wrap our component instance first in an `Rc`, then in our `VComponent` newtype.
    let app_vdom = Node::Component(VComponent(Rc::new(root_component)));

    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");

    let mount_element: DomElement = document
        .get_element_by_id(&mount_point_id)
        .expect(&format!(
            "Mount point with id '{}' not found",
            mount_point_id
        ));

    mount_element.set_inner_html("");

    render_node_to_dom(&app_vdom, &document, &mount_element);
}


// ----------------------------------------------------------------------------------
// --- TEST SECTION: THIS IS SUPER IMPORTANT FOR UNDERSTANDING ----------------------
// ----------------------------------------------------------------------------------

/// A recursive function that renders our VDOM to an HTML String.
fn render_node_to_string(v_node: &Node) -> String {
    match v_node {
        Node::Text(text) => text.clone(),
        Node::Element(element) => {
            let mut props_string = String::new();
            for (key, value) in &element.props {
                props_string.push_str(&format!(" {}=\"{}\"", key, value));
            }
            let children_string: String = element.children.iter().map(render_node_to_string).collect();
            format!(
                "<{}{}>{}</{}>",
                element.tag_name, props_string, children_string, element.tag_name
            )
        }
        Node::Component(v_component) => {
            // Access the inner component via `.0` to render it to a string.
            let rendered_node = v_component.0.render();
            render_node_to_string(&rendered_node)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_component_creation() {
        let app_component = App;
        let vdom_node = app_component.render();

        if let Node::Element(e) = vdom_node {
            assert_eq!(e.tag_name, "div");
            assert_eq!(e.props.get("class").unwrap(), "app-container");
            assert_eq!(e.children.len(), 2);
        } else {
            panic!("Root node rendered by App should be an Element");
        }
    }

    #[test]
    fn test_render_to_string_with_component() {
        // We now wrap our component in an Rc and then our VComponent newtype.
        let vdom = Node::Component(VComponent(Rc::new(App)));

        let html_string = render_node_to_string(&vdom);

        let expected_html = r#"<div class="app-container" data-rendered-by="rusty-react"><h1>Hello from a Rusty Component!</h1><p>This was rendered via a component trait.</p></div>"#;
        let expected_html_alt = r#"<div data-rendered-by="rusty-react" class="app-container"><h1>Hello from a Rusty Component!</h1><p>This was rendered via a component trait.</p></div>"#;

        assert!(html_string == expected_html || html_string == expected_html_alt, "Rendered HTML string did not match expected output.");
    }
}
