use crate::components::{Echo, Editor, Hero, Sidebar};
use dioxus::{html::textarea::autocomplete, prelude::*};

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "row centerRow",
            height: "80vw",
            overflow: "hidden",
            gap: "var(--spacingMedium)",
            Sidebar {}
            Editor {}
            Sidebar {}
            div {
                border: "solid 2px grey",
                spellcheck: true,
                contenteditable: true,
                height: "400px",
                width: "500px"
            }
        }
    }
}
