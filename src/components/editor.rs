use dioxus::html::onfocus;
use dioxus::logger::tracing::{info, Level};
use dioxus::prelude::*;
use dioxus::web::WebEventExt;

const CSS: Asset = asset!("/assets/styling/editor.css");

#[component]
pub fn Editor() -> Element {
    let mut line_count = use_signal(|| 1);
    let font_size = use_signal(|| "16px");
    
    let mut current_line = use_signal(|| 0);

    let mut rows = use_signal(|| vec!["content"]);
    
    info!("Rows: {:?}", rows.read());

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        // We can create elements inside the rsx macro with the element name followed by a block of attributes and children.
        div {
            class: "row centerRow editor-parent",
            font_size: font_size,
            div {
                for index in 0..rows.read().len() {
                    {info!("Row {}: {}", index + 1, rows.read()[index]);}
                    div {
                        class: "row",
                        gap: "var(--spacingSmall)",
                        div {
                            class: "line-number text-weak",
                            "{index + 1}"
                        },
                        div {
                            class: "content-row",
                            autofocus: *current_line.read() == index,
                            contenteditable: *current_line.read() == index,
                            onkeypress: move |event| async move {
                                if event.key() == Key::Enter {
                                    rows.push("");
                                    current_line.set(index + 1);
                                }
                            },
                            onmounted: async move |event| {
                                event.data.set_focus(true).await;
                                // let element = event.target();
                                // element.focus();
                            },
                            onfocus: move |_| {
                                current_line.set(index);
                            },
                            "{rows.read()[index]}",
                        }
                    }
                }
            },
        }
    }
}
