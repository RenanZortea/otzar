//! This crate contains all shared UI for the workspace.

mod outliner;
pub use outliner::Outliner;

mod tree;
pub use tree::Tree;

mod markdown;
pub use markdown::render_markdown;
