# rusty-react

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/mrchucu1/rusty-react)
[![Crates.io](https://img.shields.io/crates/v/rusty-react.svg?label=version)](https://crates.io/crates/rusty-react)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

An educational exploration into rewriting the core principles of React in Rust.

## Project Goal

`rusty-react` is a learning exercise undertaken by a (fictional) team at Meta to deeply understand the internal architecture of a modern UI library like React, while simultaneously exploring the power, safety, and performance of the Rust programming language.

**This is not intended to be a production-ready framework.** Instead, it serves as a "from-scratch" implementation to model concepts like:

*   The Virtual DOM
*   The `createElement` and element structure
*   The reconciliation (diffing) algorithm
*   Component-based architecture
*   State management

We draw heavy inspiration from the actual React codebase and other innovative Rust-based UI frameworks like Dioxus and Yew.

## Current Status

**Stage: 1 - Foundational Structures**

The project is in its earliest phase. We have successfully implemented the foundational data structures required to represent a UI in memory.

-   [x] Defined `Element` and `Node` (for text) structs.
-   [x] Implemented a `create_element` function, analogous to `React.createElement`.
-   [x] Established support for `props` (attributes) and `children`.
-   [x] The core "Virtual DOM" tree can be built in memory.

The library does not yet render anything to the screen.

## Core Concepts

The current implementation is built around two fundamental ideas:

1.  **`Node`**: The basic unit of the UI tree. A `Node` can be one of two things:
    *   `Node::Element`: Represents an HTML-like element with a tag name, props, and children (e.g., a `<div>` or `<h1>`).
    *   `Node::Text`: Represents a simple string of text.

2.  **Virtual DOM**: By composing `Node`s together, we can build a complete, in-memory representation of our target UI. This tree is the "Virtual DOM". The ultimate goal is to compare an old version of this tree with a new one to efficiently update the user interface.

## Usage

To use `rusty-react` at its current stage, you would add it as a dependency and use the `create_element` and `create_text_node` functions to build a UI tree.

Here's an example of creating a simple application structure in memory:

```rust
use rusty_react::{create_element, create_text_node, Node};
use std::collections::HashMap;

fn main() {
    // Represents: <h1>Welcome</h1>
    let heading = create_element(
        "h1".to_string(),
        HashMap::new(),
        vec![create_text_node("Welcome".to_string())],
    );

    // Represents: <p>This is our rusty-react app.</p>
    let paragraph = create_element(
        "p".to_string(),
        HashMap::new(),
        vec![create_text_node("This is our rusty-react app.".to_string())],
    );

    // Represents: <div id="app" class="container">...</div>
    let app = create_element(
        "div".to_string(),
        {
            let mut props = HashMap::new();
            props.insert("id".to_string(), "app".to_string());
            props.insert("class".to_string(), "container".to_string());
            props
        },
        vec![heading, paragraph],
    );

    // The 'app' variable now holds our complete Virtual DOM tree.
    // We can print it for inspection:
    println!("{:#?}", app);
}
```

## Roadmap

Our high-level plan is to build out the core features in the following order:

*   **[Next] Renderer**: Implement a simple renderer that can take a `Node` tree and convert it into an HTML string.
*   **Reconciliation (Diffing)**: Create the "diff and patch" algorithm that compares two Virtual DOM trees and generates a list of minimal changes.
*   **DOM Patcher**: Apply the generated patches to a real DOM (initially, maybe just printing the changes like "Set attribute 'id' on DIV").
*   **Components & State**: Introduce the concept of components as stateful functions or structs that can re-render.

## Contributing

As this is primarily an educational project, we aren't seeking major contributions at this time. However, feedback, suggestions, and questions are always welcome! Feel free to open an issue to discuss ideas.

## License

This project is licensed under the MIT License. See the [LICENSE.md](LICENSE.md) file for details.
