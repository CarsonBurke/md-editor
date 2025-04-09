use dioxus::prelude::*;
use dioxus::logger::tracing::{Level, info};

const CSS: Asset = asset!("/assets/styling/sidebar.css");

#[component]
pub fn Sidebar() -> Element {
    
    let file_names = vec![
        "file1.txt",
        "file2.txt",
        "file3.txt"
    ];
    
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }
        
        // We can create elements inside the rsx macro with the element name followed by a block of attributes and children.
        div {
            class: "column editor-parent",
            width: "fit-content",
            gap: "var(--spacingSmall)",
            for file_name in file_names {
                div {
                    class: "file-item",
                    "{file_name}"
                }
            }
        }
    }
}