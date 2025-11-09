use dioxus::prelude::*;

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        h1 { "Counter for {count}" }
        button { onclick: move |_| count += 1, "Up" }
        button { onclick: move |_| count -= 1, "Down" }
    }
}

fn main() {
dioxus::launch(App);
}
