use dioxus::prelude::*;
use crate::markdown::render_markdown;
use crate::tree::{NodeId, Tree};


#[component]
pub fn Outliner() -> Element {
    let mut tree = use_signal(|| {
        let mut t = Tree::new();

        // Sample data
        let root1 = t.add_node("# Welcome to the Outliner".to_string(), None);
        let child1 = t.add_node("This is a **LogSeq-like** outliner interface".to_string(), Some(root1));
        t.add_node("Built with *Dioxus* and Rust 🦀".to_string(), Some(child1));

        let root2 = t.add_node("## Math Support".to_string(), None);
        t.add_node("Inline math: $E = mc^2$".to_string(), Some(root2));
        t.add_node("Display math: $$\\int_0^\\infty e^{-x^2} dx = \\frac{\\sqrt{\\pi}}{2}$$".to_string(), Some(root2));

        let root3 = t.add_node("## Keyboard Shortcuts".to_string(), None);
        t.add_node("**Enter**: Create new block".to_string(), Some(root3));
        t.add_node("**Tab**: Indent block".to_string(), Some(root3));
        t.add_node("**Shift+Tab**: Outdent block".to_string(), Some(root3));

        t
    });

    let mut sidebar_open = use_signal(|| true);

    rsx! {

        div {
            id: "outliner-layout",

            // Sidebar
            if sidebar_open() {
                div {
                    id: "sidebar",
                    div {
                        class: "bg-white color-black",
                        h2 { "Pages" }
                        button {
                            class: "",
                            onclick: move |_| sidebar_open.set(false),
                            "×"
                        }
                    }

                    div {
                        class: "bg-white",
                        div { class: "sidebar-item active", "Main Page" }
                        div { class: "sidebar-item", "Journal" }
                        div { class: "sidebar-item", "Bookmarks" }
                        div { class: "sidebar-item", "Archive" }
                    }
                }
            }

            // Main content
            div {
                id: "main-content",

                div {
                    class: "toolbar",
                    if !sidebar_open() {
                        button {
                            class: "toggle-sidebar",
                            onclick: move |_| sidebar_open.set(true),
                            "☰"
                        }
                    }

                    h1 { "📝 Outliner" }
                }

                div {
                    id: "outliner-container",

                    div {
                        class: "outliner-tree",
                        for node_id in tree.read().get_root_nodes() {
                            OutlinerNode {
                                node_id: *node_id,
                                tree: tree,
                                depth: 0
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn OutlinerNode(node_id: NodeId, tree: Signal<Tree>, depth: usize) -> Element {
    let node = tree.read().get_node(node_id).cloned();
    let mut is_editing = use_signal(|| false);
    let mut edit_value = use_signal(|| String::new());

    let Some(node) = node else {
        return rsx! { div { "Node not found" } };
    };

    let has_children = !node.children.is_empty();
    let is_expanded = node.is_expanded;
    let rendered_html = render_markdown(&node.content);

    let handle_keydown = move |evt: Event<KeyboardData>| {
        // New API: pull KeyboardData and modifiers from the event
        let data = evt.data();
        let key = data.key();
        let shift = data.modifiers().shift();

        if key == Key::Enter {
            evt.prevent_default();

            if is_editing() {
                tree.write().update_content(node_id, edit_value());
                tree.write().add_sibling(node_id, String::new());
                is_editing.set(false);
            } else {
                tree.write().add_sibling(node_id, String::new());
            }
        } else if key == Key::Tab {
            evt.prevent_default();

            if shift {
                tree.write().outdent_node(node_id);
            } else {
                tree.write().indent_node(node_id);
            }
        } else if key == Key::Escape && is_editing() {
            is_editing.set(false);
        }
    };

    rsx! {
        div {
            class: "outliner-node",
            style: "margin-left: {depth * 24}px",

            div {
                class: "node-content-wrapper",

                // Expand/collapse button
                button {
                    class: if has_children { "toggle-button" } else { "toggle-button invisible" },
                    onclick: move |_| {
                        tree.write().toggle_expanded(node_id);
                    },
                    if has_children {
                        if is_expanded { "▼" } else { "▶" }
                    }
                }

                // Bullet point
                span { class: "bullet", "•" }

                // Node content
                div {
                    class: "node-content",
                    tabindex: "0",
                    onkeydown: handle_keydown,
                    onclick: move |_| {
                        if !is_editing() {
                            edit_value.set(node.content.clone());
                            is_editing.set(true);
                        }
                    },

                    if is_editing() {
                        textarea {
                            class: "node-input",
                            value: "{edit_value}",
                            autofocus: true,
                            rows: "1",
                            oninput: move |evt| {
                                edit_value.set(evt.value());
                            },
                            onkeydown: handle_keydown,
                            onblur: move |_| {
                                tree.write().update_content(node_id, edit_value());
                                is_editing.set(false);
                            }
                        }
                    } else {
                        div {
                            class: "rendered-content",
                            dangerous_inner_html: "{rendered_html}"
                        }
                    }
                }
            }

            // Render children if expanded
            if is_expanded && has_children {
                div {
                    class: "node-children",
                    for child_id in node.children.iter() {
                        OutlinerNode {
                            node_id: *child_id,
                            tree: tree,
                            depth: depth + 1
                        }
                    }
                }
            }
        }
    }
}

