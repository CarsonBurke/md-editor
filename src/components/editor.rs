use dioxus::html::onfocus;
use dioxus::logger::tracing::{info, Level};
use dioxus::prelude::*;
use dioxus::web::WebEventExt;

const CSS: Asset = asset!("/assets/styling/editor.css");

#[component]
pub fn Editor() -> Element {
    let font_size = use_signal(|| "16px");

    let mut current_line = use_signal(|| 0);

    let mut rows = use_signal(|| vec!["content".to_string()]);

    info!("Rows: {:?}", rows.read());

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        // We can create elements inside the rsx macro with the element name followed by a block of attributes and children.
        div {
            class: "row editor-parent",
            gap: "var(--spacingMedium)",
            font_size: font_size,
            div {
                for index in 0..=*current_line.read() {
                    div {
                        class: "line-number text-weak",
                        "{index + 1}"
                    },
                }
            }
            div {
                contenteditable: true,
                onkeydown: move |event| async move {
                    match event.key() {
                        Key::Enter => {
                            info!("Enter pressed");
                            // if *current_line.read() == rows.read().len() - 1 {
                            //     rows.push("<br />".to_string());
                            // }
                            
                            current_line.with_mut(|current_line| {
                               *current_line += 1;
                            });
                            
                            event.prevent_default();
                        }
                        Key::Backspace => {
                            info!("Delete pressed");
                            
                            current_line.with_mut(|current_line| {
                               *current_line -= 1;
                            });
                            
                            event.prevent_default();
                        },
                        _ => {
                            info!("Key pressed editor: {}", event.key());
                        }
                    }
                },
                onfocus: move |event| async move {
                    info!("Focus editor");
                },
                onchange: move |event| async move {
                    info!("Change editor");
                },
                oninput: move |event| async move {
                  info!("inputted editor");
                },
                for (index, row) in rows.read().iter().enumerate() {
                    {info!("Row {}: {}", index + 1, row);}
                    div {
                        class: "content-row",
                        id: format!("custom-{}", index),
                        // autofocus: *current_line.read() == index,
                        // tabindex: *current_line.read() == index,
                        // contenteditable: *current_line.read() == index,
                        // onkeypress: move |event| async move {
                        //     match event.key() {
                        //         Key::Enter => {
                        //             info!("Enter pressed");
                        //             if *current_line.read() == rows.read().len() - 1 {
                        //                 rows.push("<br />".to_string());
                        //             }

                        //             current_line.set(index + 1);
                        //         }
                        //         Key::ArrowUp => {
                        //             info!("ArrowUp pressed");
                        //             if index > 0 {
                        //                 current_line.set(index - 1);
                        //             }
                        //         }
                        //         Key::ArrowDown => {
                        //             info!("ArrowDown pressed");
                        //             if index < rows.len() {
                        //                 current_line.set(index + 1);
                        //             }
                        //         }
                        //         _ => {
                        //             info!("Key pressed");
                        //         }
                        //     }
                        // },
                        // onload: move |event| {
                        //     let target = event.data().target();

                        //     let element =

                        // },
                        // oninput: move |event| {
                        //     rows.with_mut(|rows| {
                        //         let text = event.data().value();
                        //         rows[index] = text;
                        //     });
                        // },
                        onmounted: async move |event| {
                            info!("element mounted");
                            // event.data.set_focus(true).await;
                        },
                        onfocus: move |_| {
                            info!("in focus");
                            current_line.set(index);
                        },
                        "{rows.read()[index]}",
                    }
                }
            },
        }
    }
}
