#[derive(Debug, PartialEq)]
pub struct Element {
    pub tag_name: String,
    pub children: Vec<Element>,
}

pub fn create_element(tag_name: String, children: Vec<Element>) -> Element {
    Element {
        tag_name,
        children,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_an_element() {
        let child_element = create_element("h2".to_string(), vec![]);
        let parent_element = create_element(
            "div".to_string(),
            vec![child_element],
        );

        let expected_child = Element {
            tag_name: "h2".to_string(),
            children: vec![],
        };

        let expected_parent = Element {
            tag_name: "div".to_string(),
            children: vec![expected_child],
        };

        // We'll need to update this part as our Element struct doesn't derive PartialEq for direct comparison yet
        assert_eq!(parent_element, expected_parent);
    }
}
