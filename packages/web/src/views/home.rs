use dioxus::prelude::*;
use ui::Outliner;

#[component]
pub fn Home() -> Element {
    rsx! {
        Outliner{}
    }
}
