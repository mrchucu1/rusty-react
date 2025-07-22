# rusty-react

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/mrchucu1/rusty-react)
[![Crates.io](https://img.shields.io/crates/v/rusty-react.svg?label=version)](https://crates.io/crates/rusty-react)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

An educational exploration into rewriting the core principles of React in Rust.

## Project Goal

`rusty-react` is a learning exercise undertaken by a (fictional) team at Meta to deeply understand the internal architecture of a modern UI library like React, while simultaneously exploring the power, safety, and performance of the Rust programming language.

**This is not intended to be a production-ready framework.** Instead, it serves as a "from-scratch" implementation to model concepts like:

- The Virtual DOM
- The `createElement` and element structure
- The reconciliation (diffing) algorithm
- Component-based architecture
- State management

We draw heavy inspiration from the actual React codebase and other innovative Rust-based UI frameworks like Dioxus and Yew.

## Current Status

**Stage: 3 - Component Architecture**

We have successfully abstracted our rendering logic into a component-based model. Our library can now render a UI defined within a reusable component, which is the foundational pattern for building complex applications.

- [x] Defined `Element` and `Node` structs for the Virtual DOM.
- [x] Implemented a renderer that mounts a VDOM tree to a live browser DOM.
- [x] Established support for `props` (attributes) and `children`.
- [x] **(New)** Introduced a `Component` trait to define a reusable UI contract.
- [x] **(New)** Refactored the VDOM and renderer to be "component-aware," allowing them to render abstract components instead of just static data.
- [x] **(New)** Implemented a root `App` component that encapsulates the main UI view.
- [x] **(New)** The `Node` enum now uses `Box<dyn Component>` to handle different component types polymorphically, a core concept for UI library design.

The library now renders a UI defined within a component structure, setting the stage for statefulness.

---

## Developer Guide

This guide explains how to set up the development environment, build the project, and run the example application.

### Prerequisites

You must have the following tools installed on your system:
- **Rust:** The core language toolchain. Install it via [rustup.rs](https://rustup.rs/).
- **`wasm-pack`:** The tool for compiling Rust to WebAssembly. Install it with Cargo:
  ```bash
  cargo install wasm-pack
  ```
- **Node.js & npm:** For managing the TypeScript example application. Install it from [nodejs.org](https://nodejs.org/).

### Folder Structure

- `src/`: The core Rust library code for `rusty-react`.
- `examples/basic-render-test/`: A Vite + TypeScript application that *consumes* our library to test it in a browser.
- `pkg/`: **Generated folder.** This contains the compiled WebAssembly, JavaScript glue code, and TypeScript definitions. **Do not edit files in this directory.** It's created by `wasm-pack`.
- `Cargo.toml`: The Rust project manifest.

### The Development Workflow

The core development loop involves making changes to the Rust library and then viewing those changes in the example browser app.

#### **Step 1: First-Time Setup**

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/mrchucu1/rusty-react.git
    cd rusty-react
    ```
2.  **Build the Rust library into a Wasm package:**
    This command compiles the Rust code in `src/` and places the output (our "npm package") into the `pkg/` directory.
    ```bash
    wasm-pack build --target web
    ```
3.  **Install the example app's dependencies:**
    This `npm` command sets up the Vite project and creates a symlink to our local `pkg/` directory.
    ```bash
    cd examples/basic-render-test
    npm install
    ```

#### **Step 2: Running the Application**

To see your changes, follow these steps:

1.  **If you've changed Rust code** in `src/`, you must re-compile it. From the **root** of the project (`rusty-react/`):
    ```bash
    wasm-pack build --target web
    ```
2.  **Run the Vite development server.** From the **example app** directory (`examples/basic-render-test/`):
    ```bash
    npm run dev
    ```
3.  Open the URL provided by Vite in your browser to see the rendered output.

### Running Tests

We have two types of tests:

1.  **Rust Unit Tests:** These test the core VDOM logic (like `render_to_string`) without a browser. They are fast and can be run from the project root.
    ```bash
    cargo test
    ```
2.  **End-to-End Test:** Running the example app itself is our primary end-to-end test to ensure the Wasm interacts correctly with the browser DOM.

---

## Roadmap

Our high-level plan is to build out the core features in the following order:

-   **[Done] Renderer**: Implement a simple renderer that can take a `Node` tree and convert it into real DOM nodes.
-   **[In Progress] Components & State**: Introduce the concept of components as stateful functions or structs that can re-render.
-   **[Next] State Management & Re-rendering**: Give components an internal state and a mechanism to trigger a re-render when that state changes.
-   **Reconciliation (Diffing)**: Create the "diff and patch" algorithm that compares two Virtual DOM trees and generates a list of minimal changes.
-   **DOM Patcher**: Apply the generated patches to a real DOM efficiently.


## Contributing

As this is primarily an educational project, we aren't seeking major contributions at this time. However, feedback, suggestions, and questions are always welcome! Feel free to open an issue to discuss ideas.

## License

This project is licensed under the MIT License. See the [LICENSE.md](LICENSE.md) file for details.
